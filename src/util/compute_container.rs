use glium::program::ComputeShader;
use glium::Display;

/// Unused as of now

pub const BASE_CSHADER: &str = r#"
    #version 430 core
    
    struct Boid {
        vec2 pos;
        vec2 vel;
        vec2 acc;
    }
    layout (local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
    
"#;

pub fn base_cshader(display: &Display) -> ComputeShader {
    ComputeShader::from_source(display, &BASE_CSHADER).expect("Couldn't compile code!")
}
