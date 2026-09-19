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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiagramNode {
    id: NodeId,
    concept_id: Uuid,
    label: String,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl DiagramNode {
    pub fn new(concept: &Concept, x: f32, y: f32) -> Self {
        Self {
            id: Uuid::new_v4(),
            concept_id: concept.id(),
            label: concept.preferred_term().to_string(),
            x,
            y,
            width: 160.0,
            height: 80.0,
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

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
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
        let x = 60.0 + (node_count % 4.0) * 200.0;
        let y = 60.0 + (node_count / 4.0).floor() * 120.0;
        let node = DiagramNode::new(concept, x, y);
        let id = node.id();
        self.nodes.push(node);
        id
    }

    pub fn add_relation(&mut self, from: NodeId, to: NodeId, kind: RelationKind) {
        self.edges.push(DiagramEdge::new(from, to, kind));
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

    pub fn edges(&self) -> &[DiagramEdge] {
        &self.edges
    }
}
