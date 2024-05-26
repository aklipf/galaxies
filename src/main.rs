use miniquad::*;

use glam::{vec3, Vec3};

mod draw;
use draw::render;

const MAX_PARTICLES: usize = 32 * 1024;
const NUM_PARTICLES_EMITTED_PER_FRAME: usize = 1024;


struct Stage {
    draw: render::Draw,
    pos: Vec<Vec3>,
    vel: Vec<Vec3>,
}

impl Stage {
    pub fn new() -> Stage {
        Stage {
            draw: render::Draw::new(MAX_PARTICLES),
            pos: Vec::with_capacity(MAX_PARTICLES),
            vel: Vec::with_capacity(MAX_PARTICLES),
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {
        let frame_time = 1. / 60.;
        // emit new particles
        for _ in 0..NUM_PARTICLES_EMITTED_PER_FRAME {
            if self.pos.len() < MAX_PARTICLES {
                self.pos.push(vec3(0., 0., 0.));
                self.vel.push(vec3(
                    quad_rand::gen_range(-1., 1.),
                    quad_rand::gen_range(0., 2.),
                    quad_rand::gen_range(-1., 1.),
                ));
            } else {
                break;
            }
        }
        //let size = self.pos.len();
        //println!("particules: {size}");

        // update particle positions
        for i in 0..self.pos.len() {
            self.vel[i] -= vec3(0., frame_time, 0.);
            self.pos[i] += self.vel[i] * frame_time;
            /* bounce back from 'ground' */
            if self.pos[i].y < -2.0 {
                self.pos[i].y = -1.8;
                self.vel[i] *= vec3(0.8, -0.8, 0.8);
            }
        }
    }

    fn draw(&mut self) {
        self.draw.draw(&self.pos);
    }
}

fn main() {
    miniquad::start(conf::Conf::default(), move || Box::new(Stage::new()));
}
