use crate::features::concept_model::{DiagramEdge, DiagramNode, NodeId, GRID_SIZE};
use crate::features::information_model::{ClassGraph, InformationModel};
use crate::ui::edge_router::EdgeRouter;
use crate::ui::graph_canvas::{CanvasViewport, ClickRecord};
use crate::ui::theme::ThemeColors;
use iced::mouse;
use iced::widget::canvas::{Action, Event, Frame, Geometry, Path, Program, Stroke, Text};
use iced::{alignment, Color, Point, Rectangle, Renderer, Size, Theme, Vector};
use std::time::Instant;

#[derive(Debug, Default)]
pub struct InformationCanvasState {
    dragging_node: Option<(NodeId, Vector)>,
    last_click: Option<ClickRecord>,
    panning_start: Option<Point>,
    is_panning_space: bool,
    space_pressed: bool,
    modifiers: iced::keyboard::Modifiers,
}

pub struct InformationCanvas<'a, Message> {
    graph: &'a ClassGraph,
    model: &'a InformationModel,
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

impl<'a, Message> InformationCanvas<'a, Message> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        graph: &'a ClassGraph,
        model: &'a InformationModel,
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
            model,
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

impl<'a, Message> Program<Message, Theme, Renderer> for InformationCanvas<'a, Message> {
    type State = InformationCanvasState;

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
                        return Some(Action::publish((self.on_viewport_changed)(new_vp)));
                    }
                } else {
                    let mut new_vp = self.viewport;
                    new_vp.translate(Vector::new(dx, dy));
                    return Some(Action::publish((self.on_viewport_changed)(new_vp)));
                }
                None
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Middle)) => {
                state.panning_start = Some(cursor_pos);
                None
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Middle)) => {
                state.panning_start = None;
                None
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if state.space_pressed || self.is_space_pressed {
                    state.is_panning_space = true;
                    state.panning_start = Some(cursor_pos);
                    return None;
                }

                let world_pos = self.viewport.to_world(cursor_pos);
                let mut hit_node = None;

                for node in self.graph.nodes().iter().rev() {
                    if node.contains(world_pos.x, world_pos.y) {
                        hit_node = Some(node);
                        break;
                    }
                }

                let is_double_click = if let Some(last) = state.last_click {
                    let dist = ((last.position.x - cursor_pos.x).powi(2)
                        + (last.position.y - cursor_pos.y).powi(2))
                    .sqrt();
                    last.time.elapsed().as_millis() < 400 && dist < 8.0
                } else {
                    false
                };

                state.last_click = Some(ClickRecord {
                    position: cursor_pos,
                    time: Instant::now(),
                });

                if let Some(node) = hit_node {
                    if is_double_click {
                        return Some(Action::publish((self.on_node_double_clicked)(node.id())));
                    }

                    let offset = Vector::new(world_pos.x - node.x(), world_pos.y - node.y());
                    state.dragging_node = Some((node.id(), offset));
                    Some(Action::publish((self.on_node_selected)(Some(node.id()))))
                } else {
                    if is_double_click {
                        return Some(Action::publish((self.on_canvas_double_clicked)(
                            world_pos.x,
                            world_pos.y,
                        )));
                    }
                    Some(Action::publish((self.on_node_selected)(None)))
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                state.dragging_node = None;
                if state.is_panning_space {
                    state.is_panning_space = false;
                    state.panning_start = None;
                }
                None
            }
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                let current_pos = *position;

                if let Some(start) = state.panning_start {
                    let delta = Vector::new(current_pos.x - start.x, current_pos.y - start.y);
                    state.panning_start = Some(current_pos);
                    let mut new_vp = self.viewport;
                    new_vp.translate(delta);
                    return Some(Action::publish((self.on_viewport_changed)(new_vp)));
                }

                if let Some((node_id, offset)) = state.dragging_node {
                    let world_pos = self.viewport.to_world(current_pos);
                    let mut new_x = world_pos.x - offset.x;
                    let mut new_y = world_pos.y - offset.y;

                    if self.snap_to_grid {
                        let snapped =
                            CanvasViewport::snap_to_grid(Point::new(new_x, new_y), GRID_SIZE);
                        new_x = snapped.x;
                        new_y = snapped.y;
                    }

                    Some(Action::publish((self.on_node_moved)(node_id, new_x, new_y)))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        // 1. Grid rendering
        let top_left_world = self.viewport.to_world(Point::ORIGIN);
        let bottom_right_world = self
            .viewport
            .to_world(Point::new(bounds.width, bounds.height));

        let step = GRID_SIZE;
        let start_x = (top_left_world.x / step).floor() * step;
        let start_y = (top_left_world.y / step).floor() * step;
        let max_x = (bottom_right_world.x / step).ceil() * step;
        let max_y = (bottom_right_world.y / step).ceil() * step;

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

        // 2. Transformer resten af tegningen via frame transformation
        frame.translate(self.viewport.pan());
        frame.scale(self.viewport.zoom());

        // 3. Konverter ClassDiagramNode/ClassDiagramEdge til routing-format
        let diagram_nodes: Vec<DiagramNode> = self
            .graph
            .nodes()
            .iter()
            .map(|cn| {
                let label = self
                    .model
                    .get_class(cn.class_id())
                    .map(|c| c.name().to_string())
                    .unwrap_or_else(|| "Klasse".to_string());
                DiagramNode::custom(cn.id(), label, cn.x(), cn.y(), cn.width(), cn.height())
            })
            .collect();

        let diagram_edges: Vec<DiagramEdge> = self
            .graph
            .edges()
            .iter()
            .map(|ce| {
                DiagramEdge::with_label(
                    ce.from(),
                    ce.to(),
                    ce.kind(),
                    ce.label().map(|s| s.to_string()),
                )
            })
            .collect();

        let routed_edges = EdgeRouter::route_edges(&diagram_nodes, &diagram_edges);

        // A. Tegn ortogonale linjeforløb med krydsningsbroer
        for routed in &routed_edges {
            if routed.points.len() < 2 {
                continue;
            }

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
                    .with_color(Color::from_rgb(0.3, 0.3, 0.3))
                    .with_width(1.5),
            );
        }

        // 4. Tegn 3-sektions UML Klassekasser jf. FDA Modelreglerne
        for node in self.graph.nodes() {
            let class_opt = self.model.get_class(node.class_id());
            let top_left = Point::new(node.x(), node.y());
            let size = Size::new(node.width(), node.height());
            let radius = 6.0;

            // Skygge
            let shadow_path =
                Path::rounded_rectangle(Point::new(node.x(), node.y() + 2.0), size, radius.into());
            frame.fill(&shadow_path, Color::from_rgba(0.05, 0.1, 0.2, 0.07));

            // Hele boksens baggrund (FDA Sand: #FEFAF7)
            let node_path = Path::rounded_rectangle(top_left, size, radius.into());
            frame.fill(&node_path, ThemeColors::FDA_SAND);

            // Ramme
            let is_selected = self.selected_node_id == Some(node.id());
            let (border_color, border_width) = if is_selected {
                (ThemeColors::PRIMARY, 2.5)
            } else {
                (ThemeColors::FDA_SAND_BORDER, 1.2)
            };

            frame.stroke(
                &node_path,
                Stroke::default()
                    .with_color(border_color)
                    .with_width(border_width),
            );

            // Stereotype: «Informationsklasse»
            frame.fill_text(Text {
                content: "«Informationsklasse»".to_string(),
                position: Point::new(node.x() + node.width() / 2.0, node.y() + 14.0),
                color: ThemeColors::SLATE_500,
                size: 10.5.into(),
                align_x: alignment::Horizontal::Center.into(),
                align_y: alignment::Vertical::Center,
                ..Default::default()
            });

            // Klassenavn (Fed skrift)
            let class_name = class_opt.map(|c| c.name()).unwrap_or("Ukendt Klasse");
            frame.fill_text(Text {
                content: class_name.to_string(),
                position: Point::new(node.x() + node.width() / 2.0, node.y() + 32.0),
                color: ThemeColors::SLATE_900,
                size: 14.0.into(),
                align_x: alignment::Horizontal::Center.into(),
                align_y: alignment::Vertical::Center,
                ..Default::default()
            });

            // Horisontal skillelinje under header
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

            // Attribut-sektion
            let mut attr_y = divider_y + 14.0;
            if let Some(class) = class_opt {
                if class.attributes().is_empty() {
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
                    for attr in class.attributes() {
                        let line_str = format!(
                            "+ {} : {} [{}]",
                            attr.name(),
                            attr.data_type().as_str(),
                            attr.multiplicity()
                        );
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
        }

        // 5. Tegn Pilehoveder og Labels ovenpå noder
        for routed in &routed_edges {
            if let Some(ref arrow) = routed.arrow_head {
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
                        .with_color(Color::from_rgb(0.2, 0.2, 0.2))
                        .with_width(1.5),
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

                    frame.fill_text(Text {
                        content: label_text.to_string(),
                        position: pos,
                        color: ThemeColors::PRIMARY,
                        size: 11.0.into(),
                        align_x: alignment::Horizontal::Center.into(),
                        align_y: alignment::Vertical::Center,
                        ..Default::default()
                    });
                }
            }
        }

        vec![frame.into_geometry()]
    }
}
