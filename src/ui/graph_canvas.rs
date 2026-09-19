use crate::features::concept_model::{ConceptGraph, NodeId, RelationKind};
use crate::ui::theme::ThemeColors;
use iced::mouse;
use iced::widget::canvas::{Action, Event, Frame, Geometry, Path, Program, Stroke, Text};
use iced::{alignment, Color, Point, Rectangle, Renderer, Size, Theme, Vector};

#[derive(Debug, Default)]
pub struct GraphCanvasState {
    dragging_node: Option<(NodeId, Vector)>,
}

pub struct GraphCanvas<'a, Message> {
    graph: &'a ConceptGraph,
    selected_node_id: Option<NodeId>,
    on_node_selected: Box<dyn Fn(Option<NodeId>) -> Message + 'a>,
    on_node_moved: Box<dyn Fn(NodeId, f32, f32) -> Message + 'a>,
}

impl<'a, Message> GraphCanvas<'a, Message> {
    pub fn new(
        graph: &'a ConceptGraph,
        selected_node_id: Option<NodeId>,
        on_node_selected: impl Fn(Option<NodeId>) -> Message + 'a,
        on_node_moved: impl Fn(NodeId, f32, f32) -> Message + 'a,
    ) -> Self {
        Self {
            graph,
            selected_node_id,
            on_node_selected: Box::new(on_node_selected),
            on_node_moved: Box::new(on_node_moved),
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
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                // Tjek noder i omvendt rækkefølge (øverste node først)
                for node in self.graph.nodes().iter().rev() {
                    if node.contains(cursor_pos.x, cursor_pos.y) {
                        let offset = Vector::new(cursor_pos.x - node.x(), cursor_pos.y - node.y());
                        state.dragging_node = Some((node.id(), offset));
                        return Some(
                            Action::publish((self.on_node_selected)(Some(node.id()))).and_capture(),
                        );
                    }
                }
                // Klik på tomt lærred fjerner markering
                state.dragging_node = None;
                Some(Action::publish((self.on_node_selected)(None)))
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if let Some((node_id, offset)) = state.dragging_node {
                    let new_x = (cursor_pos.x - offset.x).max(10.0);
                    let new_y = (cursor_pos.y - offset.y).max(10.0);
                    return Some(
                        Action::publish((self.on_node_moved)(node_id, new_x, new_y)).and_capture(),
                    );
                }
                None
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
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
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry<Renderer>> {
        let mut frame = Frame::new(renderer, bounds.size());

        // 1. Tegn UML relationer (edges) først, så de ligger under noderne
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

                            // Træk spidsen tilbage til kanten af noden ca.
                            let target_pt = Point::new(p2.x - ux * (to.width() * 0.35), p2.y - uy * (to.height() * 0.35));
                            let arrow_len = 16.0;
                            let arrow_width = 8.0;

                            let base_pt = Point::new(target_pt.x - ux * arrow_len, target_pt.y - uy * arrow_len);
                            let left_pt = Point::new(base_pt.x - uy * arrow_width, base_pt.y + ux * arrow_width);
                            let right_pt = Point::new(base_pt.x + uy * arrow_width, base_pt.y - ux * arrow_width);

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
                        // Association jf. FDA Tabel 1: Linje med associationsnavn
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
                                    align_y: alignment::Vertical::Center.into(),
                                    ..Default::default()
                                });
                            }
                        }
                    }
                }
            }
        }

        // 2. Tegn Noder jf. FDA Modelreglerne (Kapitel 5 & 7.3)
        for node in self.graph.nodes() {
            let top_left = Point::new(node.x(), node.y());
            let size = Size::new(node.width(), node.height());
            let node_path = Path::rounded_rectangle(top_left, size, 6.0.into());

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
            } else {
                (Color::from_rgb(0.5, 0.5, 0.5), 1.2)
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
                position: Point::new(node.x() + node.width() / 2.0, node.y() + 24.0),
                color: Color::from_rgb(0.1, 0.1, 0.1),
                size: 14.0.into(),
                align_x: alignment::Horizontal::Center.into(),
                align_y: alignment::Vertical::Center.into(),
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
                position: Point::new(node.x() + node.width() / 2.0, node.y() + 48.0),
                color: Color::from_rgb(0.45, 0.45, 0.45),
                size: 10.0.into(),
                align_x: alignment::Horizontal::Center.into(),
                align_y: alignment::Vertical::Center.into(),
                ..Default::default()
            });
        }

        vec![frame.into_geometry()]
    }
}
