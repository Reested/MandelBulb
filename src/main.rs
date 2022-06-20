extern crate kiss3d;
extern crate nalgebra as na;
extern crate pbr;

use kiss3d::camera::Camera;
use kiss3d::context::Context;
use kiss3d::planar_camera::PlanarCamera;
use kiss3d::post_processing::PostProcessingEffect;
use kiss3d::renderer::Renderer;
use kiss3d::resource::{
    AllocationType, BufferType, Effect, GPUVec, ShaderAttribute, ShaderUniform,
};
use kiss3d::text::Font;
use kiss3d::window::{State, Window};
use na::{Matrix4, Point2, Point3};

use pbr::ProgressBar;

const DIMENSIONS: i32 = 256;
const MAX_ITER: i32 = 20;

#[link(name = "math", kind = "static")]
extern "C" {
    fn c_sqrt(x: f32) -> f32;
    fn c_atan2(x: f32, y: f32) -> f32;
    fn c_pow(x: f32, y: f32) -> f32;
    fn c_sin(x: f32) -> f32;
    fn c_cos(x: f32) -> f32;
    fn c_remap(val: f32, init_low: f32, init_high: f32, new_low: f32, new_high: f32) -> f32;
}

fn sqrt(x: f32) -> f32 {
    unsafe { c_sqrt(x) }
}
fn atan2(x: f32, y: f32) -> f32 {
    unsafe { c_atan2(x, y) }
}
fn pow(x: f32, y: f32) -> f32 {
    unsafe { c_pow(x, y) }
}
fn sin(x: f32) -> f32 {
    unsafe { c_sin(x) }
}
fn cos(x: f32) -> f32 {
    unsafe { c_cos(x) }
}
fn remap(val: f32, init_low: f32, init_high: f32, new_low: f32, new_high: f32) -> f32 {
    unsafe { c_remap(val, init_low, init_high, new_low, new_high) }
}

const VERTEX_SHADER_SRC: &str = "#version 100
    attribute vec3 position;
    attribute vec3 color;
    varying   vec3 Color;
    uniform   mat4 proj;
    uniform   mat4 view;
    void main() {
        gl_Position = proj * view * vec4(position, 1.0);
        Color = color;
    }";

const FRAGMENT_SHADER_SRC: &str = "#version 100
#ifdef GL_FRAGMENT_PRECISION_HIGH
   precision highp float;
#else
   precision mediump float;
#endif
    varying vec3 Color;
    void main() {
        gl_FragColor = vec4(Color, 1.0);
    }";

#[derive(Copy, Clone)]
struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

struct Polar {
    r: f32,
    theta: f32,
    phi: f32,
}

impl Polar {
    fn new(x: f32, y: f32, z: f32) -> Polar {
        let r: f32 = sqrt(x * x + y * y + z * z);
        let theta: f32 = atan2(sqrt(x * x + y * y), z);
        let phi: f32 = atan2(y, x);
        Polar { r, theta, phi }
    }
}

struct AppState {
    mandlebulb_renderer: MandleBulbRenderer,
    mandlebulb: Vec<Vector>,
    done_processing: bool,
}

impl State for AppState {
    // Return the custom renderer that will be called at each
    // render loop.
    fn cameras_and_effect_and_renderer(
        &mut self,
    ) -> (
        Option<&mut dyn Camera>,
        Option<&mut dyn PlanarCamera>,
        Option<&mut dyn Renderer>,
        Option<&mut dyn PostProcessingEffect>,
    ) {
        (None, None, Some(&mut self.mandlebulb_renderer), None)
    }

    fn step(&mut self, window: &mut Window) {
        if !self.done_processing {
            let mut pb = ProgressBar::new(DIMENSIONS.try_into().unwrap());
            pb.format("╢▌▌░╟");
            pb.show_message = true;
            pb.message("Setting Points : ");
            for iterator in 0..self.mandlebulb.len() - 1 {
                let color_map: f32 =
                    remap(iterator as f32, 0., self.mandlebulb.len() as f32, 0., 1.);
                let set_color: Point3<f32> = Point3::new(0.1, 1., color_map);
                self.mandlebulb_renderer
                    .push(to_point3(self.mandlebulb[iterator]), set_color);
                pb.inc();
            }
            pb.finish_print("Finished Setting Points.");
            self.done_processing = true;
        }

        let num_points_text = format!(
            "Number of points: {}",
            self.mandlebulb_renderer.num_points()
        );
        window.draw_text(
            &num_points_text,
            &Point2::new(0.0, 20.0),
            60.0,
            &Font::default(),
            &Point3::new(1.0, 1.0, 1.0),
        );
    }
}

struct MandleBulbRenderer {
    shader: Effect,
    pos: ShaderAttribute<Point3<f32>>,
    color: ShaderAttribute<Point3<f32>>,
    proj: ShaderUniform<Matrix4<f32>>,
    view: ShaderUniform<Matrix4<f32>>,
    colored_points: GPUVec<Point3<f32>>,
    point_size: f32,
}

impl MandleBulbRenderer {
    /// Creates a new points renderer.
    fn new(point_size: f32) -> MandleBulbRenderer {
        let mut shader = Effect::new_from_str(VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC);

        shader.use_program();

        MandleBulbRenderer {
            colored_points: GPUVec::new(Vec::new(), BufferType::Array, AllocationType::StreamDraw),
            pos: shader.get_attrib::<Point3<f32>>("position").unwrap(),
            color: shader.get_attrib::<Point3<f32>>("color").unwrap(),
            proj: shader.get_uniform::<Matrix4<f32>>("proj").unwrap(),
            view: shader.get_uniform::<Matrix4<f32>>("view").unwrap(),
            shader,
            point_size,
        }
    }

    fn push(&mut self, point: Point3<f32>, color: Point3<f32>) {
        if let Some(colored_points) = self.colored_points.data_mut() {
            colored_points.push(point);
            colored_points.push(color);
        }
    }

    fn num_points(&self) -> usize {
        self.colored_points.len() / 2
    }
}

impl Renderer for MandleBulbRenderer {
    /// Actually draws the points.
    fn render(&mut self, pass: usize, camera: &mut dyn Camera) {
        if self.colored_points.len() == 0 {
            return;
        }

        self.shader.use_program();
        self.pos.enable();
        self.color.enable();

        camera.upload(pass, &mut self.proj, &mut self.view);

        self.color.bind_sub_buffer(&mut self.colored_points, 1, 1);
        self.pos.bind_sub_buffer(&mut self.colored_points, 1, 0);

        let ctxt = Context::get();
        ctxt.point_size(self.point_size);
        ctxt.draw_arrays(Context::POINTS, 0, (self.colored_points.len() / 2) as i32);

        self.pos.disable();
        self.color.disable();
    }
}

fn create_bulb() -> Vec<Vector> {
    let mut pb = ProgressBar::new(DIMENSIONS.try_into().unwrap());
    pb.tick_format("|/-\\");
    pb.format("╢▌▌░╟");
    pb.show_message = true;
    pb.message("Calculating Points : ");

    let mut mandlebub: Vec<Vector> = Vec::new();

    for i in 0..DIMENSIONS - 1 {
        for j in 0..DIMENSIONS - 1 {
            let mut edge: bool = false;
            for k in 0..DIMENSIONS - 1 {
                let x: f32 = remap(i as f32, 0.0, DIMENSIONS as f32, -1.0, 1.0);
                let y: f32 = remap(j as f32, 0.0, DIMENSIONS as f32, -1.0, 1.0);
                let z: f32 = remap(k as f32, 0.0, DIMENSIONS as f32, -1.0, 1.0);

                let mut zeta: Vector = Vector {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };

                let n: f32 = 8.0;
                let mut iter: i32 = 0;

                loop {
                    let polar: Polar = Polar::new(zeta.x, zeta.y, zeta.z);

                    zeta.x = x + (pow(polar.r, n) * sin(polar.theta * n) * cos(polar.phi * n));
                    zeta.y = y + (pow(polar.r, n) * sin(polar.theta * n) * sin(polar.phi * n));
                    zeta.z = z + (pow(polar.r, n) * cos(polar.theta * n));

                    iter += 1;

                    if polar.r > 2.0 {
                        if edge {
                            edge = false;
                        }
                        break;
                    }

                    if iter > MAX_ITER {
                        if !edge {
                            edge = true;
                            mandlebub.push(Vector { x, y, z });
                        }
                        break;
                    }
                }
            }
        }
        pb.inc();
    }
    pb.finish_print("Points Calculated.");
    mandlebub
}

fn to_point3(v: Vector) -> Point3<f32> {
    Point3::new(v.x, v.y, v.z)
}

fn main() {
    let bulb = create_bulb();

    let window = Window::new("MandleBulb");
    let app = AppState {
        mandlebulb_renderer: MandleBulbRenderer::new(0.0),
        mandlebulb: bulb,
        done_processing: false,
    };

    window.render_loop(app)
}
