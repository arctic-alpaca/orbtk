use orbtk::prelude::*;
use orbtk::shell::prelude::Key;
use orbtk::widgets::behaviors::{FocusBehaviorState, CanvasBehaviorState};

const CANVAS_ID: &str = "canvas_id";

// OrbTk 2D drawing
#[derive(Clone, Default, PartialEq, Pipeline)]
struct Graphic2DPipeline{
    color: Color,
}

impl RenderPipeline for Graphic2DPipeline {
    fn draw(&self, render_target: &mut RenderTarget) {
        let mut render_context =
            RenderContext2D::new(render_target.width(), render_target.height());


       render_context.set_fill_style(utils::Brush::SolidColor(Color::from("#FFFFFF")));

        render_context.fill_rect(0.0, 0.0, render_target.width(), render_target.width());
        render_target.draw(render_context.data());
    }
}

#[derive(Default, AsAny)]
pub struct MainViewState {
    my_color: Color,
    focused: bool,
    focused_entity: Entity,
}

impl MainViewState {
    fn print_something(&mut self) {
        println!("test");
    }
    fn focused(&self) -> bool{
        self.focused
    }
}


impl State for MainViewState {
    fn init(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        // set initial focus false
        self.focused = false;
        // safe canvas entity to later retrieve focus value
        self.focused_entity = ctx.entity_of_child(CANVAS_ID).unwrap();
        println!("focused_entity: {:#?}", self.focused_entity);
    }

    fn update(&mut self, _registry: &mut Registry, ctx: &mut Context) {
    }

    fn update_post_layout(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        // get the canvas widget container
        let canvas_widget_container = ctx.get_widget(self.focused_entity);
        // get the focused field of the canvas struct
        self.focused = *canvas_widget_container.get::<bool>("focused");
    }
}

widget!(
    MainView<MainViewState> {
         render_pipeline: DefaultRenderPipeline,
         text: String
    }
);

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView")
            .render_pipeline(DefaultRenderPipeline(Box::new(Graphic2DPipeline::default())))
            .child(
                Grid::new()
                    .rows(Rows::create().push("auto").push("auto").push("*"))
                    .child(
                        TextBox::new()
                            .water_mark("TextBox...")
                            .text(("text", id))
                            .margin((0, 8, 0, 0))
                            .attach(Grid::row(1))
                            .build(ctx),
                    )
                    .child(
                        TextBlock::new()
                            .attach(Grid::row(0))
                            .text("Canvas (render with OrbTk)")
                            .style("text-block")
                            .style("text_block_header")
                            .margin(4.0)
                            .build(ctx),
                    )
                    .child(
                        Canvas::new()
                            .id(CANVAS_ID)
                            .attach(Grid::row(2))
                            .render_pipeline(id)
                            .on_click(move |states, point| {
                                if states.get_mut::<MainViewState>(id).focused(){
                                    println!("on_click: {:#?}", point);
                                    println!("on_click_id: {:#?}", id);}
                                true
                            })
                            .on_mouse_move(move |states, point| {
                                if states.get_mut::<MainViewState>(id).focused(){
                                    println!("on_mouse_move: {:#?}", point);
                                    println!("on_mouse_move_id: {:#?}", id);}
                                true
                            })
                            .on_mouse_down(move |states, point| {
                                if states.get_mut::<MainViewState>(id).focused(){
                                    println!("on_mouse_down: {:#?}", point);
                                    println!("on_mouse_down_id: {:#?}", id);
                                }
                                true
                            })
                            .on_mouse_up(move |states, point| {
                                if states.get_mut::<MainViewState>(id).focused(){
                                    println!("on_mouse_up: {:#?}", point);
                                    println!("on_mouse_up_id: {:#?}", id);}
                            })
                            .on_scroll(move |states, point| {
                                if states.get_mut::<MainViewState>(id).focused(){
                                    println!("on_scroll: {:#?}", point);
                                    println!("on_scroll_id: {:#?}", id);}
                                true
                            })
                            .on_key_down(move |states, key_event| {
                                if states.get_mut::<MainViewState>(id).focused(){
                                    println!("on_key_down: {:#?}", key_event);
                                    println!("on_key_down_id: {:#?}", id);}
                                false
                            })
                            .on_key_up(move |states, key_event| {
                                if states.get_mut::<MainViewState>(id).focused(){
                                    println!("on_key_up: {:#?}", key_event);
                                    println!("on_key_up_id: {:#?}", id);}
                                true
                            })
                            .on_changed("focused", move |states, event| {
                                println!("on_changed_focus");
                            })
                            .build(ctx),
                    )
                    .build(ctx),
            )

    }
}

fn main() {
    // use this only if you want to run it as web application.
    orbtk::initialize();

    Application::new()
        .window(|ctx| {
            orbtk::prelude::Window::new()
                .title("OrbTk - canvas example")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .resizeable(true)
                .child(MainView::new().build(ctx))
                .build(ctx)
        })
        .run();
}