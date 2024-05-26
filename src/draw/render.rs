use miniquad::*;

use glam::{vec3, Mat4, Vec3};

use super::shader;

pub struct Draw {
    ctx: Box<dyn RenderingBackend>,

    pipeline: Pipeline,
    bindings: Bindings,

    ry: f32,
}

impl Draw {
    pub fn new(max_particles: usize) -> Draw {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        #[rustfmt::skip]
        let vertices: &[f32] = &[
             -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, -1.0, -1.0
        ];
        // vertex buffer for static geometry
        let geometry_vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vertices),
        );

        #[rustfmt::skip]
        let indices: &[u16] = &[
            0, 1, 2, 0, 2, 3
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
            BufferSource::empty::<Vec3>(max_particles),
        );

        let bindings = Bindings {
            vertex_buffers: vec![geometry_vertex_buffer, positions_vertex_buffer],
            index_buffer: index_buffer,
            images: vec![],
        };

        let shader = ctx
            .new_shader(
                shader::source(),
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

        Draw {
            ctx,
            pipeline,
            bindings,
            ry:0.0,
        }
    }

    pub fn draw(&mut self, pos: &Vec<Vec3>){
        assert_eq!(std::mem::size_of::<Vec3>(), 12);

        self.ctx.buffer_update(
            self.bindings.vertex_buffers[1],
            BufferSource::slice(&pos[..]),
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
        self.ctx.draw(0, 6, pos.len() as i32);
        self.ctx.end_render_pass();

        self.ctx.commit_frame();
    }
}
