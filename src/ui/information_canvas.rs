use crate::features::concept_model::NodeId;
use crate::features::information_model::{
    ClassDiagramEdge, ClassDiagramNode, ClassGraph, InformationModel,
};
pub use crate::ui::diagram_canvas::{
    render_uml_class_node, CanvasViewport, ClickRecord, DiagramCanvas,
    DiagramCanvasState as InformationCanvasState,
};
use iced::mouse;
use iced::widget::canvas::{Action, Event, Frame, Geometry, Program};
use iced::{Rectangle, Renderer, Theme};

type ClassNodeRenderer<'a> = Box<dyn Fn(&mut Frame, &ClassDiagramNode, bool, CanvasViewport) + 'a>;
type InnerDiagramCanvas<'a, Message> =
    DiagramCanvas<'a, Message, ClassDiagramNode, ClassDiagramEdge, ClassNodeRenderer<'a>>;

/// Bakudkompatibel wrapper for InformationCanvas baseret på den unificerede DiagramCanvas
pub struct InformationCanvas<'a, Message> {
    inner: InnerDiagramCanvas<'a, Message>,
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
        let renderer = Box::new(
            move |frame: &mut Frame,
                  node: &ClassDiagramNode,
                  is_selected: bool,
                  vp: CanvasViewport| {
                let class_opt = model.get_class(node.class_id());
                let class_name = class_opt.map(|c| c.name()).unwrap_or("Ukendt Klasse");
                let attributes: Vec<(String, String, String)> = class_opt
                    .map(|c| {
                        c.attributes()
                            .iter()
                            .map(|a| {
                                (
                                    a.name().to_string(),
                                    a.data_type().as_str().to_string(),
                                    a.multiplicity().to_string(),
                                )
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                render_uml_class_node(frame, node, class_name, &attributes, false, is_selected, vp);
            },
        );

        Self {
            inner: DiagramCanvas::new(
                graph.nodes(),
                graph.edges(),
                selected_node_id,
                viewport,
                snap_to_grid,
                is_space_pressed,
                renderer,
                on_node_selected,
                on_node_moved,
                on_canvas_double_clicked,
                on_node_double_clicked,
                on_viewport_changed,
            ),
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
