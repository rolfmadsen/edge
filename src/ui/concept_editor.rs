use crate::features::concepts::{BelongsToDomain, Concept, ConceptValidator, ValidationError};
use crate::ui::app::Message;
use crate::ui::theme::ThemeColors;
use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Alignment, Element, Length};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConceptFormField {
    PreferredTerm,
    Definition,
    BelongsToDomain,
    ModelUri,
    AcceptedTerm,
    DeprecatedTerm,
    Example,
    Comment,
    ApplicationNote,
    LegalSource,
    Source,
    Identifier,
    DerivedFrom,
}

#[derive(Debug, Clone)]
pub struct ConceptEditorState {
    pub editing_id: Option<Uuid>,
    pub preferred_term: String,
    pub definition: String,
    pub belongs_to_domain: BelongsToDomain,
    pub model_uri: String,
    pub accepted_term: String,
    pub deprecated_term: String,
    pub example: String,
    pub comment: String,
    pub application_note: String,
    pub legal_source: String,
    pub source: String,
    pub identifier: String,
    pub derived_from: String,
    pub show_supplementary: bool,
    pub validation_error: Option<String>,
}

impl Default for ConceptEditorState {
    fn default() -> Self {
        Self::new_empty()
    }
}

impl ConceptEditorState {
    pub fn new_empty() -> Self {
        Self {
            editing_id: None,
            preferred_term: String::new(),
            definition: String::new(),
            belongs_to_domain: BelongsToDomain::Yes,
            model_uri: String::new(),
            accepted_term: String::new(),
            deprecated_term: String::new(),
            example: String::new(),
            comment: String::new(),
            application_note: String::new(),
            legal_source: String::new(),
            source: String::new(),
            identifier: String::new(),
            derived_from: String::new(),
            show_supplementary: false,
            validation_error: None,
        }
    }

    pub fn from_concept(concept: &Concept) -> Self {
        let (belongs, model_uri) = match concept.belongs_to_domain() {
            BelongsToDomain::Yes => (BelongsToDomain::Yes, String::new()),
            BelongsToDomain::No => (BelongsToDomain::No, String::new()),
            BelongsToDomain::ModelRef(uri) => (BelongsToDomain::No, uri.clone()),
        };

        Self {
            editing_id: Some(concept.id()),
            preferred_term: concept.preferred_term().to_string(),
            definition: concept.definition().to_string(),
            belongs_to_domain: belongs,
            model_uri,
            accepted_term: concept.accepted_term().unwrap_or_default().to_string(),
            deprecated_term: concept.deprecated_term().unwrap_or_default().to_string(),
            example: concept.example().unwrap_or_default().to_string(),
            comment: concept.comment().unwrap_or_default().to_string(),
            application_note: concept.application_note().unwrap_or_default().to_string(),
            legal_source: concept.legal_source().unwrap_or_default().to_string(),
            source: concept.source().unwrap_or_default().to_string(),
            identifier: concept.identifier().unwrap_or_default().to_string(),
            derived_from: concept.derived_from().unwrap_or_default().to_string(),
            show_supplementary: concept.accepted_term().is_some()
                || concept.deprecated_term().is_some()
                || concept.example().is_some()
                || concept.comment().is_some()
                || concept.application_note().is_some()
                || concept.identifier().is_some()
                || concept.derived_from().is_some(),
            validation_error: None,
        }
    }

    pub fn set_domain(&mut self, domain: BelongsToDomain) {
        self.belongs_to_domain = domain;
    }

    pub fn update_field(&mut self, field: ConceptFormField, value: String) {
        self.validation_error = None;
        match field {
            ConceptFormField::PreferredTerm => self.preferred_term = value,
            ConceptFormField::Definition => self.definition = value,
            ConceptFormField::BelongsToDomain => {
                let parsed = BelongsToDomain::from_str_loose(&value);
                match parsed {
                    BelongsToDomain::ModelRef(uri) => {
                        self.belongs_to_domain = BelongsToDomain::No;
                        self.model_uri = uri;
                    }
                    other => {
                        self.belongs_to_domain = other;
                    }
                }
            }
            ConceptFormField::ModelUri => self.model_uri = value,
            ConceptFormField::AcceptedTerm => self.accepted_term = value,
            ConceptFormField::DeprecatedTerm => self.deprecated_term = value,
            ConceptFormField::Example => self.example = value,
            ConceptFormField::Comment => self.comment = value,
            ConceptFormField::ApplicationNote => self.application_note = value,
            ConceptFormField::LegalSource => self.legal_source = value,
            ConceptFormField::Source => self.source = value,
            ConceptFormField::Identifier => self.identifier = value,
            ConceptFormField::DerivedFrom => self.derived_from = value,
        }
    }

    pub fn build_concept(&self) -> Result<Concept, ValidationError> {
        let belongs = match &self.belongs_to_domain {
            BelongsToDomain::Yes => BelongsToDomain::Yes,
            BelongsToDomain::No | BelongsToDomain::ModelRef(_) => {
                let trimmed = self.model_uri.trim();
                if trimmed.is_empty() {
                    BelongsToDomain::No
                } else {
                    BelongsToDomain::ModelRef(trimmed.to_string())
                }
            }
        };
        let mut concept = Concept::new(&self.preferred_term, &self.definition, belongs.clone());

        if let Some(id) = self.editing_id {
            concept = Concept::new_with_id(id, &self.preferred_term, &self.definition, belongs);
        }

        let opt = |s: &str| {
            let t = s.trim();
            if t.is_empty() {
                None
            } else {
                Some(t.to_string())
            }
        };

        concept.set_accepted_term(opt(&self.accepted_term));
        concept.set_deprecated_term(opt(&self.deprecated_term));
        concept.set_example(opt(&self.example));
        concept.set_comment(opt(&self.comment));
        concept.set_application_note(opt(&self.application_note));
        concept.set_legal_source(opt(&self.legal_source));
        concept.set_source(opt(&self.source));
        concept.set_identifier(opt(&self.identifier));
        concept.set_derived_from(opt(&self.derived_from));

        ConceptValidator::validate(&concept)?;
        Ok(concept)
    }

    pub fn view(&self) -> Element<'_, Message> {
        use crate::ui::theme::{
            card_container_style, modern_input_style, primary_button_style, secondary_button_style,
        };

        let is_edit = self.editing_id.is_some();
        let title_text = if is_edit {
            format!("Rediger Begreb: {}", self.preferred_term)
        } else {
            "Opret Nyt Begreb".to_string()
        };

        let title_row = row![
            text(title_text).size(20).color(ThemeColors::SLATE_900),
            Space::new().width(Length::Fill),
            button(text("✕").size(14))
                .style(secondary_button_style)
                .on_press(Message::CancelConceptEdit),
        ]
        .align_y(Alignment::Center);

        let mut form = column![title_row].spacing(16);

        if let Some(err) = &self.validation_error {
            form = form.push(
                container(
                    row![
                        text("⚠️ ").size(16),
                        text(err).color(ThemeColors::ACCENT_RED).size(13),
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center),
                )
                .style(|_theme: &iced::Theme| container::Style {
                    background: Some(iced::Background::Color(ThemeColors::ACCENT_RED_LIGHT)),
                    border: iced::Border {
                        color: ThemeColors::ACCENT_RED,
                        width: 1.0,
                        radius: 6.0.into(),
                    },
                    ..Default::default()
                })
                .padding([10, 14]),
            );
        }

        // 1. Primære felter (Foretrukken term, Emneområde)
        let term_input = column![
            text("Foretrukken dansk term *")
                .size(13)
                .color(ThemeColors::SLATE_800),
            text_input(
                "F.eks. Køretøj, Personbil, Myndighed...",
                &self.preferred_term
            )
            .id("preferred_term_input")
            .style(modern_input_style)
            .on_input(|v| Message::UpdateConceptField(ConceptFormField::PreferredTerm, v))
            .padding(8),
            text("Den officielle primære sproglige betegnelse (§18, §19)")
                .size(11)
                .color(ThemeColors::SLATE_500),
        ]
        .spacing(4)
        .width(Length::FillPortion(2));

        let is_local = self.belongs_to_domain == BelongsToDomain::Yes;
        let mut domain_col = column![
            text("Domæne / Kontekst (Tilhører emneområde §26) *")
                .size(13)
                .color(ThemeColors::SLATE_800),
            row![
                button(text("Lokalt begreb (Ja)").size(11))
                    .style(if is_local {
                        primary_button_style
                    } else {
                        secondary_button_style
                    })
                    .on_press(Message::SetConceptDomain(BelongsToDomain::Yes))
                    .padding([6, 12]),
                button(text("Indlånt begreb (Nej)").size(11))
                    .style(if !is_local {
                        primary_button_style
                    } else {
                        secondary_button_style
                    })
                    .on_press(Message::SetConceptDomain(BelongsToDomain::No))
                    .padding([6, 12]),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            text("Lokale begreber defineres her (FDA Sand). Indlånte markeres med FDA Blå (§26)")
                .size(11)
                .color(ThemeColors::SLATE_500),
        ]
        .spacing(4)
        .width(Length::FillPortion(2));

        if !is_local {
            domain_col = domain_col.push(
                column![
                    text("Kildemodel URI (valgfri)")
                        .size(12)
                        .color(ThemeColors::SLATE_700),
                    text_input("F.eks. https://data.gov.dk/model/core/cpr", &self.model_uri)
                        .style(modern_input_style)
                        .on_input(|v| Message::UpdateConceptField(ConceptFormField::ModelUri, v))
                        .padding(6),
                    text("URI på den eksterne FDA-model, begrebet er defineret i")
                        .size(11)
                        .color(ThemeColors::SLATE_500),
                ]
                .spacing(3),
            );
        }

        let term_domain_row = row![term_input, domain_col].spacing(16);

        // 2. Definition
        let definition_input = column![
            text("Definition * (Aristoteles' formel)").size(13).color(ThemeColors::SLATE_800),
            text_input(
                "Genus proximum + differentia specifica (hvad er det, og hvad adskiller det)...",
                &self.definition,
            )
            .style(modern_input_style)
            .on_input(|v| Message::UpdateConceptField(ConceptFormField::Definition, v))
            .padding(10),
            text("Formuleret iht. Aristoteles' formel. Undgå cirkulære eller negative definitioner (§20-§22)")
                .size(11)
                .color(ThemeColors::SLATE_500),
        ]
        .spacing(4);

        // 3. Kilder
        let legal_source_input = column![
            text("Juridisk kilde (Lovhjemmel)")
                .size(13)
                .color(ThemeColors::SLATE_800),
            text_input("F.eks. LBK nr 1324 af 21/11/2023 § 2", &self.legal_source)
                .style(modern_input_style)
                .on_input(|v| Message::UpdateConceptField(ConceptFormField::LegalSource, v))
                .padding(8),
        ]
        .spacing(4)
        .width(Length::FillPortion(1));

        let general_source_input = column![
            text("Kilde").size(13).color(ThemeColors::SLATE_800),
            text_input(
                "F.eks. Dansk Standard, ISO 10241, fagordbog...",
                &self.source
            )
            .style(modern_input_style)
            .on_input(|v| Message::UpdateConceptField(ConceptFormField::Source, v))
            .padding(8),
        ]
        .spacing(4)
        .width(Length::FillPortion(1));

        let sources_row = row![legal_source_input, general_source_input].spacing(16);

        let primary_card =
            container(column![term_domain_row, definition_input, sources_row].spacing(14))
                .style(card_container_style)
                .padding(16)
                .width(Length::Fill);

        form = form.push(primary_card);

        // Foldbar sektion med supplerende felter
        let toggle_supplementary_btn = button(
            text(if self.show_supplementary {
                "▲ Skjul supplerende felter"
            } else {
                "▼ Vis supplerende felter (Accepteret term, Eksempel, Identifikator m.fl.)"
            })
            .size(13),
        )
        .style(secondary_button_style)
        .on_press(Message::ToggleShowAllFields)
        .padding([7, 14]);

        form = form.push(toggle_supplementary_btn);

        if self.show_supplementary {
            let supplementary_content = column![
                row![
                    column![
                        text("Accepteret dansk term")
                            .size(13)
                            .color(ThemeColors::SLATE_700),
                        text_input("Synonym eller tilladt betegnelse...", &self.accepted_term)
                            .style(modern_input_style)
                            .on_input(|v| Message::UpdateConceptField(
                                ConceptFormField::AcceptedTerm,
                                v
                            ))
                            .padding(8),
                    ]
                    .spacing(4)
                    .width(Length::FillPortion(1)),
                    column![
                        text("Frarådet dansk term")
                            .size(13)
                            .color(ThemeColors::SLATE_700),
                        text_input("Betegnelse der ikke bør anvendes...", &self.deprecated_term)
                            .style(modern_input_style)
                            .on_input(|v| Message::UpdateConceptField(
                                ConceptFormField::DeprecatedTerm,
                                v
                            ))
                            .padding(8),
                    ]
                    .spacing(4)
                    .width(Length::FillPortion(1)),
                ]
                .spacing(16),
                row![
                    column![
                        text("Eksempel").size(13).color(ThemeColors::SLATE_700),
                        text_input("Typisk tilfælde der illustrerer begrebet...", &self.example)
                            .style(modern_input_style)
                            .on_input(|v| Message::UpdateConceptField(ConceptFormField::Example, v))
                            .padding(8),
                    ]
                    .spacing(4)
                    .width(Length::FillPortion(1)),
                    column![
                        text("Kommentar").size(13).color(ThemeColors::SLATE_700),
                        text_input("Supplerende bemærkning...", &self.comment)
                            .style(modern_input_style)
                            .on_input(|v| Message::UpdateConceptField(ConceptFormField::Comment, v))
                            .padding(8),
                    ]
                    .spacing(4)
                    .width(Length::FillPortion(1)),
                ]
                .spacing(16),
                column![
                    text("Anvendelsesnote")
                        .size(13)
                        .color(ThemeColors::SLATE_700),
                    text_input(
                        "Note om specifik anvendelseskontekst...",
                        &self.application_note
                    )
                    .style(modern_input_style)
                    .on_input(|v| Message::UpdateConceptField(ConceptFormField::ApplicationNote, v))
                    .padding(8),
                ]
                .spacing(4),
                row![
                    column![
                        text("Identifikator (HTTP-URI)")
                            .size(13)
                            .color(ThemeColors::SLATE_700),
                        text_input(
                            "https://data.gov.dk/model/core/domain/Term",
                            &self.identifier
                        )
                        .style(modern_input_style)
                        .on_input(|v| Message::UpdateConceptField(ConceptFormField::Identifier, v))
                        .padding(8),
                        text("Begrebets egen persistente URI i begrebskataloget (Bilag D)")
                            .size(11)
                            .color(ThemeColors::SLATE_500),
                    ]
                    .spacing(4)
                    .width(Length::FillPortion(1)),
                    column![
                        text("Afledt af").size(13).color(ThemeColors::SLATE_700),
                        text_input("HTTP-URI på oprindeligt begreb...", &self.derived_from)
                            .style(modern_input_style)
                            .on_input(|v| Message::UpdateConceptField(
                                ConceptFormField::DerivedFrom,
                                v
                            ))
                            .padding(8),
                    ]
                    .spacing(4)
                    .width(Length::FillPortion(1)),
                ]
                .spacing(16),
            ]
            .spacing(14);

            let supp_card = container(supplementary_content)
                .style(card_container_style)
                .padding(16)
                .width(Length::Fill);

            form = form.push(supp_card);
        }

        let action_bar = row![
            Space::new().width(Length::Fill),
            button(text("Annuller").size(13))
                .style(secondary_button_style)
                .on_press(Message::CancelConceptEdit)
                .padding([8, 16]),
            button(
                text(if is_edit {
                    "Gem ændringer"
                } else {
                    "Opret begreb"
                })
                .size(13)
            )
            .style(primary_button_style)
            .on_press(Message::SaveConcept)
            .padding([8, 20]),
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        form = form.push(Space::new().height(8));
        form = form.push(action_bar);

        scrollable(form).height(Length::Fill).into()
    }
}
