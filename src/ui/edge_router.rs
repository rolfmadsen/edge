use crate::features::concept_model::{DiagramEdge, DiagramNode, NodeId, RelationKind};
use iced::Point;
use std::collections::HashMap;

/// Minimum clearance in pixels needed between node borders to draw direct facing arrows.
pub const MIN_ARROW_CLEARANCE: f32 = 36.0;
/// Length of the UML generalization arrowhead triangle.
pub const ARROW_HEAD_LENGTH: f32 = 14.0;
/// Base width of the UML generalization arrowhead triangle.
pub const ARROW_HEAD_WIDTH: f32 = 14.0;
/// Horizontal/vertical spacing between multiple relation ports on the same node side.
pub const SLOT_SPACING: f32 = 24.0;
/// Channel offset between parallel orthogonal line segments.
pub const CHANNEL_OFFSET: f32 = 14.0;
/// Radius for line jump bridges over intersecting edges.
pub const BRIDGE_RADIUS: f32 = 5.0;

/// The four connection sides/ports of a rectangular diagram node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PortSide {
    Top,
    Right,
    Bottom,
    Left,
}

impl PortSide {
    /// Outward unit normal vector for the port side.
    pub fn normal(self) -> (f32, f32) {
        match self {
            Self::Top => (0.0, -1.0),
            Self::Bottom => (0.0, 1.0),
            Self::Left => (-1.0, 0.0),
            Self::Right => (1.0, 0.0),
        }
    }

    /// Whether this side connects vertically (Top or Bottom).
    pub fn is_vertical(self) -> bool {
        matches!(self, Self::Top | Self::Bottom)
    }
}

/// Represents the geometric triangle of a UML arrowhead touching a node boundary.
#[derive(Debug, Clone, PartialEq)]
pub struct ArrowHead {
    pub tip: Point,
    pub left: Point,
    pub right: Point,
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
    pub bridges: Vec<BridgeHop>,
}

type SideAttachment = (usize, RelationKind, bool);
type SideAttachmentMap = HashMap<(NodeId, PortSide), Vec<SideAttachment>>;

#[derive(Debug, Clone)]
struct EdgePortAssignment {
    edge_idx: usize,
    from_id: NodeId,
    to_id: NodeId,
    kind: RelationKind,
    from_side: PortSide,
    to_side: PortSide,
    from_slot_offset: f32,
    to_slot_offset: f32,
    channel_index: usize,
}

pub struct EdgeRouter;

impl EdgeRouter {
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

            let (from_side, to_side) = Self::select_ports(from, to, edge.kind());

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
        // Samme type deles om ankerpunktet (slot offset 0.0).
        // Forskellige typer fordeles symmetrisk langs siden.
        let mut slot_offsets: HashMap<(usize, bool), f32> = HashMap::new();

        for ((_node_id, _side), attachments) in side_attachments {
            // Find unikke RelationKinds på denne side
            let mut distinct_kinds = Vec::new();
            for &(_, kind, _) in &attachments {
                if !distinct_kinds.contains(&kind) {
                    distinct_kinds.push(kind);
                }
            }

            let kind_count = distinct_kinds.len();
            for (edge_idx, kind, is_source) in attachments {
                let offset = if kind_count <= 1 {
                    0.0
                } else {
                    let k_idx = distinct_kinds.iter().position(|&k| k == kind).unwrap_or(0);
                    let mid = (kind_count as f32 - 1.0) / 2.0;
                    (k_idx as f32 - mid) * SLOT_SPACING
                };
                slot_offsets.insert((edge_idx, is_source), offset);
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

    fn select_ports(
        from: &DiagramNode,
        to: &DiagramNode,
        kind: RelationKind,
    ) -> (PortSide, PortSide) {
        let (from_cx, from_cy) = from.center();
        let (to_cx, to_cy) = to.center();

        let dx = to_cx - from_cx;
        let dy = to_cy - from_cy;

        if kind == RelationKind::Generalization {
            // FDA Princip: Generaliseringspile peger opad mod superklassen
            // superklasse = to, subklasse = from
            let super_bottom = to.y() + to.height();
            let sub_top = from.y();
            let vert_clearance = sub_top - super_bottom;

            if vert_clearance >= MIN_ARROW_CLEARANCE {
                // Normaltilstand: Subklasse er under superklasse med god plads
                (PortSide::Top, PortSide::Bottom)
            } else {
                // Kritisk nærhed eller sideværts forskydning:
                // Hvis subklassen er skubbet til venstre for superklassen
                if from.x() + from.width() <= to.x() + 40.0 {
                    (PortSide::Right, PortSide::Left)
                } else if to.x() + to.width() <= from.x() + 40.0 {
                    (PortSide::Left, PortSide::Right)
                } else {
                    // Direkte over hinanden med < 36px lodret plads:
                    // Skift til side-porte (højre til højre) via en C-løkke, så pilen ikke mastes
                    (PortSide::Right, PortSide::Right)
                }
            }
        } else {
            // Association & Komposition: Vælg modstående porte baseret på relativ retning og clearance
            let horiz_clearance = if to.x() > from.x() + from.width() {
                to.x() - (from.x() + from.width())
            } else if from.x() > to.x() + to.width() {
                from.x() - (to.x() + to.width())
            } else {
                0.0
            };

            let vert_clearance = if to.y() > from.y() + from.height() {
                to.y() - (from.y() + from.height())
            } else if from.y() > to.y() + to.height() {
                from.y() - (to.y() + to.height())
            } else {
                0.0
            };

            if horiz_clearance >= vert_clearance && horiz_clearance >= MIN_ARROW_CLEARANCE {
                if dx > 0.0 {
                    (PortSide::Right, PortSide::Left)
                } else {
                    (PortSide::Left, PortSide::Right)
                }
            } else if vert_clearance >= MIN_ARROW_CLEARANCE {
                if dy > 0.0 {
                    (PortSide::Bottom, PortSide::Top)
                } else {
                    (PortSide::Top, PortSide::Bottom)
                }
            } else {
                // Meget tæt på hinanden: vælg side-porte
                if dx.abs() > dy.abs() {
                    if dx > 0.0 {
                        (PortSide::Right, PortSide::Left)
                    } else {
                        (PortSide::Left, PortSide::Right)
                    }
                } else if dy > 0.0 {
                    (PortSide::Bottom, PortSide::Top)
                } else {
                    (PortSide::Top, PortSide::Bottom)
                }
            }
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

        // Generer 90-graders ortogonale punkter
        let points = Self::build_orthogonal_path(
            start_pt,
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

        RoutedEdge {
            from: from_node.id(),
            to: to_node.id(),
            kind: assign.kind,
            label,
            label_pos,
            points,
            arrow_head,
            bridges: Vec::new(),
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
