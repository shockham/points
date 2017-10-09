use caper::shader::default;
use caper::game::Game;

pub fn add_custom_shaders(game: &mut Game) {
    let shaders = &mut game.renderer.shaders;
    let display = &game.renderer.display;
    let _ = shaders.add_shader(
        display,
        "points",
        default::gl330::VERT,
        points::FRAG,
        points::GEOM,
        default::gl330::TESS_CONTROL,
        default::gl330::TESS_EVAL,
    );
}


mod points {
    // fragment shader
    pub const FRAG: &'static str = "
        #version 330

        uniform vec3 cam_pos;

        const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);

        in vec3 g_normal;
        in vec3 g_pos;

        out vec4 frag_output;

        void main() {
            float lum = max(dot(normalize(g_normal), normalize(LIGHT)), 0.0);
            float dist = abs(distance(cam_pos, g_pos)) / 400.0;
            float norm_height = normalize(g_pos).y;
            float height = g_pos.y / 100.0;
            float col_val = clamp(norm_height, 0.1, 1.0);
            vec3 base_color = vec3(col_val);

            base_color = mix(base_color, vec3(1.0), dist);

            vec3 color = base_color * ((0.05 * lum) + (0.95 * dist));
            frag_output = vec4(color, 1.0);
        }
    ";

    // geometry shader
    pub const GEOM: &'static str = "
        #version 330

        const float SIZE = 0.2;

        layout(triangles) in;
        layout(triangle_strip, max_vertices=6) out;

        in vec3 te_normal[];
        in vec3 te_pos[];
        in vec2 te_texture[];

        out vec3 g_normal;
        out vec3 g_pos;
        out vec2 g_texture;

        void main(void) {
            for(int i = 0; i < gl_in.length(); i++){
                g_normal = te_normal[i];
                g_pos = te_pos[i];
                g_texture = te_texture[i];
                gl_Position = gl_in[i].gl_Position;
                EmitVertex();

                g_normal = te_normal[i];
                g_pos = te_pos[i];
                g_pos.x += SIZE;
                g_texture = te_texture[i];
                gl_Position = gl_in[i].gl_Position;
                gl_Position.x += SIZE;
                EmitVertex();

                g_normal = te_normal[i];
                g_pos = te_pos[i];
                g_pos.y += SIZE;
                g_texture = te_texture[i];
                gl_Position = gl_in[i].gl_Position;
                gl_Position.y += SIZE;
                EmitVertex();

                //EndPrimitive();

                g_normal = te_normal[i];
                g_pos = te_pos[i];
                g_pos.x += SIZE;
                g_texture = te_texture[i];
                gl_Position = gl_in[i].gl_Position;
                gl_Position.x += SIZE;
                EmitVertex();

                g_normal = te_normal[i];
                g_pos = te_pos[i];
                g_pos.y += SIZE;
                g_texture = te_texture[i];
                gl_Position = gl_in[i].gl_Position;
                gl_Position.y += SIZE;
                EmitVertex();

                g_normal = te_normal[i];
                g_pos = te_pos[i];
                g_pos.y += SIZE;
                g_pos.x += SIZE;
                g_texture = te_texture[i];
                gl_Position = gl_in[i].gl_Position;
                gl_Position.y += SIZE;
                gl_Position.x += SIZE;
                EmitVertex();

                EndPrimitive();
            }
        }
    ";
}
