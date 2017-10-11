extern crate caper;

mod shaders;

use caper::types::{RenderItemBuilder, TransformBuilder, MaterialBuilder};
use caper::game::Game;
use caper::imgui::Ui;
use caper::input::Key;
use caper::mesh::gen_sphere;
use caper::posteffect::PostShaderOptionsBuilder;
use caper::utils::handle_fp_inputs;


fn main() {
    let mut game = Game::new();

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
        // screenshot
        if game.input.keys_pressed.contains(&Key::P) {
            game.renderer.save_screenshot();
        }
        // run the engine update
        game.update(|_: &Ui| {});

        // update the first person inputs
        handle_fp_inputs(&mut game.input, &mut game.cam);

        // quit
        if game.input.keys_down.contains(&Key::Escape) {
            break;
        }
    }
}