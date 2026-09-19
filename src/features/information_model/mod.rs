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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Multiplicity {
    lower: u32,
    upper: Option<u32>,
}

impl Multiplicity {
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Attribute {
    name: String,
    data_type: PrimitiveType,
    multiplicity: Multiplicity,
}

impl Attribute {
    pub fn new(
        name: impl Into<String>,
        data_type: PrimitiveType,
        multiplicity: Multiplicity,
    ) -> Self {
        Self {
            name: name.into(),
            data_type,
            multiplicity,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn data_type(&self) -> PrimitiveType {
        self.data_type
    }

    pub fn multiplicity(&self) -> Multiplicity {
        self.multiplicity
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InformationClass {
    concept_id: Uuid,
    name: String,
    attributes: Vec<Attribute>,
}

impl InformationClass {
    pub fn from_concept(concept: &Concept) -> Self {
        Self {
            concept_id: concept.id(),
            name: concept.preferred_term().to_string(),
            attributes: Vec::new(),
        }
    }

    pub fn concept_id(&self) -> Uuid {
        self.concept_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }

    pub fn add_attribute(&mut self, attribute: Attribute) {
        self.attributes.push(attribute);
    }
}
