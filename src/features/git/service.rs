use crate::features::git::events::{ChangeAction, DomainChangeEvent, DomainEventMapper};
use crate::features::model::storage::ProjectStorage;
use crate::features::model::ModelProject;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Command;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum GitError {
    #[error("Git er ikke installeret eller tilgængeligt på systemet: {0}")]
    GitNotInstalled(String),
    #[error("Git kommando fejlede ({cmd}): {message}")]
    CommandFailed { cmd: String, message: String },
    #[error("I/O fejl: {0}")]
    Io(#[from] std::io::Error),
    #[error("Lagringsfejl: {0}")]
    Storage(#[from] crate::features::model::storage::StorageError),
    #[error("Serialiseringsfejl: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Repository ikke fundet eller ugyldigt: {0}")]
    InvalidRepository(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RepoSyncStatus {
    Uninitialized,
    Synced,
    PendingChanges,
    UnpublishedCommits(usize),
    IncomingCommits(usize),
    Diverged { local: usize, remote: usize },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitCommitInfo {
    pub oid: String,
    pub short_oid: String,
    pub author_name: String,
    pub author_email: String,
    pub timestamp: String,
    pub message: String,
    pub changes: Vec<DomainChangeEvent>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PullResult {
    UpToDate,
    FastForwarded,
    Merged {
        conflicts: Vec<crate::features::model::merge::ModelConflict>,
    },
}

pub struct GitService;

impl GitService {
    /// Tjekker om git eksekverbare fil er tilgængelig i PATH.
    pub fn is_git_installed() -> bool {
        Command::new("git")
            .arg("--version")
            .output()
            .map(|out| out.status.success())
            .unwrap_or(false)
    }

    fn check_git_installed() -> Result<(), GitError> {
        if !Self::is_git_installed() {
            Err(GitError::GitNotInstalled(
                "git kommandoen blev ikke fundet i systemets PATH".to_string(),
            ))
        } else {
            Ok(())
        }
    }

    /// Sikrer at en tom sti håndteres som aktuel arbejdsmappe ('.').
    pub fn effective_repo_dir(repo_path: &Path) -> &Path {
        if repo_path.as_os_str().is_empty() {
            Path::new(".")
        } else {
            repo_path
        }
    }

    pub fn run_git_with_env(
        repo_dir: &Path,
        args: &[&str],
        envs: &[(&str, &str)],
    ) -> Result<String, GitError> {
        Self::check_git_installed()?;
        let effective_dir = Self::effective_repo_dir(repo_dir);
        let mut cmd = Command::new("git");
        cmd.current_dir(effective_dir);
        for &(k, v) in envs {
            cmd.env(k, v);
        }
        cmd.args(args);
        let output = cmd.output().map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                GitError::GitNotInstalled(e.to_string())
            } else {
                GitError::Io(e)
            }
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            let msg = if stderr.is_empty() {
                String::from_utf8_lossy(&output.stdout).trim().to_string()
            } else {
                stderr
            };
            return Err(GitError::CommandFailed {
                cmd: format!("git {}", args.join(" ")),
                message: msg,
            });
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn run_git(repo_dir: &Path, args: &[&str]) -> Result<String, GitError> {
        Self::run_git_with_env(repo_dir, args, &[])
    }

    /// Initialiserer et Git repository i den angivne mappe.
    pub fn init_repository(repo_path: &Path) -> Result<(), GitError> {
        Self::check_git_installed()?;
        // Prøv med standard initial branch main, fallback til init
        if Self::run_git(repo_path, &["init", "-b", "main"]).is_err() {
            Self::run_git(repo_path, &["init"])?;
        }
        Ok(())
    }

    /// Henter synkroniseringsstatus for det lokale repository i forhold til remote.
    pub fn get_sync_status(repo_path: &Path) -> Result<RepoSyncStatus, GitError> {
        Self::check_git_installed()?;
        let effective_path = Self::effective_repo_dir(repo_path);

        if Self::run_git(effective_path, &["rev-parse", "--is-inside-work-tree"]).is_err() {
            return Ok(RepoSyncStatus::Uninitialized);
        }

        let has_head = Self::run_git(effective_path, &["rev-parse", "--verify", "HEAD"]).is_ok();
        if !has_head {
            let status_out =
                Self::run_git(effective_path, &["status", "--porcelain", "-uall", ".kant"])?;
            if status_out.trim().is_empty() {
                return Ok(RepoSyncStatus::UnpublishedCommits(0));
            } else {
                return Ok(RepoSyncStatus::PendingChanges);
            }
        }

        let status_out =
            Self::run_git(effective_path, &["status", "--porcelain", "-uall", ".kant"])?;
        if !status_out.trim().is_empty() {
            return Ok(RepoSyncStatus::PendingChanges);
        }

        let upstream = Self::run_git(effective_path, &["rev-parse", "--abbrev-ref", "@{u}"]);
        match upstream {
            Ok(_) => {
                let rev_list = Self::run_git(
                    effective_path,
                    &[
                        "rev-list",
                        "--left-right",
                        "--count",
                        "HEAD...@{u}",
                        "--",
                        ".kant",
                    ],
                )?;
                let counts: Vec<&str> = rev_list.split_whitespace().collect();
                let local = counts
                    .first()
                    .and_then(|c| c.parse::<usize>().ok())
                    .unwrap_or(0);
                let remote = counts
                    .get(1)
                    .and_then(|c| c.parse::<usize>().ok())
                    .unwrap_or(0);

                if local == 0 && remote == 0 {
                    Ok(RepoSyncStatus::Synced)
                } else if local > 0 && remote == 0 {
                    Ok(RepoSyncStatus::UnpublishedCommits(local))
                } else if local == 0 && remote > 0 {
                    Ok(RepoSyncStatus::IncomingCommits(remote))
                } else {
                    Ok(RepoSyncStatus::Diverged { local, remote })
                }
            }
            Err(_) => {
                let count_out = Self::run_git(
                    effective_path,
                    &["rev-list", "--count", "HEAD", "--", ".kant"],
                )?;
                let count = count_out.trim().parse::<usize>().unwrap_or(0);
                Ok(RepoSyncStatus::UnpublishedCommits(count))
            }
        }
    }

    /// Gemmer projektet i dekomponeret format, stager ændringer og foretager en Git commit.
    pub fn publish_model(
        repo_path: &Path,
        project: &ModelProject,
        message: &str,
    ) -> Result<String, GitError> {
        Self::check_git_installed()?;

        // 1. Gem projekt dekomponeret i .kant/
        ProjectStorage::save_to_directory(project, repo_path)?;

        // 2. Stage .kant/
        Self::run_git(repo_path, &["add", ".kant"])?;

        // 3. Tjek om der er staged ændringer at committe
        let is_clean = Self::run_git(repo_path, &["diff", "--cached", "--quiet"]).is_ok();
        if is_clean {
            let oid = Self::run_git(repo_path, &["rev-parse", "HEAD"]).unwrap_or_default();
            return Ok(oid.trim().to_string());
        }

        // 4. Afgør commit-besked
        let commit_message = if message.trim().is_empty() {
            let staged_diff = Self::run_git(repo_path, &["diff", "--cached", "--name-status"])?;
            let changes = Self::parse_staged_diff(repo_path, &staged_diff)?;
            let summary = DomainEventMapper::generate_commit_summary(&changes);
            if summary.trim().is_empty() {
                "Opdatering af model".to_string()
            } else {
                summary
            }
        } else {
            message.trim().to_string()
        };

        // 4. Foretag commit med fallback til FDA default identitet
        let has_configured_author = Self::run_git(repo_path, &["config", "user.name"])
            .map(|s| !s.trim().is_empty())
            .unwrap_or(false);

        if has_configured_author {
            let commit_res = Self::run_git(repo_path, &["commit", "-m", &commit_message]);
            if let Err(e) = commit_res {
                let err_msg = e.to_string();
                if err_msg.contains("Author identity unknown")
                    || err_msg.contains("Please tell me who you are")
                {
                    Self::commit_with_fallback(repo_path, &commit_message)?;
                } else {
                    return Err(e);
                }
            }
        } else {
            Self::commit_with_fallback(repo_path, &commit_message)?;
        }

        let oid = Self::run_git(repo_path, &["rev-parse", "HEAD"])?;
        Ok(oid.trim().to_string())
    }

    fn commit_with_fallback(repo_path: &Path, message: &str) -> Result<(), GitError> {
        Self::run_git_with_env(
            repo_path,
            &["commit", "-m", message],
            &[
                ("GIT_AUTHOR_NAME", "Kant Modellør"),
                ("GIT_AUTHOR_EMAIL", "modeller@kant.local"),
                ("GIT_COMMITTER_NAME", "Kant Modellør"),
                ("GIT_COMMITTER_EMAIL", "modeller@kant.local"),
            ],
        )?;
        Ok(())
    }

    /// Henter op til `max_count` commits i modelhistorikken med tilhørende domænehændelser.
    pub fn get_commit_history(
        repo_path: &Path,
        max_count: usize,
    ) -> Result<Vec<GitCommitInfo>, GitError> {
        Self::check_git_installed()?;
        if Self::run_git(repo_path, &["rev-parse", "--verify", "HEAD"]).is_err() {
            return Ok(Vec::new());
        }

        let max_arg = format!("-n{}", max_count);
        let log_out = Self::run_git(
            repo_path,
            &[
                "log",
                &max_arg,
                "--format=format:%H%x1f%h%x1f%an%x1f%ae%x1f%aI%x1f%s%x1e",
            ],
        )?;

        Self::parse_commit_log_output(repo_path, &log_out)
    }

    /// Henter revisionshistorik for et specifikt element identificeret ved UUID.
    pub fn get_element_history(
        repo_path: &Path,
        entity_uuid: Uuid,
    ) -> Result<Vec<GitCommitInfo>, GitError> {
        Self::check_git_installed()?;
        if Self::run_git(repo_path, &["rev-parse", "--verify", "HEAD"]).is_err() {
            return Ok(Vec::new());
        }

        let pattern = format!("*{}*.json", entity_uuid);
        let log_out = Self::run_git(
            repo_path,
            &[
                "log",
                "--format=format:%H%x1f%h%x1f%an%x1f%ae%x1f%aI%x1f%s%x1e",
                "--",
                &pattern,
            ],
        )?;

        Self::parse_commit_log_output(repo_path, &log_out)
    }

    /// Parser output fra `git log` formateret med unit/record separatorer.
    fn parse_commit_log_output(
        repo_path: &Path,
        log_out: &str,
    ) -> Result<Vec<GitCommitInfo>, GitError> {
        let mut commits = Vec::new();
        for record in log_out.split('\x1e') {
            let record = record.trim();
            if record.is_empty() {
                continue;
            }
            let fields: Vec<&str> = record.split('\x1f').collect();
            if fields.len() < 6 {
                continue;
            }

            let oid = fields[0].trim().to_string();
            let short_oid = fields[1].trim().to_string();
            let author_name = fields[2].trim().to_string();
            let author_email = fields[3].trim().to_string();
            let timestamp = fields[4].trim().to_string();
            let message = fields[5].trim().to_string();

            // Hent diff for denne commit
            let diff_tree = Self::run_git(
                repo_path,
                &[
                    "diff-tree",
                    "--no-commit-id",
                    "--name-status",
                    "-r",
                    "--root",
                    &oid,
                ],
            )?;

            let changes = Self::parse_commit_diff(repo_path, &oid, &diff_tree)?;

            if !changes.is_empty() {
                commits.push(GitCommitInfo {
                    oid,
                    short_oid,
                    author_name,
                    author_email,
                    timestamp,
                    message,
                    changes,
                });
            }
        }

        Ok(commits)
    }

    fn parse_commit_diff(
        repo_path: &Path,
        commit_oid: &str,
        diff_tree: &str,
    ) -> Result<Vec<DomainChangeEvent>, GitError> {
        let mut changes = Vec::new();
        for line in diff_tree.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let mut parts = line.split('\t');
            let status_str = match parts.next() {
                Some(s) => s.trim(),
                None => continue,
            };
            let p1 = match parts.next() {
                Some(p) => p.trim(),
                None => continue,
            };
            let file_path = if let Some(p2) = parts.next() {
                p2.trim()
            } else {
                p1
            };

            let status_char = match status_str.chars().next() {
                Some(c) => c,
                None => continue,
            };

            let action = match status_char {
                'A' | 'C' => ChangeAction::Added,
                'D' => ChangeAction::Deleted,
                _ => ChangeAction::Modified,
            };

            let content = match action {
                ChangeAction::Deleted => {
                    let parent_spec = format!("{}~1:{}", commit_oid, file_path);
                    Self::run_git(repo_path, &["show", &parent_spec]).ok()
                }
                _ => {
                    let spec = format!("{}:{}", commit_oid, file_path);
                    Self::run_git(repo_path, &["show", &spec]).ok()
                }
            };

            if let Some(event) =
                DomainEventMapper::map_file_change(action, file_path, content.as_deref())
            {
                changes.push(event);
            }
        }
        Ok(changes)
    }

    fn parse_staged_diff(
        repo_path: &Path,
        staged_diff: &str,
    ) -> Result<Vec<DomainChangeEvent>, GitError> {
        let mut changes = Vec::new();
        for line in staged_diff.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let mut parts = line.split('\t');
            let status_str = match parts.next() {
                Some(s) => s.trim(),
                None => continue,
            };
            let p1 = match parts.next() {
                Some(p) => p.trim(),
                None => continue,
            };
            let file_path = if let Some(p2) = parts.next() {
                p2.trim()
            } else {
                p1
            };

            let status_char = match status_str.chars().next() {
                Some(c) => c,
                None => continue,
            };

            let action = match status_char {
                'A' | 'C' => ChangeAction::Added,
                'D' => ChangeAction::Deleted,
                _ => ChangeAction::Modified,
            };

            let content = match action {
                ChangeAction::Deleted => {
                    let spec = format!("HEAD:{}", file_path);
                    Self::run_git(repo_path, &["show", &spec]).ok()
                }
                _ => std::fs::read_to_string(repo_path.join(file_path)).ok(),
            };

            if let Some(event) =
                DomainEventMapper::map_file_change(action, file_path, content.as_deref())
            {
                changes.push(event);
            }
        }
        Ok(changes)
    }

    /// Henter ændringer fra serveren og udfører semantisk 3-vejs merge ved divergens.
    pub fn pull_model(
        repo_path: &Path,
        remote: &str,
        branch: &str,
    ) -> Result<PullResult, GitError> {
        Self::check_git_installed()?;
        Self::run_git(repo_path, &["fetch", remote, branch])?;

        let head = Self::run_git(repo_path, &["rev-parse", "HEAD"])?
            .trim()
            .to_string();
        let fetch_head = Self::run_git(repo_path, &["rev-parse", "FETCH_HEAD"])?
            .trim()
            .to_string();

        if head == fetch_head {
            return Ok(PullResult::UpToDate);
        }

        let base = Self::run_git(repo_path, &["merge-base", "HEAD", "FETCH_HEAD"])?
            .trim()
            .to_string();

        if base == fetch_head {
            return Ok(PullResult::UpToDate);
        }

        if base == head {
            Self::run_git(repo_path, &["merge", "--ff-only", "FETCH_HEAD"])?;
            return Ok(PullResult::FastForwarded);
        }

        // Divergeret: Kør semantisk 3-vejs merge!
        let base_tmp = std::env::temp_dir().join(format!("kant_merge_base_{}", Uuid::new_v4()));
        let remote_tmp = std::env::temp_dir().join(format!("kant_merge_remote_{}", Uuid::new_v4()));
        std::fs::create_dir_all(&base_tmp)?;
        std::fs::create_dir_all(&remote_tmp)?;

        let base_archive = format!(
            "git archive {} .kant | tar -x -C {}",
            base,
            base_tmp.display()
        );
        let _ = Command::new("sh")
            .arg("-c")
            .arg(&base_archive)
            .current_dir(repo_path)
            .status();

        let remote_archive = format!(
            "git archive {} .kant | tar -x -C {}",
            fetch_head,
            remote_tmp.display()
        );
        let _ = Command::new("sh")
            .arg("-c")
            .arg(&remote_archive)
            .current_dir(repo_path)
            .status();

        let base_model = ProjectStorage::load(&base_tmp)?;
        let remote_model = ProjectStorage::load(&remote_tmp)?;
        let local_model = ProjectStorage::load(repo_path)?;

        let _ = std::fs::remove_dir_all(&base_tmp);
        let _ = std::fs::remove_dir_all(&remote_tmp);

        let merge_result =
            crate::features::model::merge::merge_models(&base_model, &local_model, &remote_model);
        ProjectStorage::save_to_directory(&merge_result.merged_project, repo_path)?;

        Self::run_git(repo_path, &["add", ".kant"])?;
        let _ = Self::run_git_with_env(
            repo_path,
            &[
                "merge",
                "-s",
                "ours",
                "FETCH_HEAD",
                "-m",
                "Flettet model fra remote (semantisk 3-vejs fusion)",
            ],
            &[
                ("GIT_AUTHOR_NAME", "Kant Modellør"),
                ("GIT_AUTHOR_EMAIL", "modeller@kant.local"),
                ("GIT_COMMITTER_NAME", "Kant Modellør"),
                ("GIT_COMMITTER_EMAIL", "modeller@kant.local"),
            ],
        );

        Ok(PullResult::Merged {
            conflicts: merge_result.conflicts,
        })
    }

    /// Skubber modellens lokale commits til remote repository.
    pub fn push_model(repo_path: &Path, remote: &str) -> Result<(), GitError> {
        Self::check_git_installed()?;
        let current_branch = Self::run_git(repo_path, &["rev-parse", "--abbrev-ref", "HEAD"])
            .unwrap_or_else(|_| "main".to_string())
            .trim()
            .to_string();
        let branch = if current_branch.is_empty() || current_branch == "HEAD" {
            "main"
        } else {
            &current_branch
        };

        let res = Self::run_git_with_env(
            repo_path,
            &["push", "-u", remote, branch],
            &[("GIT_TERMINAL_PROMPT", "0")],
        );

        match res {
            Ok(_) => Ok(()),
            Err(e) => {
                let err_str = e.to_string();
                if err_str.contains("could not read Username")
                    || err_str.contains("Authentication failed")
                    || err_str.contains("Invalid username or password")
                    || err_str.contains("Permission denied")
                {
                    Err(GitError::CommandFailed {
                        cmd: "push".to_string(),
                        message: "Autentifikation påkrævet eller afvist. Indtast dit Personal Access Token i Git-forbindelse for at få skriveadgang.".to_string(),
                    })
                } else {
                    Err(e)
                }
            }
        }
    }

    /// Udtrækker eventuelt indlejret token fra en HTTPS URL og returnerer den rensede URL og tokenet.
    pub fn parse_remote_url(raw_url: &str) -> (String, Option<String>) {
        if let Some(stripped) = raw_url.strip_prefix("https://") {
            if let Some(at_idx) = stripped.find('@') {
                let auth_part = &stripped[..at_idx];
                let host_and_path = &stripped[at_idx + 1..];
                let token = if let Some(colon_idx) = auth_part.find(':') {
                    &auth_part[colon_idx + 1..]
                } else {
                    auth_part
                };
                let clean_url = format!("https://{}", host_and_path);
                return (clean_url, Some(token.to_string()));
            }
        }
        (raw_url.to_string(), None)
    }

    /// Udleder repository-navnet fra en Git URL (fx 'kant_begrebs_og_informationsmodeller' fra 'https://github.com/org/kant_begrebs_og_informationsmodeller.git').
    pub fn parse_repo_name(url: &str) -> Option<String> {
        let trimmed = url.trim().trim_end_matches('/');
        if trimmed.is_empty() {
            return None;
        }
        let without_git = trimmed.strip_suffix(".git").unwrap_or(trimmed);
        let segment = without_git.rsplit(['/', ':']).next()?;
        let clean_name = segment.trim();
        if clean_name.is_empty() {
            None
        } else {
            Some(clean_name.to_string())
        }
    }

    /// Validerer om en mappe er en sikker, isoleret modelmappe og IKKE et software- eller systemkatalog.
    pub fn is_safe_model_repo_dir(dir: &Path) -> bool {
        if dir.as_os_str().is_empty() {
            return false;
        }
        let canonical = dir.canonicalize().unwrap_or_else(|_| dir.to_path_buf());
        if canonical == Path::new("/") {
            return false;
        }
        if let Ok(home) = std::env::var("HOME") {
            if !home.is_empty() && (dir == Path::new(&home) || canonical == Path::new(&home)) {
                return false;
            }
        }
        if let Ok(profile) = std::env::var("USERPROFILE") {
            if !profile.is_empty() && (dir == Path::new(&profile) || canonical == Path::new(&profile)) {
                return false;
            }
        }

        // Tjek for typiske software-udviklingskataloger, som aldrig må bruges som model-repo
        let software_manifests = [
            "Cargo.toml",
            "package.json",
            "pom.xml",
            "go.mod",
            "build.gradle",
            "requirements.txt",
        ];
        for manifest in &software_manifests {
            if dir.join(manifest).exists() {
                return false;
            }
        }

        true
    }

    /// Sammensætter en URL med token hvis specificeret (agnostisk overfor GitLab, Gitea, GitHub, etc.).
    pub fn build_authenticated_url(url: &str, token: &str) -> String {
        let (clean_url, _) = Self::parse_remote_url(url);
        let trimmed_token = token.trim();
        if trimmed_token.is_empty() {
            return clean_url;
        }
        if let Some(stripped) = clean_url.strip_prefix("https://") {
            if trimmed_token.contains(':') {
                format!("https://{}@{}", trimmed_token, stripped)
            } else if stripped.contains("github.com") {
                format!("https://x-access-token:{}@{}", trimmed_token, stripped)
            } else if stripped.contains("gitlab.com") || trimmed_token.starts_with("glpat-") {
                format!("https://oauth2:{}@{}", trimmed_token, stripped)
            } else {
                format!("https://{}@{}", trimmed_token, stripped)
            }
        } else {
            clean_url
        }
    }

    /// Henter remote URL for origin, hvis konfigureret.
    pub fn get_remote_url(repo_path: &Path) -> Result<Option<String>, GitError> {
        let output = Self::run_git(repo_path, &["remote", "get-url", "origin"]);
        match output {
            Ok(url) => {
                let trimmed = url.trim();
                if trimmed.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(trimmed.to_string()))
                }
            }
            Err(_) => Ok(None),
        }
    }

    /// Konfigurerer remote URL for origin (tilføjer eller opdaterer).
    pub fn set_remote_url(repo_path: &Path, url: &str) -> Result<(), GitError> {
        let current = Self::get_remote_url(repo_path)?;
        if current.is_some() {
            Self::run_git(repo_path, &["remote", "set-url", "origin", url])?;
        } else {
            Self::run_git(repo_path, &["remote", "add", "origin", url])?;
        }
        Ok(())
    }

    /// Fjerner origin remote hvis den findes.
    pub fn remove_remote(repo_path: &Path) -> Result<(), GitError> {
        let current = Self::get_remote_url(repo_path)?;
        if current.is_some() {
            let _ = Self::run_git(repo_path, &["remote", "remove", "origin"]);
        }
        Ok(())
    }

    /// Henter lokalt eller globalt konfigureret forfatternavn og e-mail.
    pub fn get_user_identity(
        repo_path: &Path,
    ) -> Result<(Option<String>, Option<String>), GitError> {
        let name_res = Self::run_git(repo_path, &["config", "user.name"]);
        let name = match name_res {
            Ok(s) => {
                let t = s.trim();
                if t.is_empty() {
                    None
                } else {
                    Some(t.to_string())
                }
            }
            Err(_) => None,
        };

        let email_res = Self::run_git(repo_path, &["config", "user.email"]);
        let email = match email_res {
            Ok(s) => {
                let t = s.trim();
                if t.is_empty() {
                    None
                } else {
                    Some(t.to_string())
                }
            }
            Err(_) => None,
        };

        Ok((name, email))
    }

    /// Sætter lokalt forfatternavn og e-mail i repositoriet.
    pub fn set_user_identity(repo_path: &Path, name: &str, email: &str) -> Result<(), GitError> {
        let name_trimmed = name.trim();
        if !name_trimmed.is_empty() {
            Self::run_git(repo_path, &["config", "user.name", name_trimmed])?;
        }
        let email_trimmed = email.trim();
        if !email_trimmed.is_empty() {
            Self::run_git(repo_path, &["config", "user.email", email_trimmed])?;
        }
        Ok(())
    }

    /// Kloner et eksternt Git repository til en lokal destination.
    pub fn clone_repository(remote_url: &str, destination_dir: &Path) -> Result<(), GitError> {
        if destination_dir.exists()
            && destination_dir
                .read_dir()
                .is_ok_and(|mut i| i.next().is_some())
        {
            return Err(GitError::Io(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "Destinationsmappen eksisterer allerede og er ikke tom",
            )));
        }
        let dest_str = destination_dir
            .to_str()
            .ok_or_else(|| GitError::CommandFailed {
                cmd: "clone".to_string(),
                message: "Ugyldig destinationssti".to_string(),
            })?;

        let parent = destination_dir.parent().unwrap_or_else(|| Path::new("."));
        let output = Command::new("git")
            .args(["clone", remote_url, dest_str])
            .current_dir(parent)
            .output()?;

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            return Err(GitError::CommandFailed {
                cmd: "clone".to_string(),
                message: format!("Kloning fejlede: {}", err),
            });
        }

        Ok(())
    }
}
