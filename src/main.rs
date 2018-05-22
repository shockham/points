extern crate caper;

mod shaders;

use caper::types::{RenderItemBuilder, TransformBuilder, MaterialBuilder, DefaultTag};
use caper::game::*;
use caper::imgui::Ui;
use caper::input::Key;
use caper::mesh::gen_sphere;
use caper::posteffect::PostShaderOptionsBuilder;
use caper::utils::handle_fp_inputs;
use caper::utils::load_wavefront;


fn main() {
    let mut game = Game::<DefaultTag>::new();
    let mut debug_mode = false;

    // create a vector of render items
    game.add_render_item(
        RenderItemBuilder::default()
            .vertices(gen_sphere())
            .material(
                MaterialBuilder::default()
                    .shader_name("points".to_string())
                    .texture_name(Some("default".to_string()))
                    .build()
                    .unwrap(),
            )
            .instance_transforms(vec![
                TransformBuilder::default()
                    .pos((0f32, 0f32, 0f32))
                    .rot((0f32, 0f32, 0f32, 1f32))
                    .scale((1f32, 1f32, 1f32))
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap(),
    );
    game.add_render_item(
        RenderItemBuilder::default()
            .vertices(load_wavefront(include_bytes!("../assets/test.obj")))
            .material(
                MaterialBuilder::default()
                    .shader_name("points".to_string())
                    .texture_name(Some("default".to_string()))
                    .build()
                    .unwrap(),
            )
            .instance_transforms(vec![
                TransformBuilder::default()
                    .pos((10f32, 0f32, 0f32))
                    .rot((0f32, 0f32, 0f32, 1f32))
                    .scale((1f32, 1f32, 1f32))
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap(),
    );
    game.add_render_item(
        RenderItemBuilder::default()
            .vertices(gen_sphere())
            .material(
                MaterialBuilder::default()
                    .shader_name("texture".to_string())
                    .texture_name(Some("default".to_string()))
                    .build()
                    .unwrap(),
            )
            .instance_transforms(vec![
                TransformBuilder::default()
                    .pos((0f32, 3f32, 0f32))
                    .rot((0f32, 0f32, 0f32, 1f32))
                    .scale((1f32, 1f32, 1f32))
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap(),
    );

    // initial setup
    {
        shaders::add_custom_shaders(&mut game);

        game.renderer.post_effect.post_shader_options = PostShaderOptionsBuilder::default()
            .chrom_amt(1f32)
            .blur_amt(2f32)
            .blur_radius(2f32)
            .bokeh(true)
            .bokeh_focal_depth(0.45f32)
            .bokeh_focal_width(0.4f32)
            .color_offset((1f32, 1f32, 1f32, 1f32))
            .build()
            .unwrap();
    }

    loop {
        // run the engine update
        let status = game.update(
            |_: &Ui| {},
            |game: &mut Game<DefaultTag>| -> UpdateStatus {

                // update the first person inputs
                if game.input.hide_mouse {
                    handle_fp_inputs(&mut game.input, &mut game.cams[0]);
                }

                // screenshot
                if game.input.keys_pressed.contains(&Key::P) {
                    game.renderer.save_screenshot();
                }

                // editor shortcuts
                if game.input.keys_down.contains(&Key::LShift) {
                    if game.input.keys_down.contains(&Key::L) {
                        debug_mode = true;
                    }
                    if game.input.keys_down.contains(&Key::K) {
                        debug_mode = false;
                    }
                    game.input.hide_mouse = !game.input.keys_down.contains(&Key::M);
                }
                game.renderer.show_editor = debug_mode;

                // quit
                if game.input.keys_down.contains(&Key::Escape) {
                    return UpdateStatus::Finish;
                }

                UpdateStatus::Continue
            },
        );

        if let UpdateStatus::Finish = status {
            break;
        }
    }
}
