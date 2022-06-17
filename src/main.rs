extern crate kiss3d;
extern crate nalgebra as na;

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

#[link(name = "math", kind = "static")]
extern "C" {
    fn c_sqrt(x: f32) -> f32;
    fn c_atan2(x: f32, y: f32) -> f32;
    fn c_pow(x: f32, y: f32) -> f32;
    fn c_sin(x: f32) -> f32;
    fn c_cos(x: f32) -> f32;

    fn c_remap(val: f32, init_low: f32, init_high: f32, new_low: f32, new_high: f32) -> f32;
}

const DIMENSIONS: i32 = 512;
const MAX_ITER: i32 = 20;
static mut MANDLEBULB: Vec<Vector> = Vec::new();
static mut DONE: bool = false;

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
        let r: f32 = unsafe { c_sqrt(x * x + y * y + z * z) };
        let theta: f32 = unsafe { c_atan2(c_sqrt(x * x + y * y), z) };
        let phi: f32 = unsafe { c_atan2(y, x) };
        Polar { r, theta, phi }
    }
}

struct AppState {
    mandlebulb_renderer: MandleBulbRenderer,
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
        // if self.mandlebulb_renderer.num_points() < 1_000_000 {
        //     // Add some random points to the point cloud.
        //     for _ in 0..1_000 {
        //         let random: Point3<f32> = rand::random();
        //         self.mandlebulb_renderer
        //             .push((random - Vector3::repeat(0.5)) * 0.5, rand::random());
        //     }
        // }
        unsafe {
            if !DONE {
                for iterator in 0..MANDLEBULB.len() - 1 {
                    let set_color: Point3<f32> = Point3::new(255., 255., 255.);
                    self.mandlebulb_renderer
                        .push(to_point3(MANDLEBULB[iterator]), set_color);
                    println!("Setting Points {} of {}", iterator, MANDLEBULB.len() - 1);
                }
                DONE = true;
            }
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

fn create_bulb() {
    for i in 0..DIMENSIONS - 1 {
        for j in 0..DIMENSIONS - 1 {
            let mut edge: bool = false;
            for k in 0..DIMENSIONS - 1 {
                let x: f32 = unsafe { c_remap(i as f32, 0.0, DIMENSIONS as f32, -1.0, 1.0) };
                let y: f32 = unsafe { c_remap(j as f32, 0.0, DIMENSIONS as f32, -1.0, 1.0) };
                let z: f32 = unsafe { c_remap(k as f32, 0.0, DIMENSIONS as f32, -1.0, 1.0) };

                let mut zeta: Vector = Vector {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };

                let n: f32 = 8.0;
                let mut iter: i32 = 0;

                loop {
                    let polar: Polar = Polar::new(zeta.x, zeta.y, zeta.z);
                    let new_x: f32 = unsafe {
                        c_pow(polar.r, n) * c_sin(polar.theta * n) * c_cos(polar.phi * n)
                    };
                    let new_y: f32 = unsafe {
                        c_pow(polar.r, n) * c_sin(polar.theta * n) * c_sin(polar.phi * n)
                    };
                    let new_z: f32 = unsafe { c_pow(polar.r, n) * c_cos(polar.theta * n) };

                    zeta.x = new_x + x;
                    zeta.y = new_y + y;
                    zeta.z = new_z + z;

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
                            unsafe {
                                MANDLEBULB.push(Vector { x, y, z });
                            }
                        }
                        break;
                    }
                }
            }
        }
        println!("Iteration: {} of {}", i, DIMENSIONS);
    }
}

fn to_point3(v: Vector) -> Point3<f32> {
    return Point3::new(v.x, v.y, v.z);
}

fn main() {
    create_bulb();
    unsafe {
        println!("Num of points in MandleBulb: {}", MANDLEBULB.len());
    }
    let window = Window::new("MandleBulb");
    let app = AppState {
        mandlebulb_renderer: MandleBulbRenderer::new(0.0),
    };

    window.render_loop(app)
}
