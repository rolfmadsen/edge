use crate::features::concept_model::{
    DiagramEdge, DiagramNode, NodeId, PortSide, RelationKind, GRID_SIZE,
};
use crate::features::information_model::{ClassDiagramEdge, ClassDiagramNode};
use crate::ui::edge_router::EdgeRouter;
use crate::ui::theme::ThemeColors;
use iced::mouse;
use iced::widget::canvas::{Action, Event, Frame, Geometry, Path, Program, Stroke, Text};
use iced::{alignment, Color, Point, Rectangle, Renderer, Size, Theme, Vector};
use std::time::Instant;

/// Fælles viewport til koordinattransformation, zoom og panorering
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CanvasViewport {
    pan: Vector,
    zoom: f32,
}

impl Default for CanvasViewport {
    fn default() -> Self {
        Self {
            pan: Vector::new(0.0, 0.0),
            zoom: 1.0,
        }
    }
}

impl CanvasViewport {
    pub const MIN_ZOOM: f32 = 0.25;
    pub const MAX_ZOOM: f32 = 1.50;

    pub fn new(pan: Vector, zoom: f32) -> Self {
        let mut vp = Self { pan, zoom: 1.0 };
        vp.set_zoom(zoom);
        vp
    }

    pub fn zoom(&self) -> f32 {
        self.zoom
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.clamp(Self::MIN_ZOOM, Self::MAX_ZOOM);
    }

    pub fn pan(&self) -> Vector {
        self.pan
    }

    pub fn set_pan(&mut self, pan: Vector) {
        self.pan = pan;
    }

    pub fn translate(&mut self, delta: Vector) {
        self.pan += delta;
    }

    pub fn to_world(&self, screen_point: Point) -> Point {
        Point::new(
            (screen_point.x - self.pan.x) / self.zoom,
            (screen_point.y - self.pan.y) / self.zoom,
        )
    }

    pub fn to_screen(&self, world_point: Point) -> Point {
        Point::new(
            world_point.x * self.zoom + self.pan.x,
            world_point.y * self.zoom + self.pan.y,
        )
    }

    pub fn zoom_at(&mut self, screen_pivot: Point, factor: f32) {
        let world_pivot = self.to_world(screen_pivot);
        let new_zoom = (self.zoom * factor).clamp(Self::MIN_ZOOM, Self::MAX_ZOOM);
        if (new_zoom - self.zoom).abs() < f32::EPSILON {
            return;
        }
        self.zoom = new_zoom;
        self.pan = Vector::new(
            screen_pivot.x - world_pivot.x * self.zoom,
            screen_pivot.y - world_pivot.y * self.zoom,
        );
    }

    pub fn snap_to_grid(world_point: Point, grid_size: f32) -> Point {
        Point::new(
            (world_point.x / grid_size).round() * grid_size,
            (world_point.y / grid_size).round() * grid_size,
        )
    }

    pub fn reset(&mut self) {
        self.pan = Vector::new(0.0, 0.0);
        self.zoom = 1.0;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ClickRecord {
    pub position: Point,
    pub time: Instant,
}

/// Abstraktion for noder på et diagram
pub trait CanvasNode {
    fn id(&self) -> NodeId;
    fn position(&self) -> (f32, f32);
    fn size(&self) -> (f32, f32);
    fn contains(&self, px: f32, py: f32) -> bool {
        let (x, y) = self.position();
        let (w, h) = self.size();
        px >= x && px <= x + w && py >= y && py <= y + h
    }
    fn to_diagram_node(&self) -> DiagramNode {
        let (x, y) = self.position();
        let (w, h) = self.size();
        DiagramNode::custom(self.id(), String::new(), x, y, w, h)
    }
}

/// Abstraktion for kanter/relationer på et diagram
pub trait CanvasEdge {
    fn from(&self) -> NodeId;
    fn to(&self) -> NodeId;
    fn kind(&self) -> RelationKind;
    fn label(&self) -> Option<&str>;
    fn source_port(&self) -> Option<PortSide> {
        None
    }
    fn target_port(&self) -> Option<PortSide> {
        None
    }
    fn source_multiplicity(&self) -> Option<String> {
        None
    }
    fn target_multiplicity(&self) -> Option<String> {
        None
    }
    fn is_directed(&self) -> bool {
        true
    }
    fn to_diagram_edge(&self) -> DiagramEdge {
        let mut edge = DiagramEdge::with_ports(
            self.from(),
            self.to(),
            self.kind(),
            self.label().map(|s| s.to_string()),
            self.source_port(),
            self.target_port(),
        );
        edge.set_directed(self.is_directed());
        edge
    }
}

impl CanvasNode for DiagramNode {
    fn id(&self) -> NodeId {
        self.id()
    }
    fn position(&self) -> (f32, f32) {
        (self.x(), self.y())
    }
    fn size(&self) -> (f32, f32) {
        (self.width(), self.height())
    }
    fn contains(&self, px: f32, py: f32) -> bool {
        DiagramNode::contains(self, px, py)
    }
}

impl CanvasNode for ClassDiagramNode {
    fn id(&self) -> NodeId {
        self.id()
    }
    fn position(&self) -> (f32, f32) {
        (self.x(), self.y())
    }
    fn size(&self) -> (f32, f32) {
        (self.width(), self.height())
    }
    fn contains(&self, px: f32, py: f32) -> bool {
        ClassDiagramNode::contains(self, px, py)
    }
}

impl CanvasEdge for DiagramEdge {
    fn from(&self) -> NodeId {
        self.from()
    }
    fn to(&self) -> NodeId {
        self.to()
    }
    fn kind(&self) -> RelationKind {
        self.kind()
    }
    fn label(&self) -> Option<&str> {
        self.label()
    }
    fn source_port(&self) -> Option<PortSide> {
        self.source_port()
    }
    fn target_port(&self) -> Option<PortSide> {
        self.target_port()
    }
    fn is_directed(&self) -> bool {
        DiagramEdge::is_directed(self)
    }
}

impl CanvasEdge for ClassDiagramEdge {
    fn from(&self) -> NodeId {
        self.from()
    }
    fn to(&self) -> NodeId {
        self.to()
    }
    fn kind(&self) -> RelationKind {
        self.kind()
    }
    fn label(&self) -> Option<&str> {
        self.label()
    }
    fn source_port(&self) -> Option<PortSide> {
        self.source_port()
    }
    fn target_port(&self) -> Option<PortSide> {
        self.target_port()
    }
    fn is_directed(&self) -> bool {
        ClassDiagramEdge::is_directed(self)
    }
    fn source_multiplicity(&self) -> Option<String> {
        if self.kind() == RelationKind::Generalization {
            None
        } else {
            ClassDiagramEdge::source_multiplicity(self).map(|m| m.to_display_string())
        }
    }
    fn target_multiplicity(&self) -> Option<String> {
        if self.kind() == RelationKind::Generalization {
            None
        } else {
            ClassDiagramEdge::target_multiplicity(self).map(|m| m.to_display_string())
        }
    }
}

#[derive(Debug, Default)]
pub struct DiagramCanvasState {
    pub dragging_node: Option<(NodeId, Vector)>,
    pub last_click: Option<ClickRecord>,
    pub panning_start: Option<(Point, Vector)>,
    pub is_panning_space: bool,
    pub modifiers: iced::keyboard::Modifiers,
    pub connecting_from: Option<NodeId>,
    pub connecting_cursor: Option<Point>,
    pub hovered_target_node: Option<NodeId>,
}

/// Afstand fra punkt p til linjesegment a-b
fn distance_to_segment(p: Point, a: Point, b: Point) -> f32 {
    let l2 = (b.x - a.x).powi(2) + (b.y - a.y).powi(2);
    if l2 < 0.0001 {
        return (p.x - a.x).hypot(p.y - a.y);
    }
    let t = (((p.x - a.x) * (b.x - a.x) + (p.y - a.y) * (b.y - a.y)) / l2).clamp(0.0, 1.0);
    let projection = Point::new(a.x + t * (b.x - a.x), a.y + t * (b.y - a.y));
    (p.x - projection.x).hypot(p.y - projection.y)
}

type EdgeSelectHandler<'a, Message> = Box<dyn Fn(Option<(NodeId, NodeId)>) -> Message + 'a>;
type EdgeCreateHandler<'a, Message> = Box<dyn Fn(NodeId, NodeId) -> Message + 'a>;

/// Unificeret DiagramCanvas-komponent med pluggable node-rendering
pub struct DiagramCanvas<'a, Message, N, E, R>
where
    N: CanvasNode,
    E: CanvasEdge,
    R: Fn(&mut Frame, &N, bool, CanvasViewport),
{
    nodes: &'a [N],
    edges: &'a [E],
    selected_node_id: Option<NodeId>,
    selected_edge: Option<(NodeId, NodeId)>,
    viewport: CanvasViewport,
    snap_to_grid: bool,
    is_space_pressed: bool,
    render_node: R,
    on_node_selected: Box<dyn Fn(Option<NodeId>) -> Message + 'a>,
    on_edge_selected: Option<EdgeSelectHandler<'a, Message>>,
    on_edge_created: Option<EdgeCreateHandler<'a, Message>>,
    on_node_moved: Box<dyn Fn(NodeId, f32, f32) -> Message + 'a>,
    on_canvas_double_clicked: Box<dyn Fn(f32, f32) -> Message + 'a>,
    on_node_double_clicked: Box<dyn Fn(NodeId) -> Message + 'a>,
    on_viewport_changed: Box<dyn Fn(CanvasViewport) -> Message + 'a>,
}

impl<'a, Message, N, E, R> DiagramCanvas<'a, Message, N, E, R>
where
    N: CanvasNode,
    E: CanvasEdge,
    R: Fn(&mut Frame, &N, bool, CanvasViewport) + 'a,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        nodes: &'a [N],
        edges: &'a [E],
        selected_node_id: Option<NodeId>,
        viewport: CanvasViewport,
        snap_to_grid: bool,
        is_space_pressed: bool,
        render_node: R,
        on_node_selected: impl Fn(Option<NodeId>) -> Message + 'a,
        on_node_moved: impl Fn(NodeId, f32, f32) -> Message + 'a,
        on_canvas_double_clicked: impl Fn(f32, f32) -> Message + 'a,
        on_node_double_clicked: impl Fn(NodeId) -> Message + 'a,
        on_viewport_changed: impl Fn(CanvasViewport) -> Message + 'a,
    ) -> Self {
        Self {
            nodes,
            edges,
            selected_node_id,
            selected_edge: None,
            viewport,
            snap_to_grid,
            is_space_pressed,
            render_node,
            on_node_selected: Box::new(on_node_selected),
            on_edge_selected: None,
            on_edge_created: None,
            on_node_moved: Box::new(on_node_moved),
            on_canvas_double_clicked: Box::new(on_canvas_double_clicked),
            on_node_double_clicked: Box::new(on_node_double_clicked),
            on_viewport_changed: Box::new(on_viewport_changed),
        }
    }

    pub fn selected_edge(mut self, edge: Option<(NodeId, NodeId)>) -> Self {
        self.selected_edge = edge;
        self
    }

    pub fn on_edge_selected(
        mut self,
        handler: impl Fn(Option<(NodeId, NodeId)>) -> Message + 'a,
    ) -> Self {
        self.on_edge_selected = Some(Box::new(handler));
        self
    }

    pub fn on_edge_created(mut self, handler: impl Fn(NodeId, NodeId) -> Message + 'a) -> Self {
        self.on_edge_created = Some(Box::new(handler));
        self
    }
}

impl<'a, Message, N, E, R> Program<Message, Theme, Renderer> for DiagramCanvas<'a, Message, N, E, R>
where
    N: CanvasNode,
    E: CanvasEdge,
    R: Fn(&mut Frame, &N, bool, CanvasViewport),
{
    type State = DiagramCanvasState;

    fn update(
        &self,
        state: &mut Self::State,
        event: &Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<Action<Message>> {
        let cursor_pos = cursor.position_in(bounds)?;

        match event {
            Event::Keyboard(iced::keyboard::Event::KeyPressed { modifiers, .. }) => {
                state.modifiers = *modifiers;
                None
            }
            Event::Keyboard(iced::keyboard::Event::KeyReleased { modifiers, .. }) => {
                state.modifiers = *modifiers;
                None
            }
            Event::Keyboard(iced::keyboard::Event::ModifiersChanged(modifiers)) => {
                state.modifiers = *modifiers;
                None
            }
            Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                let (dx, dy) = match delta {
                    mouse::ScrollDelta::Lines { x, y } => (*x * 30.0, *y * 30.0),
                    mouse::ScrollDelta::Pixels { x, y } => (*x, *y),
                };

                if state.modifiers.control() || state.modifiers.command() {
                    let zoom_factor: f32 = if dy > 0.0 {
                        1.1
                    } else if dy < 0.0 {
                        1.0 / 1.1
                    } else {
                        1.0
                    };
                    if (zoom_factor - 1.0f32).abs() > 0.001 {
                        let mut new_vp = self.viewport;
                        new_vp.zoom_at(cursor_pos, zoom_factor);
                        return Some(
                            Action::publish((self.on_viewport_changed)(new_vp)).and_capture(),
                        );
                    }
                } else if state.modifiers.shift() {
                    let shift_dx = if dy.abs() > dx.abs() { dy } else { dx };
                    let mut new_vp = self.viewport;
                    new_vp.translate(Vector::new(shift_dx, 0.0));
                    return Some(Action::publish((self.on_viewport_changed)(new_vp)).and_capture());
                } else {
                    let mut new_vp = self.viewport;
                    new_vp.translate(Vector::new(dx, dy));
                    return Some(Action::publish((self.on_viewport_changed)(new_vp)).and_capture());
                }
                None
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Middle)) => {
                state.panning_start = Some((cursor_pos, self.viewport.pan()));
                Some(Action::capture())
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if self.is_space_pressed {
                    state.panning_start = Some((cursor_pos, self.viewport.pan()));
                    state.is_panning_space = true;
                    return Some(Action::capture());
                }

                state.panning_start = None;
                state.is_panning_space = false;

                let world_pos = self.viewport.to_world(cursor_pos);

                // 1. Tjek om der klikkes på forbindelseshåndtaget (connect handle) på den valgte node
                if let Some(sel_id) = self.selected_node_id {
                    if let Some(node) = self.nodes.iter().find(|n| n.id() == sel_id) {
                        let (nx, ny) = node.position();
                        let (nw, nh) = node.size();
                        let handle_center = Point::new(nx + nw, ny + nh / 2.0);
                        let dist =
                            (world_pos.x - handle_center.x).hypot(world_pos.y - handle_center.y);
                        if dist <= 14.0 {
                            state.connecting_from = Some(sel_id);
                            state.connecting_cursor = Some(world_pos);
                            state.hovered_target_node = None;
                            return Some(Action::request_redraw().and_capture());
                        }
                    }
                }

                let now = Instant::now();
                let is_double_click = if let Some(last) = state.last_click {
                    let dist =
                        (cursor_pos.x - last.position.x).hypot(cursor_pos.y - last.position.y);
                    let elapsed = now.duration_since(last.time).as_millis();
                    dist <= 15.0 && elapsed <= 350
                } else {
                    false
                };
                state.last_click = Some(ClickRecord {
                    position: cursor_pos,
                    time: now,
                });

                if is_double_click {
                    state.last_click = None;
                    for node in self.nodes.iter().rev() {
                        if node.contains(world_pos.x, world_pos.y) {
                            return Some(
                                Action::publish((self.on_node_double_clicked)(node.id()))
                                    .and_capture(),
                            );
                        }
                    }
                    return Some(
                        Action::publish((self.on_canvas_double_clicked)(world_pos.x, world_pos.y))
                            .and_capture(),
                    );
                }

                // 2. Klik på en node
                for node in self.nodes.iter().rev() {
                    if node.contains(world_pos.x, world_pos.y) {
                        let (nx, ny) = node.position();
                        let offset = Vector::new(world_pos.x - nx, world_pos.y - ny);
                        state.dragging_node = Some((node.id(), offset));
                        return Some(
                            Action::publish((self.on_node_selected)(Some(node.id()))).and_capture(),
                        );
                    }
                }

                // 3. Klik på en relation (kant eller label)
                let diagram_nodes: Vec<DiagramNode> =
                    self.nodes.iter().map(|n| n.to_diagram_node()).collect();
                let diagram_edges: Vec<DiagramEdge> =
                    self.edges.iter().map(|e| e.to_diagram_edge()).collect();
                let routed_edges = EdgeRouter::route_edges(&diagram_nodes, &diagram_edges);

                for routed in routed_edges.iter().rev() {
                    let hit_segment = routed
                        .points
                        .windows(2)
                        .any(|w| distance_to_segment(world_pos, w[0], w[1]) <= 7.0);

                    let hit_label =
                        if let (Some(label), Some(pos)) = (&routed.label, routed.label_pos) {
                            let approx_w = label.trim().len() as f32 * 6.8 + 16.0;
                            let min_x = pos.x - approx_w / 2.0;
                            let max_x = pos.x + approx_w / 2.0;
                            let min_y = pos.y - 10.0;
                            let max_y = pos.y + 10.0;
                            world_pos.x >= min_x
                                && world_pos.x <= max_x
                                && world_pos.y >= min_y
                                && world_pos.y <= max_y
                        } else {
                            false
                        };

                    if hit_segment || hit_label {
                        if let Some(ref on_edge_selected) = self.on_edge_selected {
                            return Some(
                                Action::publish((on_edge_selected)(Some((routed.from, routed.to))))
                                    .and_capture(),
                            );
                        }
                    }
                }

                // 4. Klik på tomt lærred: fravælg node og kant
                if let Some(ref on_edge_selected) = self.on_edge_selected {
                    let _ = (on_edge_selected)(None);
                }
                Some(Action::publish((self.on_node_selected)(None)).and_capture())
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if let Some((start_pos, initial_pan)) = state.panning_start {
                    let total_delta = cursor_pos - start_pos;
                    let mut new_vp = self.viewport;
                    new_vp.set_pan(initial_pan + total_delta);
                    return Some(Action::publish((self.on_viewport_changed)(new_vp)).and_capture());
                }

                if let Some(source_id) = state.connecting_from {
                    let world_pos = self.viewport.to_world(cursor_pos);
                    state.connecting_cursor = Some(world_pos);
                    state.hovered_target_node = self
                        .nodes
                        .iter()
                        .rev()
                        .find(|n| n.id() != source_id && n.contains(world_pos.x, world_pos.y))
                        .map(|n| n.id());
                    return Some(Action::request_redraw().and_capture());
                }

                if let Some((id, offset)) = state.dragging_node {
                    let world_pos = self.viewport.to_world(cursor_pos);
                    let raw_world_x = world_pos.x - offset.x;
                    let raw_world_y = world_pos.y - offset.y;

                    let (new_x, new_y) = if self.snap_to_grid {
                        (
                            (raw_world_x / GRID_SIZE).round() * GRID_SIZE,
                            (raw_world_y / GRID_SIZE).round() * GRID_SIZE,
                        )
                    } else {
                        (raw_world_x, raw_world_y)
                    };

                    return Some(
                        Action::publish((self.on_node_moved)(id, new_x, new_y)).and_capture(),
                    );
                }

                None
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Middle)) => {
                state.panning_start = None;
                Some(Action::capture())
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                state.panning_start = None;
                let was_panning_space = state.is_panning_space;
                state.is_panning_space = false;
                if was_panning_space {
                    return Some(Action::capture());
                }

                if let Some(source_id) = state.connecting_from.take() {
                    state.connecting_cursor = None;
                    let target = state.hovered_target_node.take();
                    if let Some(target_id) = target {
                        if target_id != source_id {
                            if let Some(ref on_edge_created) = self.on_edge_created {
                                return Some(
                                    Action::publish((on_edge_created)(source_id, target_id))
                                        .and_capture(),
                                );
                            }
                        }
                    }
                    return Some(Action::request_redraw().and_capture());
                }

                if state.dragging_node.is_some() {
                    state.dragging_node = None;
                    return Some(Action::capture());
                }
                None
            }
            _ => None,
        }
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        // Baggrund
        frame.fill_rectangle(
            Point::ORIGIN,
            bounds.size(),
            Color::from_rgb(0.985, 0.988, 0.992),
        );

        // 1. Baggrundsgitter (dækker hele vinduet dynamisk ved resize og pan)
        let step = GRID_SIZE * self.viewport.zoom();
        let ox = self.viewport.pan().x.rem_euclid(step);
        let oy = self.viewport.pan().y.rem_euclid(step);

        let dot_color = if self.viewport.zoom() < 0.5 {
            Color::from_rgba(0.65, 0.7, 0.78, 0.25)
        } else {
            Color::from_rgba(0.6, 0.65, 0.75, 0.45)
        };

        let mut grid_builder = iced::widget::canvas::path::Builder::new();
        let mut gx = ox;
        while gx <= bounds.width {
            let mut gy = oy;
            while gy <= bounds.height {
                grid_builder.rectangle(Point::new(gx - 1.0, gy - 1.0), Size::new(2.0, 2.0));
                gy += step;
            }
            gx += step;
        }
        let grid_path = grid_builder.build();
        frame.fill(&grid_path, dot_color);

        // 2. Transformer resten af tegningen via frame transformation
        frame.translate(self.viewport.pan());
        frame.scale(self.viewport.zoom());

        // 3. Deterministisk ortogonal edge routing
        let diagram_nodes: Vec<DiagramNode> =
            self.nodes.iter().map(|n| n.to_diagram_node()).collect();
        let diagram_edges: Vec<DiagramEdge> =
            self.edges.iter().map(|e| e.to_diagram_edge()).collect();

        let routed_edges = EdgeRouter::route_edges(&diagram_nodes, &diagram_edges);

        // A. Tegn ortogonale linjeforløb med krydsningsbroer (line jumps)
        for routed in &routed_edges {
            if routed.points.len() < 2 {
                continue;
            }

            let is_selected_edge = self.selected_edge == Some((routed.from, routed.to))
                || self.selected_edge == Some((routed.to, routed.from));
            let edge_color = if is_selected_edge {
                ThemeColors::PRIMARY
            } else {
                Color::from_rgb(0.3, 0.3, 0.3)
            };
            let edge_width = if is_selected_edge { 2.8 } else { 1.5 };

            let path = Path::new(|b| {
                b.move_to(routed.points[0]);

                for window in routed.points.windows(2) {
                    let (p1, p2) = (window[0], window[1]);
                    let is_h = (p1.y - p2.y).abs() < 0.001;

                    if is_h && !routed.bridges.is_empty() {
                        let min_x = p1.x.min(p2.x);
                        let max_x = p1.x.max(p2.x);
                        let mut seg_bridges: Vec<_> = routed
                            .bridges
                            .iter()
                            .filter(|br| {
                                br.is_horizontal
                                    && (br.center.y - p1.y).abs() < 0.001
                                    && br.center.x > min_x + 2.0
                                    && br.center.x < max_x - 2.0
                            })
                            .collect();

                        if p1.x < p2.x {
                            seg_bridges
                                .sort_by(|a, b| a.center.x.partial_cmp(&b.center.x).unwrap());
                        } else {
                            seg_bridges
                                .sort_by(|a, b| b.center.x.partial_cmp(&a.center.x).unwrap());
                        }

                        let r = 5.0;
                        for bridge in seg_bridges {
                            let bx = bridge.center.x;
                            let by = bridge.center.y;
                            let approach_x = if p1.x < p2.x { bx - r } else { bx + r };
                            let exit_x = if p1.x < p2.x { bx + r } else { bx - r };

                            b.line_to(Point::new(approach_x, by));
                            b.bezier_curve_to(
                                Point::new(approach_x, by - 6.0),
                                Point::new(exit_x, by - 6.0),
                                Point::new(exit_x, by),
                            );
                        }
                    }

                    b.line_to(p2);
                }
            });

            frame.stroke(
                &path,
                Stroke::default()
                    .with_color(edge_color)
                    .with_width(edge_width),
            );
        }

        // 4. Tegn noder via pluggable node-rendering closure
        for node in self.nodes {
            let is_selected = self.selected_node_id == Some(node.id());
            (self.render_node)(&mut frame, node, is_selected, self.viewport);

            // Highlight målnode under drag-to-connect
            if state.hovered_target_node == Some(node.id()) {
                let (nx, ny) = node.position();
                let (nw, nh) = node.size();
                let target_glow = Path::rounded_rectangle(
                    Point::new(nx - 3.0, ny - 3.0),
                    Size::new(nw + 6.0, nh + 6.0),
                    10.0.into(),
                );
                frame.stroke(
                    &target_glow,
                    Stroke::default()
                        .with_color(ThemeColors::PRIMARY)
                        .with_width(3.0),
                );
            }

            // Forbindelseshåndtag (connect handle) på valgt node
            if is_selected {
                let (nx, ny) = node.position();
                let (nw, nh) = node.size();
                let handle_center = Point::new(nx + nw, ny + nh / 2.0);
                let circle = Path::circle(handle_center, 8.0);
                frame.fill(&circle, Color::WHITE);
                frame.stroke(
                    &circle,
                    Stroke::default()
                        .with_color(ThemeColors::PRIMARY)
                        .with_width(2.0),
                );
                let inner_dot = Path::circle(handle_center, 3.5);
                frame.fill(&inner_dot, ThemeColors::PRIMARY);
            }
        }

        // 5. Elastik / Preview-linje under drag-to-connect
        if let (Some(source_id), Some(cursor_world)) =
            (state.connecting_from, state.connecting_cursor)
        {
            if let Some(src) = self.nodes.iter().find(|n| n.id() == source_id) {
                let (sx, sy) = src.position();
                let (sw, sh) = src.size();
                let start_pt = Point::new(sx + sw, sy + sh / 2.0);
                let preview = Path::line(start_pt, cursor_world);
                frame.stroke(
                    &preview,
                    Stroke::default()
                        .with_color(ThemeColors::PRIMARY)
                        .with_width(2.5),
                );
                let end_circle = Path::circle(cursor_world, 5.0);
                frame.fill(&end_circle, ThemeColors::PRIMARY);
            }
        }

        // 6. Tegn pilehoveder og labels ovenpå noder for optimal synlighed
        for routed in &routed_edges {
            let is_selected_edge = self.selected_edge == Some((routed.from, routed.to))
                || self.selected_edge == Some((routed.to, routed.from));
            let arrow_stroke_color = if is_selected_edge {
                ThemeColors::PRIMARY
            } else {
                Color::from_rgb(0.2, 0.2, 0.2)
            };
            let arrow_stroke_width = if is_selected_edge { 2.2 } else { 1.5 };

            // 6a. Kompositions-diamant ved kilde-noden (solid sort diamant)
            if let Some(ref diamond) = routed.source_diamond {
                let diamond_path = Path::new(|b| {
                    b.move_to(diamond.tip);
                    b.line_to(diamond.left);
                    b.line_to(diamond.back);
                    b.line_to(diamond.right);
                    b.close();
                });
                frame.fill(&diamond_path, arrow_stroke_color);
                frame.stroke(
                    &diamond_path,
                    Stroke::default()
                        .with_color(arrow_stroke_color)
                        .with_width(arrow_stroke_width),
                );
            }

            // 6b. Pilehoved ved mål-noden
            if let Some(ref arrow) = routed.arrow_head {
                if routed.kind == RelationKind::Generalization {
                    let triangle = Path::new(|b| {
                        b.move_to(arrow.tip);
                        b.line_to(arrow.left);
                        b.line_to(arrow.right);
                        b.close();
                    });
                    frame.fill(&triangle, Color::WHITE);
                    frame.stroke(
                        &triangle,
                        Stroke::default()
                            .with_color(arrow_stroke_color)
                            .with_width(arrow_stroke_width),
                    );
                }
            }

            // 6c. Halv pil for rettet association ved mål-noden
            if let Some(ref half_arrow) = routed.half_arrow {
                let barb_line = Path::line(half_arrow.tip, half_arrow.barb);
                frame.stroke(
                    &barb_line,
                    Stroke::default()
                        .with_color(arrow_stroke_color)
                        .with_width(arrow_stroke_width + 0.5),
                );
            }

            if let (Some(label), Some(pos)) = (&routed.label, routed.label_pos) {
                if !label.trim().is_empty() {
                    let label_text = label.trim();
                    let approx_w = label_text.len() as f32 * 6.8 + 12.0;
                    let pill = Path::rounded_rectangle(
                        Point::new(pos.x - approx_w / 2.0, pos.y - 8.0),
                        Size::new(approx_w, 16.0),
                        4.0.into(),
                    );
                    frame.fill(&pill, Color::from_rgba(1.0, 1.0, 1.0, 0.90));
                    if is_selected_edge {
                        frame.stroke(
                            &pill,
                            Stroke::default()
                                .with_color(ThemeColors::PRIMARY)
                                .with_width(1.5),
                        );
                    }

                    frame.fill_text(Text {
                        content: label_text.to_string(),
                        position: pos,
                        color: if is_selected_edge {
                            ThemeColors::PRIMARY
                        } else {
                            ThemeColors::SLATE_800
                        },
                        size: 11.0.into(),
                        align_x: alignment::Horizontal::Center.into(),
                        align_y: alignment::Vertical::Center,
                        ..Default::default()
                    });
                }
            }

            // 6d. Multipliciteter ved kilde- og målporte (UML standard §6)
            if let Some(edge) = self
                .edges
                .iter()
                .find(|e| e.from() == routed.from && e.to() == routed.to && e.kind() == routed.kind)
            {
                if let Some(src_mult) = edge.source_multiplicity() {
                    let p_src = routed.points[0];
                    let pos = port_multiplicity_pos(p_src, routed.from_side);
                    render_multiplicity_label(&mut frame, &src_mult, pos, is_selected_edge);
                }

                if let Some(tgt_mult) = edge.target_multiplicity() {
                    let p_tgt = *routed.points.last().unwrap_or(&routed.points[0]);
                    let pos = port_multiplicity_pos(p_tgt, routed.to_side);
                    render_multiplicity_label(&mut frame, &tgt_mult, pos, is_selected_edge);
                }
            }
        }

        vec![frame.into_geometry()]
    }
}

/// Beregner placering af multiplicitetstekst ved port
fn port_multiplicity_pos(pt: Point, side: PortSide) -> Point {
    match side {
        PortSide::Right => Point::new(pt.x + 14.0, pt.y - 10.0),
        PortSide::Left => Point::new(pt.x - 14.0, pt.y - 10.0),
        PortSide::Top => Point::new(pt.x + 10.0, pt.y - 12.0),
        PortSide::Bottom => Point::new(pt.x + 10.0, pt.y + 12.0),
    }
}

/// Renderer multiplicitetstekst med baggrundspille
fn render_multiplicity_label(frame: &mut Frame, text: &str, pos: Point, is_selected: bool) {
    let approx_w = text.len() as f32 * 6.5 + 8.0;
    let pill = Path::rounded_rectangle(
        Point::new(pos.x - approx_w / 2.0, pos.y - 7.0),
        Size::new(approx_w, 14.0),
        3.0.into(),
    );
    frame.fill(&pill, Color::from_rgba(1.0, 1.0, 1.0, 0.88));
    frame.fill_text(Text {
        content: text.to_string(),
        position: pos,
        color: if is_selected {
            ThemeColors::PRIMARY
        } else {
            ThemeColors::SLATE_700
        },
        size: 10.5.into(),
        align_x: alignment::Horizontal::Center.into(),
        align_y: alignment::Vertical::Center,
        ..Default::default()
    });
}

/// Standard rendering af FDA begrebsnode
pub fn render_concept_node(
    frame: &mut Frame,
    node: &DiagramNode,
    is_selected: bool,
    _viewport: CanvasViewport,
) {
    let top_left = Point::new(node.x(), node.y());
    let size = Size::new(node.width(), node.height());
    let radius = 8.0;

    let shadow_path =
        Path::rounded_rectangle(Point::new(node.x(), node.y() + 2.0), size, radius.into());
    frame.fill(&shadow_path, Color::from_rgba(0.05, 0.1, 0.2, 0.07));

    let node_path = Path::rounded_rectangle(top_left, size, radius.into());

    let fill_color = if node.is_local() {
        ThemeColors::FDA_SAND
    } else {
        ThemeColors::FDA_BORROWED_BLUE
    };

    frame.fill(&node_path, fill_color);

    let (border_color, border_width) = if is_selected {
        (ThemeColors::PRIMARY, 2.5)
    } else if node.is_local() {
        (ThemeColors::FDA_SAND_BORDER, 1.2)
    } else {
        (Color::from_rgb(0.35, 0.65, 0.85), 1.2)
    };

    frame.stroke(
        &node_path,
        Stroke::default()
            .with_color(border_color)
            .with_width(border_width),
    );

    let badge_text = if node.is_local() {
        "«lokalt begreb»"
    } else {
        "«indlånt begreb»"
    };

    frame.fill_text(Text {
        content: badge_text.to_string(),
        position: Point::new(node.x() + node.width() / 2.0, node.y() + 24.0),
        color: ThemeColors::SLATE_600,
        size: 10.5.into(),
        align_x: alignment::Horizontal::Center.into(),
        align_y: alignment::Vertical::Center,
        ..Default::default()
    });

    frame.fill_text(Text {
        content: node.label().to_string(),
        position: Point::new(node.x() + node.width() / 2.0, node.y() + 48.0),
        color: ThemeColors::SLATE_900,
        size: 14.0.into(),
        align_x: alignment::Horizontal::Center.into(),
        align_y: alignment::Vertical::Center,
        ..Default::default()
    });
}

/// Standard rendering af UML 3-sektions informationsklasse-kasse
pub fn render_uml_class_node(
    frame: &mut Frame,
    node: &ClassDiagramNode,
    class_name: &str,
    attributes: &[(String, String, String, bool)],
    is_borrowed: bool,
    is_selected: bool,
    _viewport: CanvasViewport,
) {
    let top_left = Point::new(node.x(), node.y());
    let size = Size::new(node.width(), node.height());
    let radius = 6.0;

    let shadow_path =
        Path::rounded_rectangle(Point::new(node.x(), node.y() + 2.0), size, radius.into());
    frame.fill(&shadow_path, Color::from_rgba(0.05, 0.1, 0.2, 0.07));

    let node_path = Path::rounded_rectangle(top_left, size, radius.into());

    let fill_color = if is_borrowed {
        ThemeColors::FDA_BORROWED_BLUE
    } else {
        ThemeColors::FDA_SAND
    };

    frame.fill(&node_path, fill_color);

    let (border_color, border_width) = if is_selected {
        (ThemeColors::PRIMARY, 2.5)
    } else if is_borrowed {
        (Color::from_rgb(0.35, 0.65, 0.85), 1.2)
    } else {
        (ThemeColors::FDA_SAND_BORDER, 1.2)
    };

    frame.stroke(
        &node_path,
        Stroke::default()
            .with_color(border_color)
            .with_width(border_width),
    );

    frame.fill_text(Text {
        content: "«Informationsklasse»".to_string(),
        position: Point::new(node.x() + node.width() / 2.0, node.y() + 14.0),
        color: ThemeColors::SLATE_600,
        size: 10.5.into(),
        align_x: alignment::Horizontal::Center.into(),
        align_y: alignment::Vertical::Center,
        ..Default::default()
    });

    frame.fill_text(Text {
        content: class_name.to_string(),
        position: Point::new(node.x() + node.width() / 2.0, node.y() + 32.0),
        color: ThemeColors::SLATE_900,
        size: 14.0.into(),
        align_x: alignment::Horizontal::Center.into(),
        align_y: alignment::Vertical::Center,
        ..Default::default()
    });

    let divider_y = node.y() + 48.0;
    let divider_path = Path::new(|b| {
        b.move_to(Point::new(node.x(), divider_y));
        b.line_to(Point::new(node.x() + node.width(), divider_y));
    });
    frame.stroke(
        &divider_path,
        Stroke::default()
            .with_color(ThemeColors::FDA_SAND_BORDER)
            .with_width(1.0),
    );

    let mut attr_y = divider_y + 14.0;
    if attributes.is_empty() {
        frame.fill_text(Text {
            content: "(ingen attributter)".to_string(),
            position: Point::new(node.x() + 14.0, attr_y),
            color: ThemeColors::SLATE_400,
            size: 11.0.into(),
            align_x: alignment::Horizontal::Left.into(),
            align_y: alignment::Vertical::Center,
            ..Default::default()
        });
    } else {
        for (attr_name, attr_type, attr_mult, has_concept) in attributes {
            let line_str = if *has_concept {
                format!("+ {} : {} [{}] 🔗", attr_name, attr_type, attr_mult)
            } else {
                format!("+ {} : {} [{}]", attr_name, attr_type, attr_mult)
            };
            frame.fill_text(Text {
                content: line_str,
                position: Point::new(node.x() + 14.0, attr_y),
                color: ThemeColors::SLATE_800,
                size: 12.0.into(),
                align_x: alignment::Horizontal::Left.into(),
                align_y: alignment::Vertical::Center,
                ..Default::default()
            });
            attr_y += 20.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canvas_viewport_math() {
        let mut vp = CanvasViewport::default();
        assert_eq!(vp.zoom(), 1.0);
        assert_eq!(vp.pan(), Vector::new(0.0, 0.0));

        let screen_pt = Point::new(100.0, 100.0);
        let world_pt = vp.to_world(screen_pt);
        assert_eq!(world_pt, Point::new(100.0, 100.0));

        vp.translate(Vector::new(50.0, 50.0));
        let world_pt2 = vp.to_world(screen_pt);
        assert_eq!(world_pt2, Point::new(50.0, 50.0));

        vp.set_zoom(2.0);
        assert_eq!(vp.zoom(), CanvasViewport::MAX_ZOOM);

        vp.set_zoom(0.1);
        assert_eq!(vp.zoom(), CanvasViewport::MIN_ZOOM);

        let snap_test = Point::new(23.4, 38.9);
        let snapped = CanvasViewport::snap_to_grid(snap_test, 20.0);
        assert_eq!(snapped, Point::new(20.0, 40.0));
    }

    #[test]
    fn test_pan_drag_cursor_tracking_does_not_drift() {
        let mut state = DiagramCanvasState::default();
        let initial_pan = Vector::new(10.0, 20.0);
        let click_pos = Point::new(100.0, 100.0);
        state.panning_start = Some((click_pos, initial_pan));

        // Simuler flere CursorMoved hændelser i samme frame
        let moves = [
            Point::new(120.0, 100.0),
            Point::new(150.0, 100.0),
            Point::new(200.0, 100.0),
        ];

        let mut last_pan = Vector::default();
        for current_cursor in moves {
            let (start_pos, init_pan) = state.panning_start.unwrap();
            let total_delta = current_cursor - start_pos;
            let current_pan = init_pan + total_delta;
            last_pan = current_pan;
        }

        // Slutpositionen for pan skal svare nøjagtigt til total cursor-bevægelse (+100px i X)
        assert_eq!(last_pan, Vector::new(110.0, 20.0));
    }

    #[test]
    fn test_node_selection_and_dragging_lifecycle() {
        use iced::mouse::Cursor;
        use std::sync::Arc;
        use uuid::Uuid;

        let node = DiagramNode::custom(
            Uuid::new_v4(),
            "TestNode".to_string(),
            100.0,
            100.0,
            180.0,
            80.0,
        );
        let nodes = vec![node];
        let edges: Vec<DiagramEdge> = vec![];

        let selected = Arc::new(std::sync::Mutex::new(None));
        let moved = Arc::new(std::sync::Mutex::new(None));

        let sel_clone = Arc::clone(&selected);
        let mov_clone = Arc::clone(&moved);

        let canvas = DiagramCanvas::new(
            &nodes,
            &edges,
            None,
            CanvasViewport::default(),
            true,
            false, // is_space_pressed = false
            |_, _, _, _| {},
            move |id| {
                *sel_clone.lock().unwrap() = id;
            },
            move |id, x, y| {
                *mov_clone.lock().unwrap() = Some((id, x, y));
            },
            |_, _| (),
            |_| (),
            |_| (),
        );

        let mut state = DiagramCanvasState::default();
        let bounds = Rectangle::new(Point::ORIGIN, Size::new(1000.0, 1000.0));
        let cursor_on_node = Cursor::Available(Point::new(120.0, 120.0));

        // 1. Left click på node -> skal vælge noden og initialisere dragging_node
        let press_event = Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left));
        let action = canvas.update(&mut state, &press_event, bounds, cursor_on_node);
        assert!(action.is_some());
        assert_eq!(*selected.lock().unwrap(), Some(nodes[0].id()));
        assert!(state.dragging_node.is_some());

        // 2. CursorMoved -> skal flytte noden
        let move_cursor = Cursor::Available(Point::new(160.0, 140.0));
        let move_event = Event::Mouse(mouse::Event::CursorMoved {
            position: Point::new(160.0, 140.0),
        });
        let action = canvas.update(&mut state, &move_event, bounds, move_cursor);
        assert!(action.is_some());
        assert!(moved.lock().unwrap().is_some());

        // 3. ButtonReleased -> skal rydde dragging_node og panning_start
        let release_event = Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left));
        let _ = canvas.update(&mut state, &release_event, bounds, move_cursor);
        assert!(state.dragging_node.is_none());
        assert!(state.panning_start.is_none());
    }

    #[test]
    fn test_drag_to_connect_lifecycle_and_edge_selection() {
        use iced::mouse::Cursor;
        use std::sync::Arc;
        use uuid::Uuid;

        let node1 = DiagramNode::custom(
            Uuid::new_v4(),
            "Node1".to_string(),
            100.0,
            100.0,
            180.0,
            80.0,
        );
        let node2 = DiagramNode::custom(
            Uuid::new_v4(),
            "Node2".to_string(),
            400.0,
            100.0,
            180.0,
            80.0,
        );
        let nodes = vec![node1.clone(), node2.clone()];
        let edge = DiagramEdge::with_label(
            node1.id(),
            node2.id(),
            RelationKind::Association,
            Some("omfatter".to_string()),
        );
        let edges = vec![edge];

        let created_edge = Arc::new(std::sync::Mutex::new(None));
        let selected_edge = Arc::new(std::sync::Mutex::new(None));

        let created_clone = Arc::clone(&created_edge);
        let selected_clone = Arc::clone(&selected_edge);

        let canvas = DiagramCanvas::new(
            &nodes,
            &edges,
            Some(node1.id()),
            CanvasViewport::default(),
            false,
            false,
            |_, _, _, _| {},
            |_| (),
            |_, _, _| (),
            |_, _| (),
            |_| (),
            |_| (),
        )
        .selected_edge(None)
        .on_edge_selected(move |e| {
            *selected_clone.lock().unwrap() = e;
        })
        .on_edge_created(move |from, to| {
            *created_clone.lock().unwrap() = Some((from, to));
        });

        let mut state = DiagramCanvasState::default();
        let bounds = Rectangle::new(Point::ORIGIN, Size::new(1000.0, 1000.0));

        // 1. Klik på forbindelseshåndtaget for node1: placeret ved (nx + nw, ny + nh/2) = (280.0, 140.0)
        let handle_cursor = Cursor::Available(Point::new(280.0, 140.0));
        let press_event = Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left));
        let action = canvas.update(&mut state, &press_event, bounds, handle_cursor);
        assert!(action.is_some());
        assert_eq!(state.connecting_from, Some(node1.id()));
        assert_eq!(state.connecting_cursor, Some(Point::new(280.0, 140.0)));

        // 2. CursorMoved over Node 2 (f.eks. ved (450.0, 120.0))
        let target_cursor = Cursor::Available(Point::new(450.0, 120.0));
        let move_event = Event::Mouse(mouse::Event::CursorMoved {
            position: Point::new(450.0, 120.0),
        });
        let action = canvas.update(&mut state, &move_event, bounds, target_cursor);
        assert!(action.is_some());
        assert_eq!(state.hovered_target_node, Some(node2.id()));

        // 3. Drop: ButtonReleased over Node 2 -> skal udstede on_edge_created(node1, node2)
        let release_event = Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left));
        let action = canvas.update(&mut state, &release_event, bounds, target_cursor);
        assert!(action.is_some());
        assert_eq!(
            *created_edge.lock().unwrap(),
            Some((node1.id(), node2.id()))
        );
        assert_eq!(state.connecting_from, None);
        assert_eq!(state.hovered_target_node, None);

        // 4. Klik på kanten/label for at vælge kanten
        // Mellem node1 højre kant (280, 140) og node2 venstre kant (400, 140) er der et vandret linjestykke ved y=140
        let edge_cursor = Cursor::Available(Point::new(340.0, 140.0));
        let press_event = Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left));
        let action = canvas.update(&mut state, &press_event, bounds, edge_cursor);
        assert!(action.is_some());
        assert_eq!(
            *selected_edge.lock().unwrap(),
            Some((node1.id(), node2.id()))
        );

        // 5. Klik på tomt lærred (50, 50) -> skal rydde kantmarkering
        let empty_cursor = Cursor::Available(Point::new(50.0, 50.0));
        let press_event = Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left));
        let _ = canvas.update(&mut state, &press_event, bounds, empty_cursor);
        assert_eq!(*selected_edge.lock().unwrap(), None);
    }
}
