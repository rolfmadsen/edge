pub use crate::features::concept_model::PortSide;
use crate::features::concept_model::{DiagramEdge, DiagramNode, NodeId, RelationKind};
use iced::Point;
use std::collections::HashMap;

/// Minimum clearance in pixels needed between node borders to draw direct facing arrows.
pub const MIN_ARROW_CLEARANCE: f32 = 36.0;
/// Length of the UML generalization arrowhead triangle.
pub const ARROW_HEAD_LENGTH: f32 = 14.0;
/// Base width of the UML generalization arrowhead triangle.
pub const ARROW_HEAD_WIDTH: f32 = 14.0;
/// Length of the UML composition diamond from tip to back.
pub const DIAMOND_LENGTH: f32 = 14.0;
/// Width of the UML composition diamond between lateral vertices.
pub const DIAMOND_WIDTH: f32 = 14.0;
/// Horizontal/vertical spacing between multiple relation ports on the same node side.
pub const SLOT_SPACING: f32 = 24.0;
/// Channel offset between parallel orthogonal line segments.
pub const CHANNEL_OFFSET: f32 = 14.0;
/// Radius for line jump bridges over intersecting edges.
pub const BRIDGE_RADIUS: f32 = 5.0;

/// Represents the geometric triangle of a UML arrowhead touching a node boundary.
#[derive(Debug, Clone, PartialEq)]
pub struct ArrowHead {
    pub tip: Point,
    pub left: Point,
    pub right: Point,
    pub direction: PortSide,
}

/// Represents the geometric diamond of a UML composition touching the source node boundary.
#[derive(Debug, Clone, PartialEq)]
pub struct Diamond {
    pub tip: Point,
    pub left: Point,
    pub right: Point,
    pub back: Point,
    pub direction: PortSide,
}

/// Represents a half arrow (single diagonal barb) for UML directed association.
#[derive(Debug, Clone, PartialEq)]
pub struct HalfArrow {
    pub tip: Point,
    pub barb: Point,
    pub direction: PortSide,
}

/// A visual bridge (line jump) rendered where a horizontal segment crosses a vertical segment.
#[derive(Debug, Clone, PartialEq)]
pub struct BridgeHop {
    pub center: Point,
    pub is_horizontal: bool,
}

/// Fully computed 90-degree orthogonal route for a diagram edge.
#[derive(Debug, Clone, PartialEq)]
pub struct RoutedEdge {
    pub from: NodeId,
    pub to: NodeId,
    pub kind: RelationKind,
    pub label: Option<String>,
    pub label_pos: Option<Point>,
    pub points: Vec<Point>,
    pub arrow_head: Option<ArrowHead>,
    pub half_arrow: Option<HalfArrow>,
    pub source_diamond: Option<Diamond>,
    pub bridges: Vec<BridgeHop>,
    pub from_side: PortSide,
    pub to_side: PortSide,
}

type SideAttachment = (usize, RelationKind, bool);
type SideAttachmentMap = HashMap<(NodeId, PortSide), Vec<SideAttachment>>;

#[derive(Debug, Clone)]
pub struct EdgePortAssignment {
    pub edge_idx: usize,
    pub from_id: NodeId,
    pub to_id: NodeId,
    pub kind: RelationKind,
    pub from_side: PortSide,
    pub to_side: PortSide,
    pub from_slot_offset: f32,
    pub to_slot_offset: f32,
    pub channel_index: usize,
}

pub struct EdgeRouter;

impl EdgeRouter {
    pub fn assign_ports(nodes: &[DiagramNode], edges: &[DiagramEdge]) -> Vec<EdgePortAssignment> {
        let node_map: HashMap<NodeId, &DiagramNode> = nodes.iter().map(|n| (n.id(), n)).collect();
        Self::compute_port_assignments(&node_map, edges)
    }

    pub fn route_edges(nodes: &[DiagramNode], edges: &[DiagramEdge]) -> Vec<RoutedEdge> {
        let node_map: HashMap<NodeId, &DiagramNode> = nodes.iter().map(|n| (n.id(), n)).collect();

        // 1. Vælg porte og tildel slots til hver edge
        let assignments = Self::compute_port_assignments(&node_map, edges);

        // 2. Generer 90-graders ortogonale stier for hver edge
        let mut routes: Vec<RoutedEdge> = assignments
            .iter()
            .filter_map(|assign| {
                let from_node = node_map.get(&assign.from_id)?;
                let to_node = node_map.get(&assign.to_id)?;
                let edge = &edges[assign.edge_idx];

                Some(Self::generate_route(from_node, to_node, edge, assign))
            })
            .collect();

        // 3. Detekter linjekrydsninger og tilføj broer (bridge hops)
        Self::detect_bridges(&mut routes);

        routes
    }

    fn compute_port_assignments(
        node_map: &HashMap<NodeId, &DiagramNode>,
        edges: &[DiagramEdge],
    ) -> Vec<EdgePortAssignment> {
        let mut initial_assignments = Vec::new();

        // Kortlæg hvilke relationer der rammer hvilke sider på hver node
        let mut side_attachments: SideAttachmentMap = HashMap::new();

        for (idx, edge) in edges.iter().enumerate() {
            let (Some(from), Some(to)) = (node_map.get(&edge.from()), node_map.get(&edge.to()))
            else {
                continue;
            };

            let (from_side, to_side) = Self::select_ports(
                from,
                to,
                edge.kind(),
                edge.source_port(),
                edge.target_port(),
            );

            side_attachments
                .entry((from.id(), from_side))
                .or_default()
                .push((idx, edge.kind(), true));

            side_attachments
                .entry((to.id(), to_side))
                .or_default()
                .push((idx, edge.kind(), false));

            initial_assignments.push((
                idx,
                edge.from(),
                edge.to(),
                edge.kind(),
                from_side,
                to_side,
            ));
        }

        // Beregn slot offsets for hver side:
        // Hvis alle tilknytninger er indgående generaliseringer til denne node, deles de om ankerpunktet (FDA Fig 7.1).
        // Ellers sorteres tilknytningerne rumligt (spatial sorting) langs nodens kant, så parallelle relationer
        // altid forsynes med porte i naturlig rækkefølge uden krydsninger.
        let mut slot_offsets: HashMap<(usize, bool), f32> = HashMap::new();

        for ((_node_id, side), attachments) in side_attachments {
            let all_incoming_gen = !attachments.is_empty()
                && attachments
                    .iter()
                    .all(|a| a.1 == RelationKind::Generalization && !a.2);

            if all_incoming_gen {
                for (edge_idx, _, is_source) in attachments {
                    slot_offsets.insert((edge_idx, is_source), 0.0);
                }
            } else {
                let mut sorted_attachments = attachments;
                sorted_attachments.sort_by(|a, b| {
                    let other_id_a = if a.2 {
                        edges[a.0].to()
                    } else {
                        edges[a.0].from()
                    };
                    let other_id_b = if b.2 {
                        edges[b.0].to()
                    } else {
                        edges[b.0].from()
                    };

                    let coord_a = node_map
                        .get(&other_id_a)
                        .map(|n| {
                            if side.is_vertical() {
                                n.center().0
                            } else {
                                n.center().1
                            }
                        })
                        .unwrap_or(0.0);
                    let coord_b = node_map
                        .get(&other_id_b)
                        .map(|n| {
                            if side.is_vertical() {
                                n.center().0
                            } else {
                                n.center().1
                            }
                        })
                        .unwrap_or(0.0);

                    coord_a
                        .partial_cmp(&coord_b)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });

                let count = sorted_attachments.len();
                if count <= 1 {
                    if let Some(&(edge_idx, _, is_source)) = sorted_attachments.first() {
                        slot_offsets.insert((edge_idx, is_source), 0.0);
                    }
                } else {
                    let mid = (count as f32 - 1.0) / 2.0;
                    for (i, &(edge_idx, _, is_source)) in sorted_attachments.iter().enumerate() {
                        let offset = (i as f32 - mid) * SLOT_SPACING;
                        slot_offsets.insert((edge_idx, is_source), offset);
                    }
                }
            }
        }

        initial_assignments
            .into_iter()
            .enumerate()
            .map(
                |(channel_idx, (edge_idx, from_id, to_id, kind, from_side, to_side))| {
                    let from_slot_offset =
                        slot_offsets.get(&(edge_idx, true)).copied().unwrap_or(0.0);
                    let to_slot_offset =
                        slot_offsets.get(&(edge_idx, false)).copied().unwrap_or(0.0);

                    EdgePortAssignment {
                        edge_idx,
                        from_id,
                        to_id,
                        kind,
                        from_side,
                        to_side,
                        from_slot_offset,
                        to_slot_offset,
                        channel_index: channel_idx,
                    }
                },
            )
            .collect()
    }

    pub fn select_ports(
        from: &DiagramNode,
        to: &DiagramNode,
        kind: RelationKind,
        current_source_port: Option<PortSide>,
        current_target_port: Option<PortSide>,
    ) -> (PortSide, PortSide) {
        // 1. Evaluer source port (from_side) med hysterese hvis port allerede er sat
        let from_side = if let Some(saved_from) = current_source_port {
            Self::evaluate_source_hysteresis(from, to, saved_from)
        } else {
            Self::select_initial_source_port(from, to, kind)
        };

        // 2. Evaluer target port (to_side) med hysterese hvis port allerede er sat, ellers matching
        let to_side = if let Some(saved_to) = current_target_port {
            Self::evaluate_target_hysteresis(to, from, saved_to, from_side)
        } else {
            Self::select_matching_target_port(from, to, kind, from_side)
        };

        (from_side, to_side)
    }

    fn evaluate_source_hysteresis(
        from: &DiagramNode,
        to: &DiagramNode,
        current_port: PortSide,
    ) -> PortSide {
        let from_right = from.x() + from.width();
        let from_bottom = from.y() + from.height();
        let from_left = from.x();
        let from_top = from.y();

        match current_port {
            PortSide::Right => {
                // Forbliver Right så længe to er til højre for kildens højre kant
                if to.x() >= from_right - 5.0 {
                    PortSide::Right
                } else {
                    // Krydset over den vertikale grænselinje mod venstre
                    if to.center().1 > from.center().1 {
                        PortSide::Bottom
                    } else if to.center().1 < from.center().1 {
                        PortSide::Top
                    } else if to.x() + to.width() <= from_left {
                        PortSide::Left
                    } else {
                        PortSide::Right
                    }
                }
            }
            PortSide::Bottom => {
                // Forbliver Bottom så længe to er under kildens bundkant
                if to.y() >= from_bottom - 5.0 {
                    PortSide::Bottom
                } else {
                    // Krydset op over den horisontale grænselinje
                    if to.center().0 > from.center().0 {
                        PortSide::Right
                    } else if to.center().0 < from.center().0 {
                        PortSide::Left
                    } else if to.y() + to.height() <= from_top {
                        PortSide::Top
                    } else {
                        PortSide::Bottom
                    }
                }
            }
            PortSide::Left => {
                // Forbliver Left så længe to er til venstre for kildens venstre kant
                if to.x() + to.width() <= from_left + 5.0 {
                    PortSide::Left
                } else {
                    // Krydset mod højre over den venstre grænselinje
                    if to.center().1 > from.center().1 {
                        PortSide::Bottom
                    } else if to.center().1 < from.center().1 {
                        PortSide::Top
                    } else if to.x() >= from_right {
                        PortSide::Right
                    } else {
                        PortSide::Left
                    }
                }
            }
            PortSide::Top => {
                // Forbliver Top så længe to er over kildens topkant
                if to.y() + to.height() <= from_top + 5.0 {
                    PortSide::Top
                } else {
                    // Krydset ned over den øverste grænselinje
                    if to.center().0 > from.center().0 {
                        PortSide::Right
                    } else if to.center().0 < from.center().0 {
                        PortSide::Left
                    } else if to.y() >= from_bottom {
                        PortSide::Bottom
                    } else {
                        PortSide::Top
                    }
                }
            }
        }
    }

    fn evaluate_target_hysteresis(
        to: &DiagramNode,
        from: &DiagramNode,
        current_to_port: PortSide,
        from_side: PortSide,
    ) -> PortSide {
        let to_right = to.x() + to.width();
        let to_bottom = to.y() + to.height();
        let to_left = to.x();
        let to_top = to.y();

        let valid = match current_to_port {
            PortSide::Right => from.x() >= to_right - 5.0,
            PortSide::Bottom => from.y() >= to_bottom - 5.0,
            PortSide::Left => from.x() + from.width() <= to_left + 5.0,
            PortSide::Top => from.y() + from.height() <= to_top + 5.0,
        };

        if valid {
            current_to_port
        } else {
            Self::select_matching_target_port(from, to, RelationKind::Association, from_side)
        }
    }

    fn select_initial_source_port(
        from: &DiagramNode,
        to: &DiagramNode,
        kind: RelationKind,
    ) -> PortSide {
        let from_right = from.x() + from.width();
        let from_bottom = from.y() + from.height();
        let from_left = from.x();
        let from_top = from.y();

        let to_right = to.x() + to.width();
        let to_bottom = to.y() + to.height();
        let to_left = to.x();
        let to_top = to.y();

        if kind == RelationKind::Generalization {
            // Generalisering: Subklasse er from, Superklasse er to
            let super_bottom = to.y() + to.height();
            let sub_top = from.y();
            let vert_clearance = sub_top - super_bottom;

            // Hvis subklassen er under superklassen med tilstrækkelig frihøjde
            if vert_clearance >= MIN_ARROW_CLEARANCE {
                PortSide::Top
            } else if from.y() + from.height() <= to.y() - MIN_ARROW_CLEARANCE {
                PortSide::Bottom
            } else if from.x() >= to_right {
                PortSide::Left
            } else if from_right <= to_left {
                PortSide::Right
            } else {
                // Kritisk nærhed (< 36px) eller overlap: brug side-port så pilen ikke mastes
                PortSide::Right
            }
        } else {
            // Association & Komposition:
            let horiz_clearance = if to_left > from_right {
                to_left - from_right
            } else if from_left > to_right {
                from_left - to_right
            } else {
                0.0
            };

            let vert_clearance = if to_top > from_bottom {
                to_top - from_bottom
            } else if from_top > to_bottom {
                from_top - to_bottom
            } else {
                0.0
            };

            let dx = to.center().0 - from.center().0;
            let dy = to.center().1 - from.center().1;

            if to_left >= from_right && horiz_clearance >= vert_clearance {
                PortSide::Right
            } else if to_right <= from_left && horiz_clearance >= vert_clearance {
                PortSide::Left
            } else if to_top >= from_bottom {
                PortSide::Bottom
            } else if to_bottom <= from_top {
                PortSide::Top
            } else if dx.abs() > dy.abs() {
                if dx > 0.0 {
                    PortSide::Right
                } else {
                    PortSide::Left
                }
            } else if dy > 0.0 {
                PortSide::Bottom
            } else {
                PortSide::Top
            }
        }
    }

    fn select_matching_target_port(
        from: &DiagramNode,
        to: &DiagramNode,
        kind: RelationKind,
        from_side: PortSide,
    ) -> PortSide {
        if kind == RelationKind::Generalization {
            let super_bottom = to.y() + to.height();
            let sub_top = from.y();
            let vert_clearance = sub_top - super_bottom;

            if from_side == PortSide::Right
                && from.x() < to.x() + to.width()
                && vert_clearance < MIN_ARROW_CLEARANCE
            {
                // Kritisk nærhed: side-porte (højre til højre)
                PortSide::Right
            } else {
                match from_side {
                    PortSide::Left => PortSide::Right,
                    PortSide::Right => PortSide::Left,
                    PortSide::Top => PortSide::Bottom,
                    PortSide::Bottom => PortSide::Top,
                }
            }
        } else {
            from_side.opposite()
        }
    }

    fn port_point(node: &DiagramNode, side: PortSide, slot_offset: f32) -> Point {
        let (cx, cy) = node.center();
        match side {
            PortSide::Top => Point::new(cx + slot_offset, node.y()),
            PortSide::Bottom => Point::new(cx + slot_offset, node.y() + node.height()),
            PortSide::Left => Point::new(node.x(), cy + slot_offset),
            PortSide::Right => Point::new(node.x() + node.width(), cy + slot_offset),
        }
    }

    fn generate_route(
        from_node: &DiagramNode,
        to_node: &DiagramNode,
        edge: &DiagramEdge,
        assign: &EdgePortAssignment,
    ) -> RoutedEdge {
        let start_pt = Self::port_point(from_node, assign.from_side, assign.from_slot_offset);
        let end_pt = Self::port_point(to_node, assign.to_side, assign.to_slot_offset);

        // Beregn pilehoved
        let arrow_head = if assign.kind == RelationKind::Generalization {
            Some(Self::compute_arrow_head(end_pt, assign.to_side))
        } else {
            None
        };

        // Rute-endepunkt: hvis der er pilehoved, stopper linjen ved pilens base
        let line_end_pt = if arrow_head.is_some() {
            match assign.to_side {
                PortSide::Bottom => Point::new(end_pt.x, end_pt.y + ARROW_HEAD_LENGTH),
                PortSide::Top => Point::new(end_pt.x, end_pt.y - ARROW_HEAD_LENGTH),
                PortSide::Left => Point::new(end_pt.x - ARROW_HEAD_LENGTH, end_pt.y),
                PortSide::Right => Point::new(end_pt.x + ARROW_HEAD_LENGTH, end_pt.y),
            }
        } else {
            end_pt
        };

        // Beregn kildemarkør (f.eks. sort diamant for komposition)
        let source_diamond = if assign.kind == RelationKind::Composition {
            Some(Self::compute_diamond(start_pt, assign.from_side))
        } else {
            None
        };

        // Rute-startpunkt: hvis der er kildediamant, starter linjen ved diamantens bagkant
        let line_start_pt = if source_diamond.is_some() {
            match assign.from_side {
                PortSide::Bottom => Point::new(start_pt.x, start_pt.y + DIAMOND_LENGTH),
                PortSide::Top => Point::new(start_pt.x, start_pt.y - DIAMOND_LENGTH),
                PortSide::Left => Point::new(start_pt.x - DIAMOND_LENGTH, start_pt.y),
                PortSide::Right => Point::new(start_pt.x + DIAMOND_LENGTH, start_pt.y),
            }
        } else {
            start_pt
        };

        // Generer 90-graders ortogonale punkter
        let points = Self::build_orthogonal_path(
            line_start_pt,
            line_end_pt,
            assign.from_side,
            assign.to_side,
            assign.channel_index,
        );

        // FDA label-semantik: Generalisering har ingen tekst-label
        let label = if assign.kind == RelationKind::Generalization {
            None
        } else {
            edge.label().map(|s| s.to_string())
        };

        // Beregn label position midt på det længste segment
        let label_pos = label
            .as_ref()
            .and_then(|_| Self::compute_label_pos(&points));

        // Beregn halv pil for rettet association
        let half_arrow = if assign.kind == RelationKind::Association && edge.is_directed() {
            Some(Self::compute_half_arrow(end_pt, assign.to_side))
        } else {
            None
        };

        RoutedEdge {
            from: from_node.id(),
            to: to_node.id(),
            kind: assign.kind,
            label,
            label_pos,
            points,
            arrow_head,
            half_arrow,
            source_diamond,
            bridges: Vec::new(),
            from_side: assign.from_side,
            to_side: assign.to_side,
        }
    }

    fn compute_half_arrow(target_boundary: Point, side: PortSide) -> HalfArrow {
        let len = 10.0;
        let w = 6.0;

        let barb = match side {
            PortSide::Left => Point::new(target_boundary.x - len, target_boundary.y - w),
            PortSide::Right => Point::new(target_boundary.x + len, target_boundary.y - w),
            PortSide::Top => Point::new(target_boundary.x - w, target_boundary.y - len),
            PortSide::Bottom => Point::new(target_boundary.x - w, target_boundary.y + len),
        };

        HalfArrow {
            tip: target_boundary,
            barb,
            direction: side,
        }
    }

    fn compute_diamond(source_boundary: Point, side: PortSide) -> Diamond {
        let half_w = DIAMOND_WIDTH / 2.0;
        let len = DIAMOND_LENGTH;
        let half_len = len / 2.0;

        match side {
            PortSide::Right => Diamond {
                tip: source_boundary,
                left: Point::new(source_boundary.x + half_len, source_boundary.y - half_w),
                right: Point::new(source_boundary.x + half_len, source_boundary.y + half_w),
                back: Point::new(source_boundary.x + len, source_boundary.y),
                direction: PortSide::Right,
            },
            PortSide::Left => Diamond {
                tip: source_boundary,
                left: Point::new(source_boundary.x - half_len, source_boundary.y - half_w),
                right: Point::new(source_boundary.x - half_len, source_boundary.y + half_w),
                back: Point::new(source_boundary.x - len, source_boundary.y),
                direction: PortSide::Left,
            },
            PortSide::Top => Diamond {
                tip: source_boundary,
                left: Point::new(source_boundary.x - half_w, source_boundary.y - half_len),
                right: Point::new(source_boundary.x + half_w, source_boundary.y - half_len),
                back: Point::new(source_boundary.x, source_boundary.y - len),
                direction: PortSide::Top,
            },
            PortSide::Bottom => Diamond {
                tip: source_boundary,
                left: Point::new(source_boundary.x - half_w, source_boundary.y + half_len),
                right: Point::new(source_boundary.x + half_w, source_boundary.y + half_len),
                back: Point::new(source_boundary.x, source_boundary.y + len),
                direction: PortSide::Bottom,
            },
        }
    }

    fn compute_arrow_head(target_boundary: Point, side: PortSide) -> ArrowHead {
        let half_w = ARROW_HEAD_WIDTH / 2.0;
        let len = ARROW_HEAD_LENGTH;

        match side {
            PortSide::Bottom => ArrowHead {
                tip: target_boundary,
                left: Point::new(target_boundary.x - half_w, target_boundary.y + len),
                right: Point::new(target_boundary.x + half_w, target_boundary.y + len),
                direction: PortSide::Bottom,
            },
            PortSide::Top => ArrowHead {
                tip: target_boundary,
                left: Point::new(target_boundary.x - half_w, target_boundary.y - len),
                right: Point::new(target_boundary.x + half_w, target_boundary.y - len),
                direction: PortSide::Top,
            },
            PortSide::Left => ArrowHead {
                tip: target_boundary,
                left: Point::new(target_boundary.x - len, target_boundary.y - half_w),
                right: Point::new(target_boundary.x - len, target_boundary.y + half_w),
                direction: PortSide::Left,
            },
            PortSide::Right => ArrowHead {
                tip: target_boundary,
                left: Point::new(target_boundary.x + len, target_boundary.y - half_w),
                right: Point::new(target_boundary.x + len, target_boundary.y + half_w),
                direction: PortSide::Right,
            },
        }
    }

    fn build_orthogonal_path(
        start: Point,
        end: Point,
        start_side: PortSide,
        end_side: PortSide,
        channel_index: usize,
    ) -> Vec<Point> {
        let channel_offset = (channel_index % 3) as f32 * CHANNEL_OFFSET;
        let mut pts = vec![start];

        let stub_len = 16.0 + channel_offset;

        // Vinkelret stub ud fra start
        let (snx, sny) = start_side.normal();
        let start_stub = Point::new(start.x + snx * stub_len, start.y + sny * stub_len);

        // Vinkelret stub ud fra end
        let (enx, eny) = end_side.normal();
        let end_stub = Point::new(end.x + enx * stub_len, end.y + eny * stub_len);

        if (start_side == PortSide::Top && end_side == PortSide::Bottom)
            || (start_side == PortSide::Bottom && end_side == PortSide::Top)
        {
            let mid_y = (start_stub.y + end_stub.y) / 2.0;
            pts.push(Point::new(start.x, mid_y));
            pts.push(Point::new(end.x, mid_y));
        } else if (start_side == PortSide::Right && end_side == PortSide::Left)
            || (start_side == PortSide::Left && end_side == PortSide::Right)
        {
            let mid_x = (start_stub.x + end_stub.x) / 2.0;
            pts.push(Point::new(mid_x, start.y));
            pts.push(Point::new(mid_x, end.y));
        } else if start_side == PortSide::Right && end_side == PortSide::Right {
            // C-bøjning på højre side
            let max_x = start_stub.x.max(end_stub.x) + 20.0 + channel_offset;
            pts.push(Point::new(max_x, start.y));
            pts.push(Point::new(max_x, end.y));
        } else if start_side == PortSide::Left && end_side == PortSide::Left {
            // C-bøjning på venstre side
            let min_x = start_stub.x.min(end_stub.x) - 20.0 - channel_offset;
            pts.push(Point::new(min_x, start.y));
            pts.push(Point::new(min_x, end.y));
        } else if start_side.is_vertical() && !end_side.is_vertical() {
            // Start vertikal, end horisontal
            pts.push(Point::new(start.x, end.y));
        } else if !start_side.is_vertical() && end_side.is_vertical() {
            // Start horisontal, end vertikal
            pts.push(Point::new(end.x, start.y));
        } else {
            // Generelt fallback med 2 hjørner via stubs
            pts.push(start_stub);
            pts.push(Point::new(end_stub.x, start_stub.y));
            pts.push(end_stub);
        }

        pts.push(end);
        Self::simplify_path(pts)
    }

    fn simplify_path(points: Vec<Point>) -> Vec<Point> {
        if points.len() <= 2 {
            return points;
        }

        let mut simplified = Vec::new();
        simplified.push(points[0]);

        for i in 1..points.len() - 1 {
            let prev = *simplified.last().unwrap();
            let curr = points[i];
            let next = points[i + 1];

            // Hvis curr ligger på den samme rette linje som prev og next, spring over curr
            let collinear_h = (prev.y - curr.y).abs() < 0.001 && (curr.y - next.y).abs() < 0.001;
            let collinear_v = (prev.x - curr.x).abs() < 0.001 && (curr.x - next.x).abs() < 0.001;

            if !collinear_h && !collinear_v {
                simplified.push(curr);
            }
        }

        simplified.push(*points.last().unwrap());
        simplified
    }

    fn compute_label_pos(points: &[Point]) -> Option<Point> {
        if points.len() < 2 {
            return None;
        }

        // Find det længste segment
        let mut best_len = 0.0;
        let mut best_pos = None;

        for window in points.windows(2) {
            let p1 = window[0];
            let p2 = window[1];
            let dx = p2.x - p1.x;
            let dy = p2.y - p1.y;
            let len = (dx * dx + dy * dy).sqrt();

            if len > best_len {
                best_len = len;
                let mid_x = (p1.x + p2.x) / 2.0;
                let mid_y = (p1.y + p2.y) / 2.0;
                // Lidt offset for overskuelighed
                let offset_y = if (p1.y - p2.y).abs() < 0.001 {
                    -10.0
                } else {
                    0.0
                };
                best_pos = Some(Point::new(mid_x, mid_y + offset_y));
            }
        }

        best_pos
    }

    fn detect_bridges(routes: &mut [RoutedEdge]) {
        let n = routes.len();
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }

                // Tjek skæringer mellem horisontale segmenter i rute i og vertikale segmenter i rute j
                let mut bridges = Vec::new();
                for w_h in routes[i].points.windows(2) {
                    let (p1, p2) = (w_h[0], w_h[1]);
                    if (p1.y - p2.y).abs() > 0.001 {
                        continue; // ikke horisontal
                    }

                    let h_y = p1.y;
                    let h_min_x = p1.x.min(p2.x);
                    let h_max_x = p1.x.max(p2.x);

                    for w_v in routes[j].points.windows(2) {
                        let (q1, q2) = (w_v[0], w_v[1]);
                        if (q1.x - q2.x).abs() > 0.001 {
                            continue; // ikke vertikal
                        }

                        let v_x = q1.x;
                        let v_min_y = q1.y.min(q2.y);
                        let v_max_y = q1.y.max(q2.y);

                        // Skærer de hinanden strengt indeni segmenterne?
                        if v_x > h_min_x + 2.0
                            && v_x < h_max_x - 2.0
                            && h_y > v_min_y + 2.0
                            && h_y < v_max_y - 2.0
                        {
                            bridges.push(BridgeHop {
                                center: Point::new(v_x, h_y),
                                is_horizontal: true,
                            });
                        }
                    }
                }

                routes[i].bridges.extend(bridges);
            }
        }
    }
}
