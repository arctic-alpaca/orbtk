use crate::{api::prelude::*, proc_macros::*};
use super::behaviors::MouseBehavior;

widget!(
    /// Canvas is used to render 3D graphics.
    Canvas: MouseHandler, KeyDownHandler, KeyUpHandler, ActivateHandler {
        /// Sets or shares the render pipeline.
        render_pipeline: DefaultRenderPipeline,

        /// Sets or shares the pressed property.
        pressed: bool,

        /// Sets or shares the focused property.
        focused: bool,

        /// Used to request focus from outside. Set to `true` tor request focus.
        request_focus: bool
    }
);

impl Template for Canvas {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("Canvas").style("canvas-three")
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        PipelineRenderObject.into()
    }
}
