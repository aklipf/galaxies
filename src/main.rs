use miniquad::*;

use glam::{vec3, Mat4, Vec3};

const MAX_PARTICLES: usize = 32 * 1024;
const NUM_PARTICLES_EMITTED_PER_FRAME: usize = 128;

struct Stage {
    ctx: Box<dyn RenderingBackend>,

    pipeline: Pipeline,
    bindings: Bindings,

    pos: Vec<Vec3>,
    vel: Vec<Vec3>,
    ry: f32,
}

impl Stage {
    pub fn new() -> Stage {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        #[rustfmt::skip]
        let vertices: &[f32] = &[
             -1.0, 1.0,1.0, 1.0, 1.0, -1.0,-1.0, -1.0
        ];
        // vertex buffer for static geometry
        let geometry_vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vertices),
        );

        #[rustfmt::skip]
        let indices: &[u16] = &[
            0, 1, 2,    0, 2, 3
        ];
        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices),
        );

        // empty, dynamic instance data vertex buffer
        let positions_vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Stream,
            BufferSource::empty::<Vec3>(MAX_PARTICLES),
        );

        let bindings = Bindings {
            vertex_buffers: vec![geometry_vertex_buffer, positions_vertex_buffer],
            index_buffer: index_buffer,
            images: vec![],
        };

        let shader = ctx
            .new_shader(
                ShaderSource::Glsl {
                    vertex: shader::VERTEX,
                    fragment: shader::FRAGMENT,
                },
                shader::meta(),
            )
            .unwrap();

        let pipeline = ctx.new_pipeline(
            &[
                BufferLayout::default(),
                BufferLayout {
                    step_func: VertexStep::PerInstance,
                    ..Default::default()
                },
            ],
            &[
                VertexAttribute::with_buffer("vertices", VertexFormat::Float2, 0),
                VertexAttribute::with_buffer("pos_particle", VertexFormat::Float3, 1),
            ],
            shader,
            PipelineParams {
                color_blend: Some(BlendState::new(
                    Equation::Add,
                    BlendFactor::Value(BlendValue::SourceAlpha),
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                )),
                alpha_blend: Some(BlendState::new(
                    Equation::Add,
                    BlendFactor::Zero,
                    BlendFactor::One,
                )),
                ..Default::default()
            },
        );

        Stage {
            ctx,
            pipeline,
            bindings,
            pos: Vec::with_capacity(MAX_PARTICLES),
            vel: Vec::with_capacity(MAX_PARTICLES),
            ry: 0.,
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
        // by default glam-rs can vec3 as u128 or #[reprc(C)](f32, f32, f32). need to ensure that the second option was used
        assert_eq!(std::mem::size_of::<Vec3>(), 12);

        self.ctx.buffer_update(
            self.bindings.vertex_buffers[1],
            BufferSource::slice(&self.pos[..]),
        );

        // model-view-projection matrix
        let (width, height) = window::screen_size();

        let proj = Mat4::perspective_rh_gl(60.0f32.to_radians(), width / height, 0.01, 50.0);
        let view = Mat4::look_at_rh(
            vec3(0.0, 1.5, 12.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
        );
        let view_proj = proj * view;

        self.ry += 0.01;
        let mvp = view_proj * Mat4::from_rotation_y(self.ry);

        self.ctx.begin_default_pass(Default::default());

        self.ctx.apply_pipeline(&self.pipeline);
        self.ctx.apply_bindings(&self.bindings);
        self.ctx
            .apply_uniforms(UniformsSource::table(&shader::Uniforms {
                mvp: mvp,
                aspect_ratio: width / height,
                size: 0.2,
            }));
        self.ctx.draw(0, 6, self.pos.len() as i32);
        self.ctx.end_render_pass();

        self.ctx.commit_frame();
    }
}

fn main() {
    miniquad::start(conf::Conf::default(), move || Box::new(Stage::new()));
}

mod shader {
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec2 vertices;
    attribute vec3 pos_particle;

    varying lowp vec2 offset;

    uniform mat4 mvp;
    uniform float aspect_ratio;
    uniform float size;

    void main() {
        vec4 pos = vec4(pos_particle, 1.0);
        gl_Position = mvp * pos + vec4(size*vertices*vec2(1.0,aspect_ratio)/2.0,1.0, 1.0);
        offset = vertices;
    }
    "#;

    pub const FRAGMENT: &str = r#"#version 100
    precision mediump float;
    varying lowp vec2 offset;

    void main() {
        float dist=sqrt(offset.x*offset.x+offset.y*offset.y);
        if(dist<1.0)
            gl_FragColor = vec4(1.0,1.0,1.0,0.2*(1.0-dist));
        else
            discard;
    }
    "#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout {
                uniforms: vec![
                    UniformDesc::new("mvp", UniformType::Mat4),
                    UniformDesc::new("aspect_ratio", UniformType::Float1),
                    UniformDesc::new("size", UniformType::Float1),
                ],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub mvp: glam::Mat4,
        pub aspect_ratio: f32,
        pub size: f32,
    }
}
