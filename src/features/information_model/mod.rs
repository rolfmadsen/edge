use crate::features::concept_model::{NodeId, PortSide, RelationKind, GRID_SIZE};
use crate::features::concepts::Concept;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod linter;
pub use linter::{
    is_lower_camel_case, is_upper_camel_case, NamingConvention, NamingIssue, NamingIssueKind,
    NamingLinter, NamingSeverity, NamingTarget,
};

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InformationDataType {
    Primitive(PrimitiveType),
    Enumeration { enumeration_id: Uuid },
    Structured { structured_id: Uuid },
}

impl From<PrimitiveType> for InformationDataType {
    fn from(p: PrimitiveType) -> Self {
        Self::Primitive(p)
    }
}

impl Default for InformationDataType {
    fn default() -> Self {
        Self::Primitive(PrimitiveType::CharacterString)
    }
}

impl PartialEq<PrimitiveType> for InformationDataType {
    fn eq(&self, other: &PrimitiveType) -> bool {
        match self {
            Self::Primitive(p) => p == other,
            _ => false,
        }
    }
}

impl PartialEq<InformationDataType> for PrimitiveType {
    fn eq(&self, other: &InformationDataType) -> bool {
        other == self
    }
}

impl PartialEq<PrimitiveType> for &InformationDataType {
    fn eq(&self, other: &PrimitiveType) -> bool {
        *self == other
    }
}

impl PartialEq<&InformationDataType> for PrimitiveType {
    fn eq(&self, other: &&InformationDataType) -> bool {
        *other == self
    }
}

impl InformationDataType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Primitive(p) => p.as_str(),
            Self::Enumeration { .. } => "«enumeration»",
            Self::Structured { .. } => "«dataType»",
        }
    }

    pub fn display_name(&self, model: &InformationModel) -> String {
        match self {
            Self::Primitive(p) => p.as_str().to_string(),
            Self::Enumeration { enumeration_id } => model
                .get_enumeration(*enumeration_id)
                .map(|e| e.name().to_string())
                .unwrap_or_else(|| "«enumeration»".to_string()),
            Self::Structured { structured_id } => model
                .get_structured_type(*structured_id)
                .map(|dt| dt.name().to_string())
                .unwrap_or_else(|| "«dataType»".to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InformationEnumeration {
    id: Uuid,
    name: String,
    #[serde(default)]
    definition: Option<String>,
    #[serde(default)]
    values: Vec<String>,
}

impl InformationEnumeration {
    pub fn new(name: impl Into<String>, values: Vec<String>) -> Self {
        Self::new_with_id(Uuid::new_v4(), name, values)
    }

    pub fn new_with_id(id: Uuid, name: impl Into<String>, values: Vec<String>) -> Self {
        Self {
            id,
            name: name.into(),
            definition: None,
            values,
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

    pub fn definition(&self) -> Option<&str> {
        self.definition.as_deref()
    }

    pub fn set_definition(&mut self, definition: Option<String>) {
        self.definition = definition;
    }

    pub fn values(&self) -> &[String] {
        &self.values
    }

    pub fn values_mut(&mut self) -> &mut Vec<String> {
        &mut self.values
    }

    pub fn add_value(&mut self, val: impl Into<String>) {
        let v = val.into();
        if !self.values.contains(&v) {
            self.values.push(v);
        }
    }

    pub fn remove_value(&mut self, val: &str) {
        self.values.retain(|v| v != val);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructuredDataType {
    id: Uuid,
    name: String,
    #[serde(default)]
    definition: Option<String>,
    #[serde(default)]
    attributes: Vec<Attribute>,
}

impl StructuredDataType {
    pub fn new(name: impl Into<String>) -> Self {
        Self::new_with_id(Uuid::new_v4(), name)
    }

    pub fn new_with_id(id: Uuid, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            definition: None,
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

    pub fn definition(&self) -> Option<&str> {
        self.definition.as_deref()
    }

    pub fn set_definition(&mut self, definition: Option<String>) {
        self.definition = definition;
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Attribute {
    id: Uuid,
    name: String,
    data_type: InformationDataType,
    multiplicity: Multiplicity,
    #[serde(default)]
    concept_ids: Vec<Uuid>,
}

impl Attribute {
    pub fn new(
        name: impl Into<String>,
        data_type: impl Into<InformationDataType>,
        multiplicity: Multiplicity,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            data_type: data_type.into(),
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

    pub fn data_type(&self) -> &InformationDataType {
        &self.data_type
    }

    pub fn set_data_type(&mut self, data_type: impl Into<InformationDataType>) {
        self.data_type = data_type.into();
    }

    pub fn primitive_type(&self) -> Option<PrimitiveType> {
        match self.data_type {
            InformationDataType::Primitive(p) => Some(p),
            _ => None,
        }
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

    pub fn set_concept_ids(&mut self, ids: Vec<Uuid>) {
        self.concept_ids = ids;
    }

    pub fn clear_concept_ids(&mut self) {
        self.concept_ids.clear();
    }
}

fn default_true() -> bool {
    true
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
    #[serde(default)]
    is_abstract: bool,
    #[serde(default = "default_true")]
    is_local: bool,
    #[serde(default)]
    origin_model: Option<String>,
}

impl InformationClass {
    pub fn new(name: impl Into<String>) -> Self {
        Self::new_with_id(Uuid::new_v4(), name)
    }

    pub fn new_with_id(id: Uuid, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            description: None,
            concept_ids: Vec::new(),
            attributes: Vec::new(),
            is_abstract: false,
            is_local: true,
            origin_model: None,
        }
    }

    pub fn from_concept(concept: &Concept) -> Self {
        let (is_local, origin_model) = match concept.belongs_to_domain() {
            crate::features::concepts::BelongsToDomain::Yes => (true, None),
            crate::features::concepts::BelongsToDomain::No => (false, None),
            crate::features::concepts::BelongsToDomain::ModelRef(uri) => (false, Some(uri.clone())),
        };

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
            is_abstract: false,
            is_local,
            origin_model,
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

    pub fn is_abstract(&self) -> bool {
        self.is_abstract
    }

    pub fn set_abstract(&mut self, is_abstract: bool) {
        self.is_abstract = is_abstract;
    }

    pub fn is_local(&self) -> bool {
        self.is_local
    }

    pub fn set_local(&mut self, is_local: bool) {
        self.is_local = is_local;
    }

    pub fn origin_model(&self) -> Option<&str> {
        self.origin_model.as_deref()
    }

    pub fn set_origin_model(&mut self, origin: Option<String>) {
        self.origin_model = origin;
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassRelation {
    #[serde(default = "Uuid::new_v4")]
    id: Uuid,
    from_class: Uuid,
    to_class: Uuid,
    kind: RelationKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    source_port: Option<PortSide>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    target_port: Option<PortSide>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    directed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    source_multiplicity: Option<Multiplicity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    target_multiplicity: Option<Multiplicity>,
}

impl ClassRelation {
    pub fn new(
        from_class: Uuid,
        to_class: Uuid,
        kind: RelationKind,
        label: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            from_class,
            to_class,
            kind,
            label,
            source_port: None,
            target_port: None,
            directed: if kind == RelationKind::Association {
                Some(true)
            } else {
                None
            },
            source_multiplicity: None,
            target_multiplicity: None,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn with_multiplicities(
        from_class: Uuid,
        to_class: Uuid,
        kind: RelationKind,
        label: Option<String>,
        source_port: Option<PortSide>,
        target_port: Option<PortSide>,
        directed: Option<bool>,
        source_multiplicity: Option<Multiplicity>,
        target_multiplicity: Option<Multiplicity>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            from_class,
            to_class,
            kind,
            label,
            source_port,
            target_port,
            directed,
            source_multiplicity,
            target_multiplicity,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn from_class(&self) -> Uuid {
        self.from_class
    }

    pub fn to_class(&self) -> Uuid {
        self.to_class
    }

    pub fn kind(&self) -> RelationKind {
        self.kind
    }

    pub fn set_kind(&mut self, kind: RelationKind) {
        self.kind = kind;
        if kind == RelationKind::Association && self.directed.is_none() {
            self.directed = Some(true);
        }
    }

    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    pub fn set_label(&mut self, label: Option<String>) {
        self.label = label;
    }

    pub fn source_port(&self) -> Option<PortSide> {
        self.source_port
    }

    pub fn target_port(&self) -> Option<PortSide> {
        self.target_port
    }

    pub fn set_ports(&mut self, source_port: Option<PortSide>, target_port: Option<PortSide>) {
        self.source_port = source_port;
        self.target_port = target_port;
    }

    pub fn is_directed(&self) -> bool {
        if self.kind == RelationKind::Association {
            self.directed.unwrap_or(true)
        } else {
            false
        }
    }

    pub fn directed(&self) -> Option<bool> {
        self.directed
    }

    pub fn set_directed(&mut self, directed: bool) {
        self.directed = Some(directed);
    }

    pub fn source_multiplicity(&self) -> Option<Multiplicity> {
        self.source_multiplicity
    }

    pub fn set_source_multiplicity(&mut self, mult: Option<Multiplicity>) {
        self.source_multiplicity = mult;
    }

    pub fn target_multiplicity(&self) -> Option<Multiplicity> {
        self.target_multiplicity
    }

    pub fn set_target_multiplicity(&mut self, mult: Option<Multiplicity>) {
        self.target_multiplicity = mult;
    }

    pub fn reverse(&mut self) {
        std::mem::swap(&mut self.from_class, &mut self.to_class);
        std::mem::swap(&mut self.source_port, &mut self.target_port);
        std::mem::swap(&mut self.source_multiplicity, &mut self.target_multiplicity);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct InformationModel {
    classes: Vec<InformationClass>,
    #[serde(default)]
    enumerations: Vec<InformationEnumeration>,
    #[serde(default)]
    structured_types: Vec<StructuredDataType>,
    #[serde(default)]
    relations: Vec<ClassRelation>,
}

impl InformationModel {
    pub fn new() -> Self {
        Self {
            classes: Vec::new(),
            enumerations: Vec::new(),
            structured_types: Vec::new(),
            relations: Vec::new(),
        }
    }

    pub fn relations(&self) -> &[ClassRelation] {
        &self.relations
    }

    pub fn relations_mut(&mut self) -> &mut Vec<ClassRelation> {
        &mut self.relations
    }

    pub fn add_relation(&mut self, relation: ClassRelation) -> Uuid {
        let id = relation.id();
        if let Some(existing) = self.relations.iter_mut().find(|r| {
            r.from_class == relation.from_class
                && r.to_class == relation.to_class
                && r.kind == relation.kind
        }) {
            *existing = relation;
            return existing.id();
        }
        self.relations.push(relation);
        id
    }

    pub fn remove_relation(&mut self, from_class: Uuid, to_class: Uuid) -> bool {
        let prev_len = self.relations.len();
        self.relations
            .retain(|r| !(r.from_class == from_class && r.to_class == to_class));
        self.relations.len() < prev_len
    }

    pub fn find_relation(&self, from_class: Uuid, to_class: Uuid) -> Option<&ClassRelation> {
        self.relations
            .iter()
            .find(|r| r.from_class == from_class && r.to_class == to_class)
    }

    pub fn find_relation_mut(
        &mut self,
        from_class: Uuid,
        to_class: Uuid,
    ) -> Option<&mut ClassRelation> {
        self.relations
            .iter_mut()
            .find(|r| r.from_class == from_class && r.to_class == to_class)
    }

    pub fn relations_for_class(&self, class_id: Uuid) -> Vec<&ClassRelation> {
        self.relations
            .iter()
            .filter(|r| r.from_class == class_id || r.to_class == class_id)
            .collect()
    }

    pub fn remove_relations_for_class(&mut self, class_id: Uuid) {
        self.relations
            .retain(|r| r.from_class != class_id && r.to_class != class_id);
    }

    pub fn update_relation_kind(
        &mut self,
        from_class: Uuid,
        to_class: Uuid,
        kind: RelationKind,
    ) -> bool {
        if let Some(rel) = self.find_relation_mut(from_class, to_class) {
            rel.set_kind(kind);
            true
        } else {
            false
        }
    }

    pub fn update_relation_label(
        &mut self,
        from_class: Uuid,
        to_class: Uuid,
        label: Option<String>,
    ) -> bool {
        if let Some(rel) = self.find_relation_mut(from_class, to_class) {
            rel.set_label(label);
            true
        } else {
            false
        }
    }

    pub fn update_relation_source_multiplicity(
        &mut self,
        from_class: Uuid,
        to_class: Uuid,
        mult: Option<Multiplicity>,
    ) -> bool {
        if let Some(rel) = self.find_relation_mut(from_class, to_class) {
            rel.set_source_multiplicity(mult);
            true
        } else {
            false
        }
    }

    pub fn update_relation_target_multiplicity(
        &mut self,
        from_class: Uuid,
        to_class: Uuid,
        mult: Option<Multiplicity>,
    ) -> bool {
        if let Some(rel) = self.find_relation_mut(from_class, to_class) {
            rel.set_target_multiplicity(mult);
            true
        } else {
            false
        }
    }

    pub fn update_relation_directed(
        &mut self,
        from_class: Uuid,
        to_class: Uuid,
        directed: bool,
    ) -> bool {
        if let Some(rel) = self.find_relation_mut(from_class, to_class) {
            rel.set_directed(directed);
            true
        } else {
            false
        }
    }

    pub fn update_relation_ports(
        &mut self,
        from_class: Uuid,
        to_class: Uuid,
        source_port: Option<PortSide>,
        target_port: Option<PortSide>,
    ) -> bool {
        if let Some(rel) = self.find_relation_mut(from_class, to_class) {
            rel.set_ports(source_port, target_port);
            true
        } else {
            false
        }
    }

    pub fn reverse_relation(&mut self, from_class: Uuid, to_class: Uuid) -> bool {
        if let Some(pos) = self
            .relations
            .iter()
            .position(|r| r.from_class == from_class && r.to_class == to_class)
        {
            self.relations[pos].reverse();
            true
        } else {
            false
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
        self.remove_relations_for_class(id);
        if let Some(pos) = self.classes.iter().position(|c| c.id() == id) {
            Some(self.classes.remove(pos))
        } else {
            None
        }
    }

    pub fn enumerations(&self) -> &[InformationEnumeration] {
        &self.enumerations
    }

    pub fn enumerations_mut(&mut self) -> &mut Vec<InformationEnumeration> {
        &mut self.enumerations
    }

    pub fn get_enumeration(&self, id: Uuid) -> Option<&InformationEnumeration> {
        self.enumerations.iter().find(|e| e.id() == id)
    }

    pub fn get_enumeration_mut(&mut self, id: Uuid) -> Option<&mut InformationEnumeration> {
        self.enumerations.iter_mut().find(|e| e.id() == id)
    }

    pub fn add_enumeration(&mut self, enumeration: InformationEnumeration) -> Uuid {
        let id = enumeration.id();
        self.enumerations.push(enumeration);
        id
    }

    pub fn remove_enumeration(&mut self, id: Uuid) -> Option<InformationEnumeration> {
        self.remove_relations_for_class(id);
        if let Some(pos) = self.enumerations.iter().position(|e| e.id() == id) {
            Some(self.enumerations.remove(pos))
        } else {
            None
        }
    }

    pub fn sync_attribute_dependencies(&mut self) {
        let mut desired_deps: std::collections::HashSet<(Uuid, Uuid)> =
            std::collections::HashSet::new();
        for class in &self.classes {
            for attr in class.attributes() {
                if let InformationDataType::Enumeration { enumeration_id } = attr.data_type() {
                    desired_deps.insert((class.id(), *enumeration_id));
                }
            }
        }
        let all_enum_ids: std::collections::HashSet<Uuid> =
            self.enumerations.iter().map(|e| e.id()).collect();
        self.relations.retain(|rel| {
            if rel.kind() == RelationKind::Dependency && all_enum_ids.contains(&rel.to_class()) {
                desired_deps.contains(&(rel.from_class(), rel.to_class()))
            } else {
                true
            }
        });
        for (from, to) in desired_deps {
            if self.find_relation(from, to).is_none() {
                self.relations.push(ClassRelation::new(
                    from,
                    to,
                    RelationKind::Dependency,
                    Some("«use»".to_string()),
                ));
            }
        }
    }

    pub fn structured_types(&self) -> &[StructuredDataType] {
        &self.structured_types
    }

    pub fn structured_types_mut(&mut self) -> &mut Vec<StructuredDataType> {
        &mut self.structured_types
    }

    pub fn get_structured_type(&self, id: Uuid) -> Option<&StructuredDataType> {
        self.structured_types.iter().find(|dt| dt.id() == id)
    }

    pub fn get_structured_type_mut(&mut self, id: Uuid) -> Option<&mut StructuredDataType> {
        self.structured_types.iter_mut().find(|dt| dt.id() == id)
    }

    pub fn add_structured_type(&mut self, structured_type: StructuredDataType) -> Uuid {
        let id = structured_type.id();
        self.structured_types.push(structured_type);
        id
    }

    pub fn remove_structured_type(&mut self, id: Uuid) -> Option<StructuredDataType> {
        if let Some(pos) = self.structured_types.iter().position(|dt| dt.id() == id) {
            Some(self.structured_types.remove(pos))
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

    pub fn remove_concept_references(&mut self, concept_id: Uuid) {
        for class in &mut self.classes {
            class.remove_concept_id(concept_id);
            for attr in class.attributes_mut() {
                attr.remove_concept_id(concept_id);
            }
        }
    }
}

pub const DEFAULT_CLASS_NODE_WIDTH: f32 = 220.0;
pub const MIN_CLASS_NODE_HEIGHT: f32 = 80.0;
pub const ATTR_LINE_HEIGHT: f32 = 20.0;

pub fn calculate_class_node_height(attr_count: usize) -> f32 {
    let header_height = 54.0;
    let compartment_padding = 16.0;
    let attrs_height = (attr_count as f32) * ATTR_LINE_HEIGHT;
    let raw_height =
        (header_height + compartment_padding + attrs_height).max(MIN_CLASS_NODE_HEIGHT);
    (raw_height / GRID_SIZE).ceil() * GRID_SIZE
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassDiagramNode {
    id: NodeId,
    class_id: Uuid,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl ClassDiagramNode {
    pub fn new(class_id: Uuid, x: f32, y: f32, attr_count: usize) -> Self {
        Self {
            id: class_id,
            class_id,
            x,
            y,
            width: DEFAULT_CLASS_NODE_WIDTH,
            height: calculate_class_node_height(attr_count),
        }
    }

    pub fn id(&self) -> NodeId {
        self.id
    }

    pub fn class_id(&self) -> Uuid {
        self.class_id
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        (self.height / GRID_SIZE).ceil() * GRID_SIZE
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn update_dimensions(&mut self, attr_count: usize) {
        self.height = calculate_class_node_height(attr_count);
    }

    pub fn center(&self) -> (f32, f32) {
        (self.x + self.width / 2.0, self.y + self.height() / 2.0)
    }

    pub fn contains(&self, px: f32, py: f32) -> bool {
        px >= self.x && px <= self.x + self.width && py >= self.y && py <= self.y + self.height()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassDiagramEdge {
    from: NodeId,
    to: NodeId,
    kind: RelationKind,
    label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    source_port: Option<PortSide>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    target_port: Option<PortSide>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    directed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    source_multiplicity: Option<Multiplicity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    target_multiplicity: Option<Multiplicity>,
}

impl ClassDiagramEdge {
    pub fn new(from: NodeId, to: NodeId, kind: RelationKind, label: Option<String>) -> Self {
        Self {
            from,
            to,
            kind,
            label,
            source_port: None,
            target_port: None,
            directed: if kind == RelationKind::Association {
                Some(true)
            } else {
                None
            },
            source_multiplicity: None,
            target_multiplicity: None,
        }
    }

    pub fn with_ports(
        from: NodeId,
        to: NodeId,
        kind: RelationKind,
        label: Option<String>,
        source_port: Option<PortSide>,
        target_port: Option<PortSide>,
    ) -> Self {
        Self {
            from,
            to,
            kind,
            label,
            source_port,
            target_port,
            directed: if kind == RelationKind::Association {
                Some(true)
            } else {
                None
            },
            source_multiplicity: None,
            target_multiplicity: None,
        }
    }

    pub fn with_all(
        from: NodeId,
        to: NodeId,
        kind: RelationKind,
        label: Option<String>,
        source_port: Option<PortSide>,
        target_port: Option<PortSide>,
        directed: Option<bool>,
    ) -> Self {
        Self {
            from,
            to,
            kind,
            label,
            source_port,
            target_port,
            directed,
            source_multiplicity: None,
            target_multiplicity: None,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn with_multiplicities(
        from: NodeId,
        to: NodeId,
        kind: RelationKind,
        label: Option<String>,
        source_port: Option<PortSide>,
        target_port: Option<PortSide>,
        directed: Option<bool>,
        source_multiplicity: Option<Multiplicity>,
        target_multiplicity: Option<Multiplicity>,
    ) -> Self {
        Self {
            from,
            to,
            kind,
            label,
            source_port,
            target_port,
            directed,
            source_multiplicity,
            target_multiplicity,
        }
    }

    pub fn from(&self) -> NodeId {
        self.from
    }

    pub fn to(&self) -> NodeId {
        self.to
    }

    pub fn kind(&self) -> RelationKind {
        self.kind
    }

    pub fn set_kind(&mut self, kind: RelationKind) {
        self.kind = kind;
        if kind == RelationKind::Association && self.directed.is_none() {
            self.directed = Some(true);
        }
    }

    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    pub fn set_label(&mut self, label: Option<String>) {
        self.label = label;
    }

    pub fn source_port(&self) -> Option<PortSide> {
        self.source_port
    }

    pub fn target_port(&self) -> Option<PortSide> {
        self.target_port
    }

    pub fn set_ports(&mut self, source_port: Option<PortSide>, target_port: Option<PortSide>) {
        self.source_port = source_port;
        self.target_port = target_port;
    }

    pub fn is_directed(&self) -> bool {
        if self.kind == RelationKind::Association {
            self.directed.unwrap_or(true)
        } else {
            false
        }
    }

    pub fn directed(&self) -> Option<bool> {
        self.directed
    }

    pub fn set_directed(&mut self, directed: bool) {
        self.directed = Some(directed);
    }

    pub fn source_multiplicity(&self) -> Option<Multiplicity> {
        self.source_multiplicity
    }

    pub fn set_source_multiplicity(&mut self, mult: Option<Multiplicity>) {
        self.source_multiplicity = mult;
    }

    pub fn target_multiplicity(&self) -> Option<Multiplicity> {
        self.target_multiplicity
    }

    pub fn set_target_multiplicity(&mut self, mult: Option<Multiplicity>) {
        self.target_multiplicity = mult;
    }
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassGraph {
    nodes: Vec<ClassDiagramNode>,
    edges: Vec<ClassDiagramEdge>,
}

impl ClassGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn nodes(&self) -> &[ClassDiagramNode] {
        &self.nodes
    }

    pub fn nodes_mut(&mut self) -> &mut Vec<ClassDiagramNode> {
        &mut self.nodes
    }

    pub fn edges(&self) -> &[ClassDiagramEdge] {
        &self.edges
    }

    pub fn edges_mut(&mut self) -> &mut Vec<ClassDiagramEdge> {
        &mut self.edges
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn find_node(&self, id: NodeId) -> Option<&ClassDiagramNode> {
        self.nodes.iter().find(|n| n.id() == id)
    }

    pub fn find_node_mut(&mut self, id: NodeId) -> Option<&mut ClassDiagramNode> {
        self.nodes.iter_mut().find(|n| n.id() == id)
    }

    pub fn find_node_by_class(&self, class_id: Uuid) -> Option<&ClassDiagramNode> {
        self.nodes.iter().find(|n| n.class_id() == class_id)
    }

    pub fn find_node_by_class_mut(&mut self, class_id: Uuid) -> Option<&mut ClassDiagramNode> {
        self.nodes.iter_mut().find(|n| n.class_id() == class_id)
    }

    pub fn is_class_on_diagram(&self, class_id: Uuid) -> bool {
        self.nodes.iter().any(|n| n.class_id() == class_id)
    }

    pub fn add_node(&mut self, class_id: Uuid, attr_count: usize) -> NodeId {
        if let Some(existing) = self.find_node_by_class(class_id) {
            return existing.id();
        }
        let node_count = self.nodes.len() as f32;
        let x = 60.0 + (node_count % 3.0) * 260.0;
        let y = 60.0 + (node_count / 3.0).floor() * 160.0;
        let node = ClassDiagramNode::new(class_id, x, y, attr_count);
        let id = node.id();
        self.nodes.push(node);
        id
    }

    pub fn add_node_at(&mut self, class_id: Uuid, x: f32, y: f32, attr_count: usize) -> NodeId {
        if let Some(existing) = self.find_node_by_class_mut(class_id) {
            existing.set_position(x, y);
            return existing.id();
        }
        let node = ClassDiagramNode::new(class_id, x, y, attr_count);
        let id = node.id();
        self.nodes.push(node);
        id
    }

    pub fn remove_node(&mut self, node_id: NodeId) {
        self.nodes.retain(|n| n.id() != node_id);
        self.edges
            .retain(|e| e.from() != node_id && e.to() != node_id);
    }

    pub fn remove_class_node(&mut self, class_id: Uuid) {
        if let Some(node) = self.find_node_by_class(class_id) {
            let node_id = node.id();
            self.remove_node(node_id);
        }
    }

    pub fn add_relation(
        &mut self,
        from: NodeId,
        to: NodeId,
        kind: RelationKind,
        label: Option<String>,
    ) {
        self.add_relation_with_multiplicities(from, to, kind, label, None, None);
    }

    pub fn add_relation_with_multiplicities(
        &mut self,
        from: NodeId,
        to: NodeId,
        kind: RelationKind,
        label: Option<String>,
        source_multiplicity: Option<Multiplicity>,
        target_multiplicity: Option<Multiplicity>,
    ) {
        if from != to
            && self.find_node(from).is_some()
            && self.find_node(to).is_some()
            && !self
                .edges
                .iter()
                .any(|e| e.from() == from && e.to() == to && e.kind() == kind)
        {
            let mut edge = ClassDiagramEdge::new(from, to, kind, label);
            edge.set_source_multiplicity(source_multiplicity);
            edge.set_target_multiplicity(target_multiplicity);
            self.edges.push(edge);
        }
    }

    pub fn remove_relation(&mut self, from: NodeId, to: NodeId) {
        self.edges.retain(|e| !(e.from() == from && e.to() == to));
    }

    pub fn find_edge(&self, from: NodeId, to: NodeId) -> Option<&ClassDiagramEdge> {
        self.edges.iter().find(|e| e.from() == from && e.to() == to)
    }

    pub fn find_edge_mut(&mut self, from: NodeId, to: NodeId) -> Option<&mut ClassDiagramEdge> {
        self.edges
            .iter_mut()
            .find(|e| e.from() == from && e.to() == to)
    }

    pub fn update_edge_kind(&mut self, from: NodeId, to: NodeId, kind: RelationKind) -> bool {
        if let Some(edge) = self.find_edge_mut(from, to) {
            edge.set_kind(kind);
            true
        } else {
            false
        }
    }

    pub fn update_edge_label(&mut self, from: NodeId, to: NodeId, label: Option<String>) -> bool {
        if let Some(edge) = self.find_edge_mut(from, to) {
            edge.set_label(label);
            true
        } else {
            false
        }
    }

    pub fn update_edge_ports(
        &mut self,
        from: NodeId,
        to: NodeId,
        source_port: Option<PortSide>,
        target_port: Option<PortSide>,
    ) -> bool {
        if let Some(edge) = self.find_edge_mut(from, to) {
            edge.set_ports(source_port, target_port);
            true
        } else {
            false
        }
    }

    pub fn update_edge_directed(&mut self, from: NodeId, to: NodeId, directed: bool) -> bool {
        if let Some(edge) = self.find_edge_mut(from, to) {
            edge.set_directed(directed);
            true
        } else {
            false
        }
    }

    pub fn update_edge_source_multiplicity(
        &mut self,
        from: NodeId,
        to: NodeId,
        mult: Option<Multiplicity>,
    ) -> bool {
        if let Some(edge) = self.find_edge_mut(from, to) {
            edge.set_source_multiplicity(mult);
            true
        } else {
            false
        }
    }

    pub fn update_edge_target_multiplicity(
        &mut self,
        from: NodeId,
        to: NodeId,
        mult: Option<Multiplicity>,
    ) -> bool {
        if let Some(edge) = self.find_edge_mut(from, to) {
            edge.set_target_multiplicity(mult);
            true
        } else {
            false
        }
    }

    pub fn reverse_relation(&mut self, from: NodeId, to: NodeId) -> bool {
        if let Some(pos) = self
            .edges
            .iter()
            .position(|e| e.from() == from && e.to() == to)
        {
            let mut edge = self.edges.remove(pos);
            let old_from = edge.from;
            let old_to = edge.to;
            let old_src_port = edge.source_port;
            let old_tgt_port = edge.target_port;
            let old_src_mult = edge.source_multiplicity;
            let old_tgt_mult = edge.target_multiplicity;

            edge.from = old_to;
            edge.to = old_from;
            edge.source_port = old_tgt_port;
            edge.target_port = old_src_port;
            edge.source_multiplicity = old_tgt_mult;
            edge.target_multiplicity = old_src_mult;

            self.edges.push(edge);
            true
        } else {
            false
        }
    }

    pub fn update_node_position(&mut self, id: NodeId, x: f32, y: f32) {
        if let Some(node) = self.find_node_mut(id) {
            node.set_position(x, y);
        }
    }

    pub fn update_class_dimensions(&mut self, class_id: Uuid, attr_count: usize) {
        if let Some(node) = self.find_node_by_class_mut(class_id) {
            node.update_dimensions(attr_count);
        }
    }

    pub fn sync_with_information_model(&mut self, info_model: &InformationModel) {
        let mut valid_entity_ids: Vec<Uuid> = info_model.classes().iter().map(|c| c.id()).collect();
        valid_entity_ids.extend(info_model.enumerations().iter().map(|e| e.id()));
        valid_entity_ids.extend(info_model.structured_types().iter().map(|st| st.id()));
        self.nodes
            .retain(|n| valid_entity_ids.contains(&n.class_id()));

        // Normaliser eventuelle ældre noder hvor node.id != node.class_id
        for node in &mut self.nodes {
            if node.id != node.class_id {
                let old_id = node.id;
                let new_id = node.class_id;
                node.id = new_id;
                for edge in &mut self.edges {
                    if edge.from == old_id {
                        edge.from = new_id;
                    }
                    if edge.to == old_id {
                        edge.to = new_id;
                    }
                }
            }
        }

        for node in &mut self.nodes {
            if let Some(class) = info_model.get_class(node.class_id()) {
                node.update_dimensions(class.attributes().len());
            } else if let Some(e) = info_model.get_enumeration(node.class_id()) {
                node.update_dimensions(e.values().len());
            } else if let Some(st) = info_model.get_structured_type(node.class_id()) {
                node.update_dimensions(st.attributes().len());
            }
        }

        // Synkroniser kanter fra semantiske relationer i info_model
        let mut expected_edges: Vec<ClassDiagramEdge> = Vec::new();
        for rel in info_model.relations() {
            if let (Some(from_node), Some(to_node)) = (
                self.find_node_by_class(rel.from_class()),
                self.find_node_by_class(rel.to_class()),
            ) {
                let from_id = from_node.id();
                let to_id = to_node.id();
                if let Some(existing) = self.find_edge(from_id, to_id) {
                    let mut updated = existing.clone();
                    updated.set_kind(rel.kind());
                    updated.set_label(rel.label().map(String::from));
                    if rel.source_port().is_some() || rel.target_port().is_some() {
                        updated.set_ports(rel.source_port(), rel.target_port());
                    }
                    if let Some(d) = rel.directed() {
                        updated.set_directed(d);
                    }
                    updated.set_source_multiplicity(rel.source_multiplicity());
                    updated.set_target_multiplicity(rel.target_multiplicity());
                    expected_edges.push(updated);
                } else {
                    let edge = ClassDiagramEdge::with_multiplicities(
                        from_id,
                        to_id,
                        rel.kind(),
                        rel.label().map(String::from),
                        rel.source_port(),
                        rel.target_port(),
                        rel.directed(),
                        rel.source_multiplicity(),
                        rel.target_multiplicity(),
                    );
                    expected_edges.push(edge);
                }
            }
        }

        if info_model.relations().is_empty() && !self.edges.is_empty() {
            let valid_node_ids: std::collections::HashSet<NodeId> =
                self.nodes.iter().map(|n| n.id()).collect();
            self.edges
                .retain(|e| valid_node_ids.contains(&e.from()) && valid_node_ids.contains(&e.to()));
        } else {
            self.edges = expected_edges;
        }
    }

    pub fn migrate_edges_to_model(&self, info_model: &mut InformationModel) {
        if info_model.relations().is_empty() && !self.edges.is_empty() {
            for edge in &self.edges {
                let from_class = self
                    .find_node(edge.from())
                    .map(|n| n.class_id())
                    .unwrap_or(edge.from());
                let to_class = self
                    .find_node(edge.to())
                    .map(|n| n.class_id())
                    .unwrap_or(edge.to());
                let rel = ClassRelation::with_multiplicities(
                    from_class,
                    to_class,
                    edge.kind(),
                    edge.label().map(String::from),
                    edge.source_port(),
                    edge.target_port(),
                    edge.directed(),
                    edge.source_multiplicity(),
                    edge.target_multiplicity(),
                );
                info_model.add_relation(rel);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class_graph_lifecycle_and_cascading_removal() {
        let mut graph = ClassGraph::new();
        let class_1 = Uuid::new_v4();
        let class_2 = Uuid::new_v4();

        let n1 = graph.add_node(class_1, 0);
        let n2 = graph.add_node(class_2, 2);

        assert_eq!(graph.node_count(), 2);
        assert!(graph.is_class_on_diagram(class_1));
        assert!(graph.is_class_on_diagram(class_2));

        // Dimensioner
        let node_1 = graph.find_node(n1).unwrap();
        let node_2 = graph.find_node(n2).unwrap();
        assert_eq!(node_1.height(), MIN_CLASS_NODE_HEIGHT);
        assert!(node_2.height() > node_1.height());

        // Relation
        graph.add_relation(n2, n1, RelationKind::Generalization, None);
        assert_eq!(graph.edge_count(), 1);

        // Kaskadesletning
        graph.remove_class_node(class_1);
        assert_eq!(graph.node_count(), 1);
        assert_eq!(
            graph.edge_count(),
            0,
            "Relationer skal kaskadeslettes uden hængende kanter"
        );
    }

    #[test]
    fn test_class_node_height_snaps_to_grid_increments() {
        for attr_count in 0..10 {
            let h = calculate_class_node_height(attr_count);
            assert_eq!(
                (h % GRID_SIZE).abs(),
                0.0,
                "Højde for {} attributter ({}) skal være et multiplum af GRID_SIZE (20.0)",
                attr_count,
                h
            );
            if attr_count > 0 {
                let prev_h = calculate_class_node_height(attr_count - 1);
                assert!(h >= prev_h, "Højde skal være monotont voksende");
            }
        }
    }
}
