use crate::features::concept_model::{DiagramEdge, DiagramNode, NodeId, RelationKind};
use iced::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortSide {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrowHead {
    pub tip: Point,
    pub left: Point,
    pub right: Point,
    pub direction: PortSide,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BridgeHop {
    pub center: Point,
    pub is_horizontal: bool,
}

use crate::features::concept_model::NodeId;

#[derive(Debug, Clone, PartialEq)]
pub struct RoutedEdge {
    pub from: NodeId,
    pub to: NodeId,
    pub kind: RelationKind,
    pub label: Option<String>,
    pub label_pos: Option<Point>,
    pub points: Vec<Point>,
    pub arrow_head: Option<ArrowHead>,
    pub bridges: Vec<BridgeHop>,
}

pub struct EdgeRouter;

impl EdgeRouter {
    pub fn route_edges(nodes: &[DiagramNode], edges: &[DiagramEdge]) -> Vec<RoutedEdge> {
        // Minimal initial stub returning empty / direct points to trigger RED tests
        let _ = (nodes, edges);
        Vec::new()
    }
}
