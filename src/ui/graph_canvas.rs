use crate::features::concept_model::{ConceptGraph, NodeId, RelationKind, GRID_SIZE};
use crate::ui::theme::ThemeColors;
use iced::mouse;
use iced::widget::canvas::{Action, Event, Frame, Geometry, Path, Program, Stroke, Text};
use iced::{alignment, Color, Point, Rectangle, Renderer, Size, Theme, Vector};
use std::time::Instant;

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

#[derive(Debug, Default)]
pub struct GraphCanvasState {
    dragging_node: Option<(NodeId, Vector)>,
    last_click: Option<ClickRecord>,
    panning_start: Option<Point>,
    is_panning_space: bool,
    space_pressed: bool,
    modifiers: iced::keyboard::Modifiers,
}

pub struct GraphCanvas<'a, Message> {
    graph: &'a ConceptGraph,
    selected_node_id: Option<NodeId>,
    viewport: CanvasViewport,
    snap_to_grid: bool,
    is_space_pressed: bool,
    on_node_selected: Box<dyn Fn(Option<NodeId>) -> Message + 'a>,
    on_node_moved: Box<dyn Fn(NodeId, f32, f32) -> Message + 'a>,
    on_canvas_double_clicked: Box<dyn Fn(f32, f32) -> Message + 'a>,
    on_node_double_clicked: Box<dyn Fn(NodeId) -> Message + 'a>,
    on_viewport_changed: Box<dyn Fn(CanvasViewport) -> Message + 'a>,
}

impl<'a, Message> GraphCanvas<'a, Message> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        graph: &'a ConceptGraph,
        selected_node_id: Option<NodeId>,
        viewport: CanvasViewport,
        snap_to_grid: bool,
        is_space_pressed: bool,
        on_node_selected: impl Fn(Option<NodeId>) -> Message + 'a,
        on_node_moved: impl Fn(NodeId, f32, f32) -> Message + 'a,
        on_canvas_double_clicked: impl Fn(f32, f32) -> Message + 'a,
        on_node_double_clicked: impl Fn(NodeId) -> Message + 'a,
        on_viewport_changed: impl Fn(CanvasViewport) -> Message + 'a,
    ) -> Self {
        Self {
            graph,
            selected_node_id,
            viewport,
            snap_to_grid,
            is_space_pressed,
            on_node_selected: Box::new(on_node_selected),
            on_node_moved: Box::new(on_node_moved),
            on_canvas_double_clicked: Box::new(on_canvas_double_clicked),
            on_node_double_clicked: Box::new(on_node_double_clicked),
            on_viewport_changed: Box::new(on_viewport_changed),
        }
    }
}

impl<'a, Message> Program<Message, Theme, Renderer> for GraphCanvas<'a, Message> {
    type State = GraphCanvasState;

    fn update(
        &self,
        state: &mut Self::State,
        event: &Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<Action<Message>> {
        let cursor_pos = cursor.position_in(bounds)?;

        match event {
            Event::Keyboard(iced::keyboard::Event::KeyPressed { key, modifiers, .. }) => {
                state.modifiers = *modifiers;
                if let iced::keyboard::Key::Named(iced::keyboard::key::Named::Space) = key {
                    state.space_pressed = true;
                }
                None
            }
            Event::Keyboard(iced::keyboard::Event::KeyReleased { key, modifiers, .. }) => {
                state.modifiers = *modifiers;
                if let iced::keyboard::Key::Named(iced::keyboard::key::Named::Space) = key {
                    state.space_pressed = false;
                    state.is_panning_space = false;
                    state.panning_start = None;
                }
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
                state.panning_start = Some(cursor_pos);
                Some(Action::capture())
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if self.is_space_pressed || state.space_pressed {
                    state.panning_start = Some(cursor_pos);
                    state.is_panning_space = true;
                    return Some(Action::capture());
                }

                let now = Instant::now();
                let is_double_click = if let Some(last) = state.last_click {
                    let dx = last.position.x - cursor_pos.x;
                    let dy = last.position.y - cursor_pos.y;
                    let dist = (dx * dx + dy * dy).sqrt();
                    dist < 6.0 && now.duration_since(last.time).as_millis() <= 350
                } else {
                    false
                };

                state.last_click = if is_double_click {
                    None
                } else {
                    Some(ClickRecord {
                        position: cursor_pos,
                        time: now,
                    })
                };

                let world_pos = self.viewport.to_world(cursor_pos);

                // Tjek noder i omvendt rækkefølge (øverste node først)
                for node in self.graph.nodes().iter().rev() {
                    if node.contains(world_pos.x, world_pos.y) {
                        if is_double_click {
                            state.dragging_node = None;
                            return Some(
                                Action::publish((self.on_node_double_clicked)(node.id()))
                                    .and_capture(),
                            );
                        }
                        let offset = Vector::new(world_pos.x - node.x(), world_pos.y - node.y());
                        state.dragging_node = Some((node.id(), offset));
                        return Some(
                            Action::publish((self.on_node_selected)(Some(node.id()))).and_capture(),
                        );
                    }
                }

                if is_double_click {
                    state.dragging_node = None;
                    return Some(
                        Action::publish((self.on_canvas_double_clicked)(world_pos.x, world_pos.y))
                            .and_capture(),
                    );
                }

                // Klik på tomt lærred fjerner markering
                state.dragging_node = None;
                Some(Action::publish((self.on_node_selected)(None)))
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if let Some(last_pos) = state.panning_start {
                    let delta = Vector::new(cursor_pos.x - last_pos.x, cursor_pos.y - last_pos.y);
                    state.panning_start = Some(cursor_pos);
                    let mut new_vp = self.viewport;
                    new_vp.translate(delta);
                    return Some(Action::publish((self.on_viewport_changed)(new_vp)).and_capture());
                }

                if let Some((node_id, offset)) = state.dragging_node {
                    let world_pos = self.viewport.to_world(cursor_pos);
                    let mut new_x = (world_pos.x - offset.x).max(10.0);
                    let mut new_y = (world_pos.y - offset.y).max(10.0);
                    if self.snap_to_grid {
                        new_x = (new_x / GRID_SIZE).round() * GRID_SIZE;
                        new_y = (new_y / GRID_SIZE).round() * GRID_SIZE;
                    }
                    return Some(
                        Action::publish((self.on_node_moved)(node_id, new_x, new_y)).and_capture(),
                    );
                }
                None
            }
            Event::Mouse(mouse::Event::ButtonReleased(button)) => {
                if *button == mouse::Button::Middle {
                    state.panning_start = None;
                    return Some(Action::capture());
                }
                if *button == mouse::Button::Left {
                    if state.is_panning_space {
                        state.panning_start = None;
                        state.is_panning_space = false;
                        return Some(Action::capture());
                    }
                    if state.dragging_node.is_some() {
                        state.dragging_node = None;
                        return Some(Action::capture());
                    }
                }
                None
            }
            _ => None,
        }
    }

    fn mouse_interaction(
        &self,
        state: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if state.is_panning_space || state.panning_start.is_some() {
            return mouse::Interaction::Grabbing;
        }
        if self.is_space_pressed || state.space_pressed {
            return mouse::Interaction::Grab;
        }

        if let Some(cursor_pos) = cursor.position_in(bounds) {
            let world_pos = self.viewport.to_world(cursor_pos);
            for node in self.graph.nodes() {
                if node.contains(world_pos.x, world_pos.y) {
                    return mouse::Interaction::Pointer;
                }
            }
        }

        mouse::Interaction::default()
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry<Renderer>> {
        let mut frame = Frame::new(renderer, bounds.size());

        // 1. Baggrund: Diskret prik-gitter (dot-grid) med LOD
        let top_left = self.viewport.to_world(Point::new(0.0, 0.0));
        let bottom_right = self
            .viewport
            .to_world(Point::new(bounds.width, bounds.height));

        let min_x = top_left.x.min(bottom_right.x);
        let max_x = top_left.x.max(bottom_right.x);
        let min_y = top_left.y.min(bottom_right.y);
        let max_y = top_left.y.max(bottom_right.y);

        let step = if self.viewport.zoom() < 0.5 {
            GRID_SIZE * 2.0
        } else {
            GRID_SIZE
        };

        let start_x = (min_x / step).floor() * step;
        let start_y = (min_y / step).floor() * step;

        let dot_color = if self.viewport.zoom() < 0.5 {
            Color::from_rgba(0.65, 0.7, 0.78, 0.25)
        } else {
            Color::from_rgba(0.6, 0.65, 0.75, 0.45)
        };

        let mut grid_builder = iced::widget::canvas::path::Builder::new();
        let mut gx = start_x;
        while gx <= max_x {
            let mut gy = start_y;
            while gy <= max_y {
                let screen_pt = self.viewport.to_screen(Point::new(gx, gy));
                grid_builder.rectangle(
                    Point::new(screen_pt.x - 1.0, screen_pt.y - 1.0),
                    Size::new(2.0, 2.0),
                );
                gy += step;
            }
            gx += step;
        }
        let grid_path = grid_builder.build();
        frame.fill(&grid_path, dot_color);

        // 2. Transformer resten af tegningen (Relationer og Noder) via frame transformation
        frame.translate(self.viewport.pan());
        frame.scale(self.viewport.zoom());

        // 3. Tegn UML relationer (edges)
        for edge in self.graph.edges() {
            let from_node = self.graph.find_node(edge.from());
            let to_node = self.graph.find_node(edge.to());

            if let (Some(from), Some(to)) = (from_node, to_node) {
                let (from_cx, from_cy) = from.center();
                let (to_cx, to_cy) = to.center();

                let p1 = Point::new(from_cx, from_cy);
                let p2 = Point::new(to_cx, to_cy);

                match edge.kind() {
                    RelationKind::Generalization => {
                        // Generalisering jf. FDA Tabel 1: Linje med hvid lukket trekant mod superklasse
                        let line = Path::line(p1, p2);
                        frame.stroke(
                            &line,
                            Stroke::default()
                                .with_color(Color::from_rgb(0.3, 0.3, 0.3))
                                .with_width(1.5),
                        );

                        // Beregn trekantspids ved mål-noden (to_node)
                        let dx = p2.x - p1.x;
                        let dy = p2.y - p1.y;
                        let len = (dx * dx + dy * dy).sqrt();

                        if len > 20.0 {
                            let ux = dx / len;
                            let uy = dy / len;

                            let target_pt = Point::new(
                                p2.x - ux * (to.width() * 0.35),
                                p2.y - uy * (to.height() * 0.35),
                            );
                            let arrow_len = 16.0;
                            let arrow_width = 8.0;

                            let base_pt = Point::new(
                                target_pt.x - ux * arrow_len,
                                target_pt.y - uy * arrow_len,
                            );
                            let left_pt = Point::new(
                                base_pt.x - uy * arrow_width,
                                base_pt.y + ux * arrow_width,
                            );
                            let right_pt = Point::new(
                                base_pt.x + uy * arrow_width,
                                base_pt.y - ux * arrow_width,
                            );

                            let triangle = Path::new(|b| {
                                b.move_to(target_pt);
                                b.line_to(left_pt);
                                b.line_to(right_pt);
                                b.close();
                            });

                            frame.fill(&triangle, Color::WHITE);
                            frame.stroke(
                                &triangle,
                                Stroke::default()
                                    .with_color(Color::from_rgb(0.2, 0.2, 0.2))
                                    .with_width(1.5),
                            );
                        }
                    }
                    RelationKind::Association | RelationKind::Composition => {
                        let line = Path::line(p1, p2);
                        frame.stroke(
                            &line,
                            Stroke::default()
                                .with_color(Color::from_rgb(0.3, 0.3, 0.3))
                                .with_width(1.5),
                        );

                        if let Some(label) = edge.label() {
                            if !label.trim().is_empty() {
                                let mid_x = (p1.x + p2.x) / 2.0;
                                let mid_y = (p1.y + p2.y) / 2.0 - 10.0;

                                frame.fill_text(Text {
                                    content: label.to_string(),
                                    position: Point::new(mid_x, mid_y),
                                    color: ThemeColors::PRIMARY,
                                    size: 11.0.into(),
                                    align_x: alignment::Horizontal::Center.into(),
                                    align_y: alignment::Vertical::Center,
                                    ..Default::default()
                                });
                            }
                        }
                    }
                }
            }
        }

        // 4. Tegn Noder jf. FDA Modelreglerne (Kapitel 5 & 7.3)
        for node in self.graph.nodes() {
            let top_left = Point::new(node.x(), node.y());
            let size = Size::new(node.width(), node.height());
            let radius = 8.0;

            // Subtil skygge for dybdevirkning
            let shadow_path =
                Path::rounded_rectangle(Point::new(node.x(), node.y() + 2.0), size, radius.into());
            frame.fill(&shadow_path, Color::from_rgba(0.05, 0.1, 0.2, 0.07));

            let node_path = Path::rounded_rectangle(top_left, size, radius.into());

            // Baggrundsfarve efter FDA-standard
            let fill_color = if node.is_local() {
                ThemeColors::FDA_SAND // #FEFAF7 - Eget/lokalt begreb
            } else {
                ThemeColors::FDA_BORROWED_BLUE // #87CDEB - Indlånt begreb
            };

            frame.fill(&node_path, fill_color);

            // Ramme: Fremhævet ved markering
            let is_selected = self.selected_node_id == Some(node.id());
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

            // Centreret foretrukken term (titel)
            frame.fill_text(Text {
                content: node.label().to_string(),
                position: Point::new(node.x() + node.width() / 2.0, node.y() + 28.0),
                color: ThemeColors::SLATE_900,
                size: 14.0.into(),
                align_x: alignment::Horizontal::Center.into(),
                align_y: alignment::Vertical::Center,
                ..Default::default()
            });

            // FDA Badge label
            let badge_text = if node.is_local() {
                "«lokalt begreb»"
            } else {
                "«indlånt begreb»"
            };

            frame.fill_text(Text {
                content: badge_text.to_string(),
                position: Point::new(node.x() + node.width() / 2.0, node.y() + 54.0),
                color: ThemeColors::SLATE_600,
                size: 10.5.into(),
                align_x: alignment::Horizontal::Center.into(),
                align_y: alignment::Vertical::Center,
                ..Default::default()
            });
        }

        vec![frame.into_geometry()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::concepts::{BelongsToDomain, Concept};

    #[derive(Debug, Clone, PartialEq)]
    enum TestMsg {
        Selected(Option<NodeId>),
        Moved(NodeId, f32, f32),
        CanvasDoubleClicked(f32, f32),
        NodeDoubleClicked(NodeId),
        ViewportChanged(CanvasViewport),
    }

    #[test]
    fn test_graph_canvas_double_click_events() {
        let mut graph = ConceptGraph::new();
        let c = Concept::new("Test", "Def", BelongsToDomain::Yes);
        let _node_id = graph.add_node(&c);

        let canvas = GraphCanvas::new(
            &graph,
            None,
            CanvasViewport::default(),
            true,
            false,
            TestMsg::Selected,
            TestMsg::Moved,
            TestMsg::CanvasDoubleClicked,
            TestMsg::NodeDoubleClicked,
            TestMsg::ViewportChanged,
        );

        let mut state = GraphCanvasState::default();
        let bounds = Rectangle::new(Point::ORIGIN, Size::new(800.0, 600.0));

        // 1. Dobbeltklik på tomt canvas (x: 400, y: 300)
        let cursor = mouse::Cursor::Available(Point::new(400.0, 300.0));
        let event = Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left));

        // Første klik
        let action1 = canvas.update(&mut state, &event, bounds, cursor);
        assert!(action1.is_some());

        // Andet klik hurtigt efter (dobbeltklik)
        let action2 = canvas.update(&mut state, &event, bounds, cursor);
        assert!(action2.is_some());

        // 2. Dobbeltklik på node (x: 100, y: 80)
        let mut node_state = GraphCanvasState::default();
        let node_cursor = mouse::Cursor::Available(Point::new(100.0, 80.0));

        // Første klik på node
        let action_node1 = canvas.update(&mut node_state, &event, bounds, node_cursor);
        assert!(action_node1.is_some());

        // Andet klik på node (dobbeltklik)
        let action_node2 = canvas.update(&mut node_state, &event, bounds, node_cursor);
        assert!(action_node2.is_some());
    }

    #[test]
    fn test_canvas_viewport_math() {
        let mut vp = CanvasViewport::default();
        assert_eq!(vp.zoom(), 1.0);
        assert_eq!(vp.pan(), Vector::new(0.0, 0.0));

        // Translate
        vp.translate(Vector::new(100.0, 50.0));
        assert_eq!(vp.pan(), Vector::new(100.0, 50.0));

        // To world & to screen
        let pt = Point::new(150.0, 70.0);
        let world = vp.to_world(pt);
        assert_eq!(world, Point::new(50.0, 20.0));
        let screen = vp.to_screen(world);
        assert_eq!(screen, pt);

        // Snap to grid
        let snapped = CanvasViewport::snap_to_grid(Point::new(43.0, 79.0), 20.0);
        assert_eq!(snapped, Point::new(40.0, 80.0));
    }
}
