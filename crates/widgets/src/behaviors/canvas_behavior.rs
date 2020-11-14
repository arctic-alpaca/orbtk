use crate::{
    api::prelude::*,
    proc_macros::*,
};

/// Actions of CanvasBehaviorState
#[derive(Clone)]
pub enum CanvasAction {
    MouseDown(Mouse),
    Drop(String, Point),
    FocusedChanged,
}


/// The `CanvasBehaviorState` handles the text processing of the `CanvasBehavior` widget.
#[derive(Default, AsAny)]
pub struct CanvasBehaviorState {
    target: Entity,
    pressed: bool,
    event_adapter: EventAdapter,
    window: Entity,
}

impl CanvasBehaviorState {

    fn request_focus(&self) {
        self.event_adapter.push_event_direct(self.window, FocusEvent::RequestFocus(self.target));
    }


    // handles mouse down event
    fn mouse_down(&mut self, ctx: &mut Context, _mouse: Mouse) {
        self.pressed = true;
        if !*CanvasBehavior::focused_ref(&ctx.widget()) {
            self.request_focus();
            return;
        }
    }

    // gets the focused state
    pub fn focused(&self, ctx: &mut Context) -> bool {
        *CanvasBehavior::focused_ref(&ctx.widget())
    }
}

impl State for CanvasBehaviorState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.target = (*CanvasBehavior::target_ref(&ctx.widget())).into();
        ctx.get_widget(self.target).update(false);
        self.event_adapter = ctx.event_adapter();
        self.window = ctx.entity_of_window();
    }

    fn messages(
        &mut self,
        mut messages: MessageReader,
        _registry: &mut Registry,
        ctx: &mut Context,
    ) {
        for action in messages.read::<CanvasAction>() {
            match action {
                CanvasAction::MouseDown(p) => {
                    self.mouse_down(ctx, p)
                }
                CanvasAction::Drop(_text, position) => {
                    if check_mouse_condition(position, &ctx.get_widget(self.target)) {
                        println!("you need to implement drop")
                    }
                }
                CanvasAction::FocusedChanged => {
                    println!("focus changed canvas_behavior");
                },
            }
        }
    }
}

widget!(
    /// The CanvasBehavior widget implements interactivity (mouse and keyboard input) for the `Canvas`
    ///
    /// Attaching to a widget makes it able to handle mouse and keyboard input.
    /// Basicly it allows a widget to use `.on_...` methods
    ///
    /// CanvasBehavior needs the following prerequisites to able to work:
    /// * a `target`: the [`Entity`] of the target widget
    ///
    /// * and must inherit the following properties from its target:
    ///     * focused
    ///     * request_focus
    ///
    /// # Example
    ///
    /// ```
    /// use orbtk::prelude::*;
    ///
    /// widget!(MyInput {
    ///     // essential properties CanvasBehavior needs to inherit
    ///     focused: bool,
    ///     request_focus: bool,
    /// });
    ///
    /// impl Template for MyInput {
    ///     fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
    ///
    ///        self.name("Canvas").style("canvas-three")
    ///        .child(CanvasBehavior::new()
    ///            .target(id.0)
    ///            .focused(id)
    ///            .request_focus(id)
    ///            .build(ctx)
    ///        )
    ///        .child(
    ///            MouseBehavior::new()
    ///            .pressed(id)
    ///            .enabled(id)
    ///            .target(id.0)
    ///            .build(ctx),
    ///        )
    ///    }
    /// ```
    ///
    /// [`Entity`]: https://docs.rs/dces/0.2.0/dces/entity/struct.Entity.html
    CanvasBehavior<CanvasBehaviorState>: ActivateHandler, KeyUpHandler, KeyDownHandler, DropHandler, MouseHandler {
        /// Reference the target (parent) widget e.g. `Canvas`.
        target: u32,

        /// Sets or shares the focused property.
        focused: bool,

        /// Sets or shares the request_focus property. Used to request focus from outside. Set to `true` to request focus.
        request_focus: bool
    }
);

impl Template for CanvasBehavior {
    fn template(self, id: Entity, _: &mut BuildContext) -> Self {
        self.name("CanvasBehavior")
            .focused(false)
            .on_drop_file(move |ctx, file_name, position| {
                ctx.send_message(CanvasAction::Drop(file_name, position), id);
                false
            })
            .on_drop_text(move |ctx, file_name, position| {
                ctx.send_message(CanvasAction::Drop(file_name, position), id);
                false
            })
            .on_mouse_down(move |ctx, m| {
                ctx.send_message(CanvasAction::MouseDown(m), id);
                false
            })
            .on_changed("focused", move |ctx, _| {
                ctx.send_message(CanvasAction::FocusedChanged, id);
            })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
}
