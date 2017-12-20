extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate num; // for complex numbers

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use num::complex::Complex;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const W_SIZE_X: u32 = 2500;
const UPDATE_TIME: u32 = 100;

struct Triangle {
    p1: Complex<f64>,
    p2: Complex<f64>,
    p3: Complex<f64>,
    line: graphics::line::Line,
}
fn conv(value: f64, maxValue:f64, scale:f64) -> f64 {
        scale * value / maxValue
    }

impl Triangle {
    fn new(p1: Complex<f64>, p2: Complex<f64>, p3: Complex<f64>) -> Self {
        Triangle{ p1: p1, p2: p2, p3: p3, line: graphics::line::Line::new(WHITE, 0.5) }
    }
    fn draw(&self, gl: &mut GlGraphics, args: &RenderArgs, width: f64, height:f64, scale:f64, margin:f64) {
        gl.draw(args.viewport(), |c, gl| {
           self.line.draw([conv(self.p1.re, width, scale) + margin, conv(self.p1.im, height, scale * height / width) + margin, conv(self.p2.re, width, scale) + margin, conv(self.p2.im, height, scale * height / width) + margin], &c.draw_state, c.transform, gl);
           self.line.draw([conv(self.p3.re, width, scale) + margin, conv(self.p3.im, height, scale * height / width) + margin, conv(self.p2.re, width, scale) + margin, conv(self.p2.im, height, scale * height / width) + margin], &c.draw_state, c.transform, gl);
           self.line.draw([conv(self.p1.re, width, scale) + margin, conv(self.p1.im, height, scale * height / width) + margin, conv(self.p3.re, width, scale) + margin, conv(self.p3.im, height, scale * height / width) + margin], &c.draw_state, c.transform, gl);
        });
    }
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    triangles: std::collections::VecDeque<Triangle>,
    updateCnt: u32,
}

impl App {
    fn render(&mut self, args: &RenderArgs, width: f64, height: f64, scale: f64, margin: f64) {
        use graphics::*;

        self.gl.draw(args.viewport(), |_c, gl| {
            // Clear the screen.
            clear(BLACK, gl);
        });

        for t in &self.triangles {
            t.draw(&mut self.gl, args, width, height, scale, margin);
            //println!("draw p1:{},{} p2:{},{} p3:{},{}",t.p1.re, t.p1.im, t.p2.re, t.p2.im ,t.p3.re, t.p3.im);
            //println!("conj p1:{},{}", t.p1.conj().re, t.p1.conj().im);
        }
    }

    fn update(&mut self, _args: &UpdateArgs) {
        if self.updateCnt > UPDATE_TIME {
            let alpha: Complex<f64> = Complex::new(0.5, (3.0 as f64).sqrt() / 6.0);
            let tNum = self.triangles.len();
            for i in 0..tNum {
                let mut np1 = alpha.conj() * self.triangles[0].p1.conj() + alpha;
                let mut np2 = alpha.conj() * self.triangles[0].p2.conj() + alpha;
                let mut np3 = alpha.conj() * self.triangles[0].p3.conj() + alpha;
                self.triangles.push_back(Triangle::new(np1, np2, np3));
                np1 = alpha * self.triangles[0].p1.conj();
                np2 = alpha * self.triangles[0].p2.conj();
                np3 = alpha * self.triangles[0].p3.conj();
                self.triangles.push_back(Triangle::new(np1, np2, np3));
                self.triangles.pop_front();
                if ((np1 - np2).norm() < 0.001) {
                    self.triangles.clear();
                    assert!(self.triangles.is_empty());
                    let p1: Complex<f64> = Complex::new(0.0, 0.0);
                    let p2: Complex<f64> = Complex::new(0.5, (3.0 as f64).sqrt() / 6.0);
                    let p3: Complex<f64> = Complex::new(1.0, 0.0);
                    let t1 = Triangle::new(p1, p2, p3);
                    self.triangles.push_back(t1);
                    break;
                }
            }
            self.updateCnt = 0;
        }
        self.updateCnt += 1;
        //println!("cnt: {}", self.updateCnt);
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let mut margin: f64 = 10.0;
    let width: f64 = W_SIZE_X as f64 - 2.0*margin;
    let height: f64 = width * (3.0 as f64).sqrt() / 6.0;
    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "Koch Curve",
            [W_SIZE_X as u32, (height + 2.0*margin) as u32]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        triangles: std::collections::VecDeque::new(),
        updateCnt: 0,
    };

    let p1: Complex<f64> = Complex::new(0.0, 0.0);
    let p2: Complex<f64> = Complex::new(0.5, (3.0 as f64).sqrt() / 6.0);
    let p3: Complex<f64> = Complex::new(1.0, 0.0);
    let t1 = Triangle::new(p1, p2, p3);
    app.triangles.push_back(t1);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r, 1.0, (4.0 as f64).sqrt() / 6.0, width, margin);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}