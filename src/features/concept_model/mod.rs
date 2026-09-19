use crate::features::concepts::Concept;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type NodeId = Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationKind {
    Generalization,
    Association,
    Composition,
}

impl RelationKind {
    pub const ALL: &'static [RelationKind] =
        &[Self::Generalization, Self::Association, Self::Composition];
}

impl std::fmt::Display for RelationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Generalization => write!(f, "Generalisering (UML trekant)"),
            Self::Association => write!(f, "Association (UML linje)"),
            Self::Composition => write!(f, "Komposition"),
        }
    }
}

pub const GRID_SIZE: f32 = 20.0;
pub const DEFAULT_NODE_WIDTH: f32 = 180.0;
pub const DEFAULT_NODE_HEIGHT: f32 = 80.0;

fn default_node_width() -> f32 {
    DEFAULT_NODE_WIDTH
}

fn default_node_height() -> f32 {
    DEFAULT_NODE_HEIGHT
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiagramNode {
    id: NodeId,
    concept_id: Uuid,
    label: String,
    #[serde(default = "default_is_local")]
    is_local: bool,
    x: f32,
    y: f32,
    #[serde(default = "default_node_width")]
    width: f32,
    #[serde(default = "default_node_height")]
    height: f32,
}

fn default_is_local() -> bool {
    true
}

impl DiagramNode {
    pub fn new(concept: &Concept, x: f32, y: f32) -> Self {
        Self {
            id: Uuid::new_v4(),
            concept_id: concept.id(),
            label: concept.preferred_term().to_string(),
            is_local: concept.belongs_to_domain().is_local(),
            x,
            y,
            width: DEFAULT_NODE_WIDTH,
            height: DEFAULT_NODE_HEIGHT,
        }
    }

    pub fn custom(id: NodeId, label: String, x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            id,
            concept_id: Uuid::nil(),
            label,
            is_local: true,
            x,
            y,
            width,
            height,
        }
    }

    pub fn id(&self) -> NodeId {
        self.id
    }

    pub fn concept_id(&self) -> Uuid {
        self.concept_id
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn set_label(&mut self, label: String) {
        self.label = label;
    }

    pub fn is_local(&self) -> bool {
        self.is_local
    }

    pub fn set_is_local(&mut self, is_local: bool) {
        self.is_local = is_local;
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
        self.height
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn center(&self) -> (f32, f32) {
        (self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

    pub fn contains(&self, px: f32, py: f32) -> bool {
        px >= self.x && px <= self.x + self.width && py >= self.y && py <= self.y + self.height
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagramEdge {
    from: NodeId,
    to: NodeId,
    kind: RelationKind,
    label: Option<String>,
}

impl DiagramEdge {
    pub fn new(from: NodeId, to: NodeId, kind: RelationKind) -> Self {
        Self {
            from,
            to,
            kind,
            label: None,
        }
    }

    pub fn with_label(from: NodeId, to: NodeId, kind: RelationKind, label: Option<String>) -> Self {
        Self {
            from,
            to,
            kind,
            label,
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

    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    pub fn set_label(&mut self, label: Option<String>) {
        self.label = label;
    }
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConceptGraph {
    nodes: Vec<DiagramNode>,
    edges: Vec<DiagramEdge>,
}

impl ConceptGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, concept: &Concept) -> NodeId {
        let node_count = self.nodes.len() as f32;
        let x = 60.0 + (node_count % 3.0) * 230.0;
        let y = 60.0 + (node_count / 3.0).floor() * 130.0;
        let node = DiagramNode::new(concept, x, y);
        let id = node.id();
        self.nodes.push(node);
        id
    }

    pub fn add_relation(&mut self, from: NodeId, to: NodeId, kind: RelationKind) {
        self.edges.push(DiagramEdge::new(from, to, kind));
    }

    pub fn add_relation_with_label(
        &mut self,
        from: NodeId,
        to: NodeId,
        kind: RelationKind,
        label: Option<String>,
    ) {
        self.edges
            .push(DiagramEdge::with_label(from, to, kind, label));
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn nodes(&self) -> &[DiagramNode] {
        &self.nodes
    }

    pub fn nodes_mut(&mut self) -> &mut Vec<DiagramNode> {
        &mut self.nodes
    }

    pub fn edges(&self) -> &[DiagramEdge] {
        &self.edges
    }

    pub fn edges_mut(&mut self) -> &mut Vec<DiagramEdge> {
        &mut self.edges
    }

    pub fn find_node(&self, id: NodeId) -> Option<&DiagramNode> {
        self.nodes.iter().find(|n| n.id() == id)
    }

    pub fn find_node_mut(&mut self, id: NodeId) -> Option<&mut DiagramNode> {
        self.nodes.iter_mut().find(|n| n.id() == id)
    }

    pub fn find_node_by_concept(&self, concept_id: Uuid) -> Option<&DiagramNode> {
        self.nodes.iter().find(|n| n.concept_id() == concept_id)
    }

    pub fn find_node_by_concept_mut(&mut self, concept_id: Uuid) -> Option<&mut DiagramNode> {
        self.nodes.iter_mut().find(|n| n.concept_id() == concept_id)
    }

    pub fn is_concept_on_diagram(&self, concept_id: Uuid) -> bool {
        self.find_node_by_concept(concept_id).is_some()
    }

    pub fn update_node_position(&mut self, id: NodeId, x: f32, y: f32) {
        if let Some(node) = self.find_node_mut(id) {
            node.set_position(x, y);
        }
    }

    pub fn remove_node(&mut self, id: NodeId) {
        if let Some(idx) = self.nodes.iter().position(|n| n.id() == id) {
            self.nodes.remove(idx);
        }
        // Kaskadesletning af tilknyttede relationer (Fail-Closed mod dangling edges)
        self.edges.retain(|e| e.from != id && e.to != id);
    }

    pub fn remove_node_by_concept(&mut self, concept_id: Uuid) {
        if let Some(node) = self.find_node_by_concept(concept_id) {
            let id = node.id();
            self.remove_node(id);
        }
    }

    pub fn remove_relation(&mut self, from: NodeId, to: NodeId) {
        self.edges.retain(|e| !(e.from == from && e.to == to));
    }

    pub fn sync_with_concepts(&mut self, concepts: &[Concept]) {
        // 1. Fjern noder der ikke længere findes i begrebslisten
        let valid_concept_ids: Vec<Uuid> = concepts.iter().map(|c| c.id()).collect();
        let removed_node_ids: Vec<NodeId> = self
            .nodes
            .iter()
            .filter(|n| !valid_concept_ids.contains(&n.concept_id()))
            .map(|n| n.id())
            .collect();

        for id in removed_node_ids {
            self.remove_node(id);
        }

        // 2. Tilføj nye eller opdater eksisterende
        for concept in concepts {
            if let Some(node) = self.find_node_by_concept_mut(concept.id()) {
                node.set_label(concept.preferred_term().to_string());
                node.set_is_local(concept.belongs_to_domain().is_local());
                node.width = DEFAULT_NODE_WIDTH;
                node.height = DEFAULT_NODE_HEIGHT;
            } else {
                let count = self.nodes.len() as f32;
                let x = 60.0 + (count % 3.0) * 230.0;
                let y = 60.0 + (count / 3.0).floor() * 130.0;
                let mut node = DiagramNode::new(concept, x, y);
                node.set_is_local(concept.belongs_to_domain().is_local());
                self.nodes.push(node);
            }
        }
    }
}
