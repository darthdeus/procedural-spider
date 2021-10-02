use crate::prelude::*;

mod prelude;
mod shaders;
mod spider;

fn window_conf() -> Conf {
    Conf {
        window_title: "Macroquad - Procedural Spider".to_owned(),
        window_width: 1400,
        window_height: 1000,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut spider = Spider::new();

    let material = load_material(shaders::VERTEX, shaders::FRAGMENT, Default::default()).unwrap();

    let orig_pos = spider.pos;

    let move_min = spider.pos - Vec2::new(20.0, 20.0);
    let move_max = spider.pos + Vec2::new(20.0, 20.0);

    let mut i = 0.0;

    let mut use_shader = true;
    let mut auto_move_spider = true;

    let main_render_target = render_target(screen_width() as u32, screen_height() as u32);
    main_render_target.texture.set_filter(FilterMode::Nearest);

    const NICE_PINK: Color = Color::new(1.0, 0.6245, 0.7, 1.0);

    loop {
        if is_key_down(KeyCode::Escape) {
            break;
        }

        i += 1.0 / 60.0;

        let new_pos = if auto_move_spider {
            orig_pos + Vec2::new(4.0 * f32::sin(i), 2.0 * f32::cos(i)) * 20.0
        } else {
            let mouse = mouse_position();
            Vec2::new(mouse.0, mouse.1)
        };

        spider.move_to(new_pos);

        egui_macroquad::ui(|ctx| {
            egui::Window::new("Window")
                .show(ctx, |ui| {
                    ui.label("macroquaaad");
                    ui.add(egui::Slider::new(&mut spider.pos.x, move_min.x..=move_max.x).text("x"));
                    ui.add(egui::Slider::new(&mut spider.pos.y, move_min.y..=move_max.y).text("y"));

                    ui.checkbox(&mut use_shader, "Use shader:");
                    ui.checkbox(&mut auto_move_spider, "Auto move spider:");
                    ui.checkbox(&mut spider.debug_leg_angles, "Debug leg angles:");
                    ui.checkbox(&mut spider.debug_color_legs, "Debug color legs:");
                    // unsafe {
                    //     ui.checkbox(&mut USE_QUAT, "Use quat");
                    // }
                });
        });

        // const SCR_W: f32 = 100.0;
        // const SCR_H: f32 = 60.0;

        let render_screen_w = screen_width() / RESIZE_RATIO;
        let render_screen_h = screen_height() / RESIZE_RATIO;

        set_camera(&Camera2D {
            zoom: vec2(1.0 / render_screen_w * 2.0, -1.0 / render_screen_h * 2.0),
            target: vec2(render_screen_w / 2.0, render_screen_h / 2.0),
            render_target: Some(main_render_target),
            ..Default::default()
        });

        // TODO: drawing with NICE_PINK changes the color for some reason?
        clear_background(Color::new(0.0, 0.0, 0.0, 0.0));
        spider.draw();

        set_default_camera();
        clear_background(NICE_PINK);

        if use_shader {
            gl_use_material(material);
        }

        draw_texture_ex(
            main_render_target.texture,
            0.0,
            0.0,
            NICE_PINK,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                flip_y: true,
                ..Default::default()
            },
        );

        gl_use_default_material();
        egui_macroquad::draw();

        // clear_background(Color::new(1.0, 0.6245, 0.7, 1.0));

        // draw_texture(main_render_target.texture, 0.0, 0.0, NICE_PINK);
        // clear_background(NICE_PINK);
        // draw_texture_ex()
        // let screen_data: Image = get_screen_data();

        // println!("f = {}", f);

        next_frame().await;
    }
}
