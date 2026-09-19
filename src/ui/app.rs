use crate::features::model::ModelProject;
use crate::ui::theme::ThemeColors;
use iced::widget::{button, column, container, row, text, Space};
use iced::{Alignment, Element, Length};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tab {
    Metadata,
    ConceptList,
    ConceptModel,
    InformationModel,
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectTab(Tab),
    NewProject,
}

pub struct App {
    project: ModelProject,
    active_tab: Tab,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            project: ModelProject::default(),
            active_tab: Tab::Metadata,
        }
    }

    pub fn active_tab(&self) -> Tab {
        self.active_tab
    }

    pub fn project(&self) -> &ModelProject {
        &self.project
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::SelectTab(tab) => {
                self.active_tab = tab;
            }
            Message::NewProject => {
                self.project = ModelProject::default();
                self.active_tab = Tab::Metadata;
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let nav_bar = row![
            text("edge")
                .size(20)
                .color(ThemeColors::PRIMARY),
            Space::new().width(20),
            button(text("1. Omslag & Metadata")).on_press(Message::SelectTab(Tab::Metadata)),
            button(text("2. Begrebsliste (Tabel)")).on_press(Message::SelectTab(Tab::ConceptList)),
            button(text("3. Begrebsmodel (Graf)")).on_press(Message::SelectTab(Tab::ConceptModel)),
            button(text("4. Informationsmodel")).on_press(Message::SelectTab(Tab::InformationModel)),
            Space::new().width(Length::Fill),
            button(text("Nyt Projekt")).on_press(Message::NewProject),
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        let content: Element<Message> = match self.active_tab {
            Tab::Metadata => column![
                text(format!("Model: {}", self.project.metadata().name())).size(24),
                text(format!("Emneområde: {}", self.project.metadata().domain_area())).color(ThemeColors::TEXT_MUTED),
                text(format!("Ansvarlig: {}", self.project.metadata().responsible_org())).color(ThemeColors::TEXT_MUTED),
                text(format!("Version: {} ({:?})", self.project.metadata().version(), self.project.metadata().status())).color(ThemeColors::TEXT_MUTED),
                text(format!("URI: {}", self.project.metadata().uri())).color(ThemeColors::TEXT_MUTED),
                text(format!("Beskrivelse: {}", self.project.metadata().description())),
            ]
            .spacing(12)
            .into(),

            Tab::ConceptList => column![
                text("Begrebsliste (FDA Bilag D & E)").size(22),
                text("Tabelkomponent til oprettelse og vedligeholdelse af begrebsdefinitioner etableres i Fase 2.").color(ThemeColors::TEXT_MUTED),
            ]
            .spacing(10)
            .into(),

            Tab::ConceptModel => column![
                text("Begrebsmodel (Graf)").size(22),
                text("Interaktiv grafkomponent med noder, generaliseringer og associationer etableres i Fase 3.").color(ThemeColors::TEXT_MUTED),
            ]
            .spacing(10)
            .into(),

            Tab::InformationModel => column![
                text("Informationsmodel (UML Klasser)").size(22),
                text("Informationsmodel med attributter, datatyper og multiplicitet etableres i Fase 4.").color(ThemeColors::TEXT_MUTED),
            ]
            .spacing(10)
            .into(),
        };

        let status_bar = row![
            text("FDA Modelregler v2.1 • Klar").size(13).color(ThemeColors::TEXT_MUTED),
            Space::new().width(Length::Fill),
            text(format!("Aktiv fane: {:?}", self.active_tab)).size(13).color(ThemeColors::TEXT_MUTED),
        ]
        .align_y(Alignment::Center);

        container(
            column![
                nav_bar,
                container(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(20),
                status_bar,
            ]
            .spacing(12)
            .padding(16),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
