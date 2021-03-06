use crate::{utils::*, PipelineTrait, RenderConfig, RenderTarget, TextMetrics};

use font_kit::handle::Handle;
use pathfinder_canvas::{
    ArcDirection, Canvas, CanvasFontContext, CanvasRenderingContext2D, FillRule, FillStyle, Path2D,
    RectF, TextBaseline,
};
use pathfinder_color::ColorU;
use pathfinder_geometry::vector::{vec2f, Vector2F};
use pathfinder_gl::GLDevice;
use pathfinder_renderer::concurrent::rayon::RayonExecutor;
use pathfinder_renderer::concurrent::scene_proxy::SceneProxy;
use pathfinder_renderer::gpu::options::RendererOptions;
use pathfinder_renderer::gpu::renderer::Renderer;
use pathfinder_renderer::options::BuildOptions;

pub use self::image::*;

#[path = "../raqote/image.rs"]
mod image;

// #[derive(Clone, Default, Debug)]
// pub struct Image {}

// impl Image {
//     pub fn width(&self) -> f64 {
//         0.0
//     }

//     /// Gets the height.
//     pub fn height(&self) -> f64 {
//         0.0
//     }
// }

// impl From<(u32, u32, Vec<u32>)> for Image {
//     fn from(image: (u32, u32, Vec<u32>)) -> Self {
//         Image {}
//     }
// }

pub struct Font {}

/// The RenderContext2D trait, provides the rendering ctx. It is used for drawing shapes, text, images, and other objects.
pub struct RenderContext2D {
    renderer: Option<Renderer<GLDevice>>,
    font_context: Option<CanvasFontContext>,
    scene: Option<SceneProxy>,
    canvas: Vec<CanvasRenderingContext2D>,
    path: Path2D,
    size: (f64, f64),
    _origin_size: (f64, f64),
    config: RenderConfig,
    device_pixel_ratio: f32,
    saved_config: Option<RenderConfig>,
}

impl RenderContext2D {
    /// Creates a new render ctx 2d.
    pub fn new(width: f64, height: f64) -> Self {
        RenderContext2D {
            renderer: None,
            font_context: None,
            scene: None,
            canvas: vec![],
            path: Path2D::new(),
            size: (width, height),
            _origin_size: (width, height),
            device_pixel_ratio: 1.0,
            config: RenderConfig::default(),
            saved_config: None,
        }
    }

    fn canvas(&mut self) -> &mut CanvasRenderingContext2D {
        self.canvas.get_mut(0).unwrap()
    }

    /// Set the background of the render context.
    pub fn set_background(&mut self, background: Color) {
        if let Some(renderer) = &mut self.renderer {
            renderer.set_options(RendererOptions {
                background_color: Some(
                    ColorU::new(
                        background.r(),
                        background.g(),
                        background.b(),
                        background.a(),
                    )
                    .to_f32(),
                ),
            })
        }
    }

    pub fn new_ex(
        origin_size: (f64, f64),
        size: (f64, f64),
        renderer: Renderer<GLDevice>,
        font_handles: Vec<Handle>,
    ) -> Self {
        let font_context = CanvasFontContext::from_fonts(font_handles.iter().cloned());

        let device_pixel_ratio = size.0 as f32 / origin_size.0 as f32;

        let canvas = Canvas::new(Vector2F::new(size.0 as f32, size.1 as f32))
            .get_context_2d(font_context.clone());

        // canvas.set_text_baseline(TextBaseline::Top);

        RenderContext2D {
            renderer: Some(renderer),
            font_context: Some(font_context),
            scene: Some(SceneProxy::new(RayonExecutor)),
            canvas: vec![canvas],
            path: Path2D::new(),
            size,
            _origin_size: origin_size,
            device_pixel_ratio,
            config: RenderConfig::default(),
            saved_config: None,
        }
    }

    pub fn resize(&mut self, _width: f64, _height: f64) {
        // if let Some(renderer) = &mut self.renderer {
        //     renderer.replace_dest_framebuffer(DestFramebuffer::full_window(vec2i(
        //         width as i32,
        //         height as i32,
        //     )));
        // }

        // if let Some(font_context) = &self.font_context {
        //     self.canvas.clear();
        //     self.canvas.push(
        //         Canvas::new(Vector2F::new(width as f32, height as f32))
        //             .get_context_2d(font_context.clone()),
        //     )
        // }
    }

    /// Registers a new font file.
    pub fn register_font(&mut self, family: &str, font_file: &'static [u8]) {}

    // Rectangles

    /// Draws a filled rectangle whose starting point is at the coordinates {x, y} with the specified width and height and whose style is determined by the fillStyle attribute.
    pub fn fill_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        let device_pixel_ratio = self.device_pixel_ratio();
        self.canvas().fill_rect(RectF::new(
            Vector2F::new(x as f32, y as f32) * device_pixel_ratio,
            Vector2F::new(width as f32, height as f32) * device_pixel_ratio,
        ));
    }

    pub fn device_pixel_ratio(&self) -> f32 {
        self.device_pixel_ratio
    }

    /// Draws a rectangle that is stroked (outlined) according to the current strokeStyle and other ctx settings.
    pub fn stroke_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        let device_pixel_ratio = self.device_pixel_ratio();
        self.canvas().stroke_rect(RectF::new(
            Vector2F::new(x as f32, y as f32) * device_pixel_ratio,
            Vector2F::new(width as f32, height as f32) * device_pixel_ratio,
        ));
    }

    // Text

    /// Draws (fills) a given text at the given (x, y) position.
    pub fn fill_text(&mut self, text: &str, x: f64, y: f64) {
        let device_pixel_ratio = self.device_pixel_ratio();

        let t_m = self.canvas().measure_text(text);

        self.canvas().fill_text(
            text,
            vec2f(x as f32, y as f32 + t_m.actual_bounding_box_ascent) * device_pixel_ratio,
        );
    }

    pub fn measure(
        &mut self,
        text: &str,
        font_size: f64,
        family: impl Into<String>,
    ) -> TextMetrics {
        self.set_font_family(family);
        self.canvas().set_font_size(font_size as f32);
        let t_m = self.canvas().measure_text(text);
        TextMetrics {
            width: t_m.width as f64,
            height: t_m.actual_bounding_box_ascent as f64,
        }
    }

    /// Returns a TextMetrics object.
    pub fn measure_text(&mut self, text: &str) -> TextMetrics {
        let t_m = self.canvas().measure_text(text);
        TextMetrics {
            width: t_m.width as f64,
            height: t_m.actual_bounding_box_ascent as f64,
        }
    }

    /// Fills the current or given path with the current file style.
    pub fn fill(&mut self) {
        let path = self.path.clone();
        self.canvas().fill_path(path, FillRule::Winding);
    }

    /// Strokes {outlines} the current or given path with the current stroke style.
    pub fn stroke(&mut self) {
        let path = self.path.clone();
        self.canvas().stroke_path(path);
    }

    /// Starts a new path by emptying the list of sub-paths. Call this when you want to create a new path.
    pub fn begin_path(&mut self) {
        self.path = Path2D::new();
    }

    /// Attempts to add a straight line from the current point to the start of the current sub-path. If the shape has already been closed or has only one point, this function does nothing.
    pub fn close_path(&mut self) {
        self.path.close_path();
    }

    /// Adds a rectangle to the current path.
    pub fn rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        let device_pixel_ratio = self.device_pixel_ratio();
        self.path.rect(
            RectF::new(
                Vector2F::new(x as f32, y as f32),
                Vector2F::new(width as f32, height as f32),
            ) * device_pixel_ratio,
        );
    }

    /// Creates a circular arc centered at (x, y) with a radius of radius. The path starts at startAngle and ends at endAngle.
    pub fn arc(&mut self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64) {
        let device_pixel_ratio = self.device_pixel_ratio();
        self.path.arc(
            Vector2F::new(x as f32, y as f32) * device_pixel_ratio,
            radius as f32 * device_pixel_ratio,
            start_angle as f32 * device_pixel_ratio,
            end_angle as f32 * device_pixel_ratio,
            ArcDirection::CW,
        )
    }

    /// Begins a new sub-path at the point specified by the given {x, y} coordinates.

    pub fn move_to(&mut self, x: f64, y: f64) {
        let device_pixel_ratio = self.device_pixel_ratio();
        let x_a = x as f32 * device_pixel_ratio;

        self.path.move_to(Vector2F::new(
            x as f32 * device_pixel_ratio,
            y as f32 * device_pixel_ratio,
        ));
    }

    /// Adds a straight line to the current sub-path by connecting the sub-path's last point to the specified {x, y} coordinates.
    pub fn line_to(&mut self, x: f64, y: f64) {
        let device_pixel_ratio = self.device_pixel_ratio();
        self.path
            .line_to(Vector2F::new(x as f32, y as f32) * device_pixel_ratio);
    }

    /// Adds a quadratic Bézier curve to the current sub-path.
    pub fn quadratic_curve_to(&mut self, cpx: f64, cpy: f64, x: f64, y: f64) {
        let device_pixel_ratio = self.device_pixel_ratio();
        self.path.quadratic_curve_to(
            Vector2F::new(cpx as f32, cpy as f32) * device_pixel_ratio,
            Vector2F::new(x as f32, y as f32) * device_pixel_ratio,
        );
    }

    /// Adds a cubic Bézier curve to the current sub-path. It requires three points: the first two are control points and the third one is the end point. The starting point is the latest point in the current path, which can be changed using MoveTo{} before creating the Bézier curve.
    pub fn bezier_curve_to(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        let device_pixel_ratio = self.device_pixel_ratio();
        self.path.bezier_curve_to(
            Vector2F::new(cp1x as f32, cp1y as f32) * device_pixel_ratio,
            Vector2F::new(cp2x as f32, cp2y as f32) * device_pixel_ratio,
            Vector2F::new(x as f32, y as f32) * device_pixel_ratio,
        );
    }

    /// Draws a render target.
    pub fn draw_render_target(&mut self, _render_target: &RenderTarget, _x: f64, _y: f64) {}

    /// Draws the image.
    pub fn draw_image(&mut self, image: &Image, x: f64, y: f64) {}

    /// Draws the given part of the image.
    pub fn draw_image_with_clip(&mut self, _image: &Image, _clip: Rectangle, _x: f64, _y: f64) {}

    pub fn draw_pipeline(
        &mut self,
        _x: f64,
        _y: f64,
        _width: f64,
        _height: f64,
        _pipeline: Box<dyn PipelineTrait>,
    ) {
    }

    /// Creates a clipping path from the current sub-paths. Everything drawn after clip() is called appears inside the clipping path only.
    pub fn clip(&mut self) {
        // let path = self.path.clone();
        // self.canvas().clip_path(path, FillRule::Winding);
    }

    // Line styles

    /// Sets the thickness of lines.
    pub fn set_line_width(&mut self, line_width: f64) {
        let device_pixel_ratio = self.device_pixel_ratio();
        self.canvas()
            .set_line_width(line_width as f32 * device_pixel_ratio);
    }

    /// Sets the alpha value,
    pub fn set_alpha(&mut self, alpha: f32) {
        self.canvas().set_global_alpha(alpha as f32);
    }

    /// Specific the font family.
    pub fn set_font_family(&mut self, family: impl Into<String>) {
        self.canvas().set_font(family.into().as_str());
    }

    /// Specifies the font size.
    pub fn set_font_size(&mut self, size: f64) {
        let device_pixel_ratio = self.device_pixel_ratio();
        self.canvas()
            .set_font_size(size as f32 * device_pixel_ratio);
    }

    // Fill and stroke styley

    /// Specifies the fill color to use inside shapes.
    pub fn set_fill_style(&mut self, fill_style: Brush) {
        match fill_style {
            Brush::SolidColor(color) => {
                self.canvas
                    .get_mut(0)
                    .unwrap()
                    .set_fill_style(FillStyle::Color(ColorU::new(
                        color.r(),
                        color.g(),
                        color.b(),
                        color.a(),
                    )))
            }
            Brush::LinearGradient { start, end, stops } => {}
        }
    }

    /// Specifies the fill stroke to use inside shapes.
    pub fn set_stroke_style(&mut self, stroke_style: Brush) {
        match stroke_style {
            Brush::SolidColor(color) => {
                self.canvas
                    .get_mut(0)
                    .unwrap()
                    .set_stroke_style(FillStyle::Color(ColorU::new(
                        color.r(),
                        color.g(),
                        color.b(),
                        color.a(),
                    )))
            }
            Brush::LinearGradient { start, end, stops } => {}
        }
    }

    // Transformations

    /// Sets the tranformation.
    pub fn set_transform(
        &mut self,
        _h_scaling: f64,
        _h_skewing: f64,
        _v_skewing: f64,
        _v_scaling: f64,
        _h_moving: f64,
        _v_moving: f64,
    ) {
    }

    // Canvas states

    /// Saves the entire state of the canvas by pushing the current state onto a stack.
    pub fn save(&mut self) {
        self.saved_config = Some(self.config.clone());
    }

    /// Restores the most recently saved canvas state by popping the top entry in the drawing state stack. If there is no saved state, this method does nothing.
    pub fn restore(&mut self) {
        if let Some(config) = &self.saved_config {
            self.config = config.clone();
        }

        self.saved_config = None;
    }

    pub fn clear(&mut self, brush: &Brush) {
        let device_pixel_ratio = self.device_pixel_ratio();
        let size = self.size;
        self.set_fill_style(brush.clone());
        self.canvas().clear_rect(RectF::new(
            Vector2F::new(0.0, 0.0),
            Vector2F::new(size.0 as f32, size.1 as f32) * device_pixel_ratio,
        ));
    }

    pub fn start(&mut self) {
        self.path = Path2D::new();
        if !self.canvas.is_empty() {
            return;
        }
        self.canvas.clear();

        if let Some(font_context) = &self.font_context {
            let mut canvas = Canvas::new(Vector2F::new(self.size.0 as f32, self.size.1 as f32))
                .get_context_2d(font_context.clone());
            canvas.set_text_baseline(TextBaseline::Top);
            self.canvas.push(
                Canvas::new(Vector2F::new(self.size.0 as f32, self.size.1 as f32))
                    .get_context_2d(font_context.clone()),
            )
        }
    }

    pub fn finish(&mut self) {
        let canvas = self.canvas.pop().unwrap();

        if let Some(scene) = &mut self.scene {
            if let Some(renderer) = &mut self.renderer {
                scene.replace_scene(canvas.into_canvas().into_scene());
                scene.build_and_render(renderer, BuildOptions::default());
            }
        }

        if let Some(font_context) = &self.font_context {
            let mut canvas = Canvas::new(Vector2F::new(self.size.0 as f32, self.size.1 as f32))
                .get_context_2d(font_context.clone());
            canvas.set_text_baseline(TextBaseline::Top);
            self.canvas.push(
                Canvas::new(Vector2F::new(self.size.0 as f32, self.size.1 as f32))
                    .get_context_2d(font_context.clone()),
            )
        }
    }
}

// --- Conversions ---

// impl From<&str> for Image {
//     fn from(s: &str) -> Image {
//         Image {}
//     }
// }

// impl From<String> for Image {
//     fn from(s: String) -> Image {
//         Image {}
//     }
// }

// --- Conversions ---
