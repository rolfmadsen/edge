use crate::features::concepts::Concept;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrimitiveType {
    CharacterString,
    Integer,
    Decimal,
    Boolean,
    Date,
    DateTime,
    Time,
    Uri,
}

impl PrimitiveType {
    pub const ALL: &'static [PrimitiveType] = &[
        Self::CharacterString,
        Self::Integer,
        Self::Decimal,
        Self::Boolean,
        Self::Date,
        Self::DateTime,
        Self::Time,
        Self::Uri,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CharacterString => "CharacterString",
            Self::Integer => "Integer",
            Self::Decimal => "Decimal",
            Self::Boolean => "Boolean",
            Self::Date => "Date",
            Self::DateTime => "DateTime",
            Self::Time => "Time",
            Self::Uri => "URI",
        }
    }
}

impl std::fmt::Display for PrimitiveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Multiplicity {
    lower: u32,
    upper: Option<u32>,
}

impl Multiplicity {
    pub const PRESETS: &'static [Multiplicity] = &[
        Multiplicity {
            lower: 1,
            upper: Some(1),
        },
        Multiplicity {
            lower: 0,
            upper: Some(1),
        },
        Multiplicity {
            lower: 0,
            upper: None,
        },
        Multiplicity {
            lower: 1,
            upper: None,
        },
    ];

    pub fn new(lower: u32, upper: Option<u32>) -> Self {
        Self { lower, upper }
    }

    pub fn exactly_one() -> Self {
        Self::new(1, Some(1))
    }

    pub fn zero_or_one() -> Self {
        Self::new(0, Some(1))
    }

    pub fn zero_or_more() -> Self {
        Self::new(0, None)
    }

    pub fn one_or_more() -> Self {
        Self::new(1, None)
    }

    pub fn lower(&self) -> u32 {
        self.lower
    }

    pub fn upper(&self) -> Option<u32> {
        self.upper
    }

    pub fn to_display_string(&self) -> String {
        match self.upper {
            Some(u) => {
                if self.lower == u {
                    format!("{}", self.lower)
                } else {
                    format!("{}..{}", self.lower, u)
                }
            }
            None => format!("{}..*", self.lower),
        }
    }
}

impl std::fmt::Display for Multiplicity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_display_string())
    }
}

pub fn is_lower_camel_case(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => c.is_ascii_lowercase(),
        None => false,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Attribute {
    id: Uuid,
    name: String,
    data_type: PrimitiveType,
    multiplicity: Multiplicity,
    #[serde(default)]
    concept_ids: Vec<Uuid>,
}

impl Attribute {
    pub fn new(
        name: impl Into<String>,
        data_type: PrimitiveType,
        multiplicity: Multiplicity,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            data_type,
            multiplicity,
            concept_ids: Vec::new(),
        }
    }

    pub fn with_concepts(mut self, ids: Vec<Uuid>) -> Self {
        self.concept_ids = ids;
        self
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    pub fn data_type(&self) -> PrimitiveType {
        self.data_type
    }

    pub fn set_data_type(&mut self, data_type: PrimitiveType) {
        self.data_type = data_type;
    }

    pub fn multiplicity(&self) -> Multiplicity {
        self.multiplicity
    }

    pub fn set_multiplicity(&mut self, multiplicity: Multiplicity) {
        self.multiplicity = multiplicity;
    }

    pub fn concept_ids(&self) -> &[Uuid] {
        &self.concept_ids
    }

    pub fn add_concept_id(&mut self, id: Uuid) {
        if !self.concept_ids.contains(&id) {
            self.concept_ids.push(id);
        }
    }

    pub fn remove_concept_id(&mut self, id: Uuid) {
        self.concept_ids.retain(|c| *c != id);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InformationClass {
    id: Uuid,
    name: String,
    description: Option<String>,
    #[serde(default)]
    concept_ids: Vec<Uuid>,
    #[serde(default)]
    attributes: Vec<Attribute>,
}

impl InformationClass {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            description: None,
            concept_ids: Vec::new(),
            attributes: Vec::new(),
        }
    }

    pub fn from_concept(concept: &Concept) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: concept.preferred_term().to_string(),
            description: if concept.definition().is_empty() {
                None
            } else {
                Some(concept.definition().to_string())
            },
            concept_ids: vec![concept.id()],
            attributes: Vec::new(),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn set_description(&mut self, desc: Option<String>) {
        self.description = desc;
    }

    pub fn concept_ids(&self) -> &[Uuid] {
        &self.concept_ids
    }

    pub fn add_concept_id(&mut self, id: Uuid) {
        if !self.concept_ids.contains(&id) {
            self.concept_ids.push(id);
        }
    }

    pub fn remove_concept_id(&mut self, id: Uuid) {
        self.concept_ids.retain(|c| *c != id);
    }

    pub fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }

    pub fn attributes_mut(&mut self) -> &mut Vec<Attribute> {
        &mut self.attributes
    }

    pub fn add_attribute(&mut self, attribute: Attribute) {
        self.attributes.push(attribute);
    }

    pub fn remove_attribute(&mut self, attr_id: Uuid) -> Option<Attribute> {
        if let Some(pos) = self.attributes.iter().position(|a| a.id() == attr_id) {
            Some(self.attributes.remove(pos))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct InformationModel {
    classes: Vec<InformationClass>,
}

impl InformationModel {
    pub fn new() -> Self {
        Self {
            classes: Vec::new(),
        }
    }

    pub fn classes(&self) -> &[InformationClass] {
        &self.classes
    }

    pub fn classes_mut(&mut self) -> &mut Vec<InformationClass> {
        &mut self.classes
    }

    pub fn get_class(&self, id: Uuid) -> Option<&InformationClass> {
        self.classes.iter().find(|c| c.id() == id)
    }

    pub fn get_class_mut(&mut self, id: Uuid) -> Option<&mut InformationClass> {
        self.classes.iter_mut().find(|c| c.id() == id)
    }

    pub fn add_class(&mut self, class: InformationClass) -> Uuid {
        let id = class.id();
        self.classes.push(class);
        id
    }

    pub fn create_class_from_concept(&mut self, concept: &Concept) -> Uuid {
        let class = InformationClass::from_concept(concept);
        self.add_class(class)
    }

    pub fn remove_class(&mut self, id: Uuid) -> Option<InformationClass> {
        if let Some(pos) = self.classes.iter().position(|c| c.id() == id) {
            Some(self.classes.remove(pos))
        } else {
            None
        }
    }

    pub fn classes_for_concept(&self, concept_id: Uuid) -> Vec<&InformationClass> {
        self.classes
            .iter()
            .filter(|c| c.concept_ids().contains(&concept_id))
            .collect()
    }

    pub fn attributes_for_concept(&self, concept_id: Uuid) -> Vec<(&InformationClass, &Attribute)> {
        let mut results = Vec::new();
        for class in &self.classes {
            for attr in class.attributes() {
                if attr.concept_ids().contains(&concept_id) {
                    results.push((class, attr));
                }
            }
        }
        results
    }
}
