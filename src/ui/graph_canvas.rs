use crate::features::concept_model::{ConceptGraph, DiagramEdge, DiagramNode, NodeId};
pub use crate::ui::diagram_canvas::{
    render_concept_node, CanvasViewport, ClickRecord, DiagramCanvas,
    DiagramCanvasState as GraphCanvasState,
};
use iced::mouse;
use iced::widget::canvas::{Action, Event, Geometry, Program};
use iced::{Rectangle, Renderer, Theme};

type ConceptNodeRenderer = fn(&mut iced::widget::canvas::Frame, &DiagramNode, bool, CanvasViewport);
type InnerDiagramCanvas<'a, Message> =
    DiagramCanvas<'a, Message, DiagramNode, DiagramEdge, ConceptNodeRenderer>;

/// Bakudkompatibel wrapper for GraphCanvas baseret på den unificerede DiagramCanvas
pub struct GraphCanvas<'a, Message> {
    inner: InnerDiagramCanvas<'a, Message>,
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
            inner: DiagramCanvas::new(
                graph.nodes(),
                graph.edges(),
                selected_node_id,
                viewport,
                snap_to_grid,
                is_space_pressed,
                render_concept_node,
                on_node_selected,
                on_node_moved,
                on_canvas_double_clicked,
                on_node_double_clicked,
                on_viewport_changed,
            ),
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
        self.inner.update(state, event, bounds, cursor)
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        self.inner.draw(state, renderer, theme, bounds, cursor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::concept_model::ConceptGraph;
    use crate::features::concepts::{BelongsToDomain, Concept};
    use iced::{Point, Size, Vector};

    #[derive(Debug, Clone)]
    #[allow(dead_code)]
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

        vp.translate(Vector::new(100.0, 50.0));
        assert_eq!(vp.pan(), Vector::new(100.0, 50.0));

        let pt = Point::new(150.0, 70.0);
        let world = vp.to_world(pt);
        assert_eq!(world, Point::new(50.0, 20.0));
        let screen = vp.to_screen(world);
        assert_eq!(screen, pt);

        let snapped = CanvasViewport::snap_to_grid(Point::new(43.0, 79.0), 20.0);
        assert_eq!(snapped, Point::new(40.0, 80.0));
    }
}
