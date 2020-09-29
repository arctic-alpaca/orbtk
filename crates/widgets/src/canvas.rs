use super::behaviors::{MouseBehavior, CanvasBehavior};

use crate::{api::prelude::*, prelude::*, proc_macros::*, theme::prelude::*};

widget!(
    /// Canvas is used to render 3D graphics.
    Canvas: MouseHandler, KeyDownHandler, KeyUpHandler {
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
            .child(CanvasBehavior::new()
                .target(id.0)
                .focused(id)
                .request_focus(id)
                .build(ctx)
            )
            .child(
                MouseBehavior::new()
                .pressed(id)
                .enabled(id)
                .target(id.0)
                .build(ctx),
            )

    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        PipelineRenderObject.into()
    }
}
