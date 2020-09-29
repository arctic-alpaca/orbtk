use std::collections::VecDeque;

use crate::{
    api::prelude::*,
    proc_macros::*,
    render::TextMetrics,
    shell::prelude::{Key, KeyEvent},
    theme::fonts,
    Cursor, TextBlock,
};

// --- KEYS --
pub static FOCUSED_STATE: &str = "focused";
// --- KEYS --

/// Actions of CanvasBehaviorState
#[derive(Clone)]
pub enum CanvasAction {
    MouseDown(Mouse),
    Drop(String, Point),
    FocusedChanged,
    /// Used to force an update on visual state and offset.
    ForceUpdate,
}


/// The `CanvasBehaviorState` handles the text processing of the `CanvasBehavior` widget.
#[derive(Default, AsAny)]
pub struct CanvasBehaviorState {
    action: VecDeque<CanvasAction>,
    target: Entity,
    pressed: bool,
    self_update: bool,
    update_selection: bool,
}

impl CanvasBehaviorState {
    /// Sets an action the the state.
    pub fn action(&mut self, action: CanvasAction) {
        self.action.push_back(action);
    }


    fn request_focus(&self, ctx: &mut Context) {
        ctx.push_event_by_window(FocusEvent::RequestFocus(self.target));
    }


    // handles mouse down event
    fn mouse_down(&mut self, ctx: &mut Context, mouse: Mouse) {
        self.pressed = true;
        if !*CanvasBehavior::focused_ref(&ctx.widget()) {
            self.request_focus(ctx);
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

    }

    fn update(&mut self, registry: &mut Registry, ctx: &mut Context) {
        if let Some(action) = self.action.pop_front() {
            match action {
                CanvasAction::MouseDown(p) => {
                    self.mouse_down(ctx, p)
                }
                CanvasAction::Drop(text, position) => {
                    if check_mouse_condition(position, &ctx.get_widget(self.target)) {
                        println!("you need to implement drop")
                    }
                }
                CanvasAction::FocusedChanged => {
                    println!("focus changed canvas_behavior");
                },
                CanvasAction::ForceUpdate => (),

            }
        }
    }

    fn update_post_layout(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        if self.update_selection {
            self.update_selection = false;
        }
    }
}

widget!(
    /// The CanvasBehavior widget shares the same logic of handling text input between
    /// tex-related widgets.
    ///
    /// Attaching to a widget makes it able to handle text input like:
    /// * input characters by keyboard
    /// * select all text with Ctrl+A key combination
    /// * delete selected text with Backspace or Delete
    /// * move cursor by the left or right arrow keys or clicking with mouse
    /// * delete characters by pressing the Backspace or the Delete key
    /// * run on_activate() callback on pressing the Enter key
    ///
    /// CanvasBehavior needs the following prerequisites to able to work:
    /// * a `cursor`: the [`Entity`] of a [`Cursor`] widget
    /// * a `target`: the [`Entity`] of the target widget
    /// * a `text_block`: the [`Entity`] of the [`TextBlock`] widget
    ///
    /// * and must inherit the following properties from its target:
    ///     * focused
    ///     * font
    ///     * font_size
    ///     * lost_focus_on_activation
    ///     * request_focus
    ///     * text
    ///     * selection
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
    ///         // Cursor depends on a TextBlock
    ///
    ///        CanvasBehavior::new()
    ///            .focused(id)
    ///            .target(id.0)
    ///            .request_focus(id)
    ///            .build(ctx)
    /// }
    /// ```
    ///
    /// [`Entity`]: https://docs.rs/dces/0.2.0/dces/entity/struct.Entity.html
    /// [`Cursor`]: ../struct.Cursor.html
    CanvasBehavior<CanvasBehaviorState>: ActivateHandler, KeyUpHandler, KeyDownHandler, DropHandler, MouseHandler {
        /// Reference the target (parent) widget e.g. `TextBox` or `PasswordBox`.
        target: u32,

        /// Sets or shares the focused property.
        focused: bool,

        /// Sets or shares the request_focus property. Used to request focus from outside.Set to `true` to request focus.
        request_focus: bool
    }
);

impl Template for CanvasBehavior {
    fn template(self, id: Entity, _: &mut BuildContext) -> Self {
        self.name("CanvasBehavior")
            .focused(false)
            .on_drop_file(move |states, file_name, position| {
                states
                    .get_mut::<CanvasBehaviorState>(id)
                    .action(CanvasAction::Drop(file_name, position));
                false
            })
            .on_drop_text(move |states, file_name, position| {
                states
                    .get_mut::<CanvasBehaviorState>(id)
                    .action(CanvasAction::Drop(file_name, position));
                false
            })
            .on_mouse_down(move |states, m| {
                states
                    .get_mut::<CanvasBehaviorState>(id)
                    .action(CanvasAction::MouseDown(m));
                true
            })
            .on_changed("focused", move |states, _| {
                states
                    .get_mut::<CanvasBehaviorState>(id)
                    .action(CanvasAction::FocusedChanged);
            })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
}
