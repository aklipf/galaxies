use glam::{vec3, Vec3};
use miniquad::*;
use rand;
use rand_distr::{Distribution, Normal};

mod draw;
use draw::render;

const MAX_PARTICLES: usize = 256*1024;
const NUM_PARTICLES_EMITTED_PER_FRAME: usize = 100000;

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
        let rng = &mut rand::thread_rng();
        let normal = Normal::new(0.0f32, 1.0f32).unwrap();
        // emit new particles
        for _ in 0..NUM_PARTICLES_EMITTED_PER_FRAME {
            if self.pos.len() < MAX_PARTICLES {
                self.pos.push(2.0*vec3(
                    normal.sample(rng),
                    normal.sample(rng),
                    normal.sample(rng)));
                self.vel.push(0.1*vec3(
                    normal.sample(rng),
                    normal.sample(rng),
                    normal.sample(rng),
                ).normalize());
            } else {
                break;
            }
        }
        //let size = self.pos.len();
        //println!("particules: {size}");

        // update particle positions
        /*let mut avg=Vec3::ZERO;
        for i in 0..self.pos.len() {
            avg += self.pos[i]/(self.pos.len() as f32);
        }
        for i in 0..self.pos.len() {
            self.pos[i]-=avg;
        }*/

        for i in 0..self.pos.len() {
            let acc = -0.01*self.pos[i]/ (self.pos[i].length().powf(3.0)+1e-6);
            /*for j in 0..self.pos.len() {
                if i == j {
                    continue;
                }
                let dij = self.pos[i]-self.pos[j];
                acc += -0.001* dij/ (dij.length().powf(3.0)+1e-6);
            }*/
            self.vel[i] += 20.0*acc*frame_time;
            self.pos[i] += 20.0*self.vel[i] * frame_time;
            
        }
    }

    fn draw(&mut self) {
        self.draw.draw(&self.pos);
    }
}

fn main() {
    miniquad::start(conf::Conf::default(), move || Box::new(Stage::new()));
}
