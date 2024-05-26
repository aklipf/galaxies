use glam;
use miniquad::*;

const VERTEX: &str = r#"#version 100
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

const FRAGMENT: &str = r#"#version 100
precision mediump float;
varying lowp vec2 offset;

void main() {
    float dist=sqrt(offset.x*offset.x+offset.y*offset.y);
    if(dist<1.0)
        gl_FragColor = vec4(0.869,0.158,0.859,0.2*(1.0-dist));
    /*if(dist<0.9)
        gl_FragColor = vec4(0.869,0.158,0.859,1.0);//0.2*(1.0-dist)
    else if(dist<1.0)
        gl_FragColor = vec4(0.0,0.0,0.0,1.0);*/
    else
        discard;
}
"#;

pub fn source<'a>() -> ShaderSource<'a> {
    ShaderSource::Glsl {
        vertex: VERTEX,
        fragment: FRAGMENT,
    }
}

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
