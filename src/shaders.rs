use caper::shader::{ default, texture };
use caper::game::Game;

pub fn add_custom_shaders(game: &mut Game) {
    let shaders = &mut game.renderer.shaders;
    let display = &game.renderer.display;
    let _ = shaders.add_shader(
        display,
        "points",
        default::gl330::VERT,
        texture::gl330::FRAG,
        points::GEOM,
        default::gl330::TESS_CONTROL,
        default::gl330::TESS_EVAL,
    );
}


mod points {
    // geometry shader
    pub const GEOM: &'static str = "
        #version 330

        uniform vec2 viewport;
        const float SIZE = 0.3;

        layout(triangles) in;
        layout(triangle_strip, max_vertices=24) out;

        in vec3 te_normal[];
        in vec3 te_pos[];
        in vec2 te_texture[];

        out vec3 g_normal;
        out vec3 g_pos;
        out vec2 g_texture;

        void emit (int i, vec4 diff) {
            g_normal = te_normal[i];
            g_pos = te_pos[i] + diff.xyz;
            g_texture = te_texture[i];
            gl_Position = gl_in[i].gl_Position + diff;
            EmitVertex();
        }

        void prim (int i, float x, float y) {
            float s_x = x * 0.7;
            float s_y = y * 0.7;

            emit(i, vec4(0));
            emit(i, vec4(x, 0, 0, 0));
            emit(i, vec4(s_x, s_y, 0, 0));
            EndPrimitive();

            emit(i, vec4(0));
            emit(i, vec4(s_x, s_y, 0, 0));
            emit(i, vec4(0, y, 0, 0));
            EndPrimitive();
        }

        void i_prim (int i, float x, float y) {
            float s_x = x * 0.8;
            float s_y = y * 0.8;

            emit(i, vec4(s_x, s_y, 0, 0));
            emit(i, vec4(x, 0, 0, 0));
            emit(i, vec4(0));
            EndPrimitive();

            emit(i, vec4(0, y, 0, 0));
            emit(i, vec4(s_x, s_y, 0, 0));
            emit(i, vec4(0));
            EndPrimitive();
        }

        void main(void) {
            float vy_size = SIZE * (viewport.x / viewport.y);

            for(int i = 0; i < gl_in.length(); i++){
                prim(i, SIZE, vy_size);
                i_prim(i, -SIZE, vy_size);
                i_prim(i, SIZE, -vy_size);
                prim(i, -SIZE, -vy_size);
            }
        }
    ";
}
