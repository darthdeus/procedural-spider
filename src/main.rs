use crate::prelude::*;
pub const RESIZE_RATIO: f32 = 1.0;

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

const EAT_DISTANCE: f32 = 40.0;

#[macroquad::main(window_conf)]
async fn main() {
    let mut music_playing: bool = false;

    let screen_center = Vec2::new(
        screen_width() / 2.0 / RESIZE_RATIO,
        screen_height() / 2.0 / RESIZE_RATIO,
    );

    let mut player_spider = Spider::new(
        1.0,
        screen_center + Vec2::new(-300.0, 0.0),
        SpiderType::Player,
    );

    let mut spiders = vec![
        Spider::new(0.5, screen_center + Vec2::new(200.0, 0.0), SpiderType::Left),
        Spider::new(0.5, screen_center + Vec2::new(0.0, 100.0), SpiderType::Left),
        Spider::new(
            0.5,
            screen_center + Vec2::new(0.0, -200.0),
            SpiderType::Right,
        ),
        Spider::new(
            0.5,
            screen_center + Vec2::new(0.0, 600.0),
            SpiderType::Right,
        ),
        Spider::new(
            0.5,
            screen_center + Vec2::new(0.0, -400.0),
            SpiderType::Left,
        ),
    ];

    let soundtrack = include_bytes!("soundtrack.wav");
    let nom_wav = include_bytes!("nom-sfx.wav");

    let sound = macroquad::audio::load_sound_from_bytes(soundtrack).await;
    let nom_sfx = macroquad::audio::load_sound_from_bytes(nom_wav).await;

    let crt_material =
        load_material(shaders::VERTEX, shaders::FRAGMENT, Default::default()).unwrap();

    let move_min = screen_center - Vec2::new(20.0, 20.0);
    let move_max = screen_center + Vec2::new(20.0, 20.0);

    let mut i = 0.0;

    let mut debug_keyboard_override = false;
    let mut use_shader = true;
    let mut auto_move_spider = true;
    let mut debug_ui = false;

    let main_render_target = render_target(screen_width() as u32, screen_height() as u32);
    main_render_target.texture.set_filter(FilterMode::Nearest);

    let spider_render_target = render_target(screen_width() as u32, screen_height() as u32);
    spider_render_target.texture.set_filter(FilterMode::Nearest);

    const NICE_PINK: Color = Color::new(1.0, 0.6245, 0.7, 1.0);

    loop {
        if is_key_down(KeyCode::Escape) {
            break;
        }

        i += 1.0 / 60.0;

        let new_pos = if auto_move_spider {
            Vec2::new(4.0 * f32::sin(i), 2.0 * f32::cos(i)) * 20.0
        } else {
            let mouse = mouse_position();
            Vec2::new(mouse.0, mouse.1) - screen_center
        };

        let mut move_dir = Vec2::new(0.0, 0.0);

        if is_key_pressed(KeyCode::Tab) {
            debug_ui = !debug_ui;
        }

        if is_key_down(KeyCode::W) {
            move_dir.y -= 1.0;
        }
        if is_key_down(KeyCode::S) {
            move_dir.y += 1.0;
        }

        if is_key_down(KeyCode::A) {
            move_dir.x -= 1.0;
        }
        if is_key_down(KeyCode::D) {
            move_dir.x += 1.0;
        }

        if move_dir.length() > 0.0 {
            if !music_playing {
                music_playing = true;

                match sound {
                    Ok(sound) => {
                        println!("playing music");

                        macroquad::audio::play_sound(
                            sound,
                            macroquad::audio::PlaySoundParams {
                                looped: true,
                                volume: 1.0,
                            },
                        );
                    }

                    Err(ref err) => {
                        println!("Failed to load sound {}", err);
                    }
                }
            }
        }

        if debug_keyboard_override {
            player_spider.move_towards(new_pos);
        } else {
            player_spider.move_towards(move_dir);
        }

        let player_pos = player_spider.pos;

        for spider in spiders.iter_mut() {
            spider.run_away_from(player_pos);
        }

        if debug_ui {
            egui_macroquad::ui(|ctx| {
                egui::Window::new("Debug UI").show(ctx, |ui| {
                    ui.add(
                        egui::Slider::new(&mut player_spider.pos.x, move_min.x..=move_max.x)
                            .text("x"),
                    );
                    ui.add(
                        egui::Slider::new(&mut player_spider.pos.y, move_min.y..=move_max.y)
                            .text("y"),
                    );
                    ui.add(
                        egui::Slider::new(&mut player_spider.max_leg_angle, 0.05..=3.0)
                            .text("Max leg angle:"),
                    );

                    ui.checkbox(&mut use_shader, "Use shader");
                    ui.checkbox(&mut auto_move_spider, "Auto move spider");
                    ui.checkbox(&mut player_spider.debug_leg_angles, "Debug leg angles");
                    ui.checkbox(&mut player_spider.debug_color_legs, "Debug color legs");
                    ui.checkbox(&mut player_spider.debug_draw_joints, "Debug draw joints");
                    ui.checkbox(&mut debug_keyboard_override, "Debug keyboard override");
                    unsafe {
                        ui.checkbox(&mut DEBUG_AI_LABELS, "Debug AI labels");
                    }

                    ui.label(format!("{:#.2?}", player_spider));
                });

                for (i, spider) in spiders.iter().enumerate() {
                    egui::Window::new(format!("spider-{}", i))
                        .current_pos(egui::pos2(spider.pos.x, spider.pos.y))
                        .show(ctx, |ui| {
                            ui.label(format!("{:#.2?}", spider));
                        });
                }
            });
        }

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

        clear_background(NICE_PINK);
        for spider in spiders.iter_mut() {
            spider.draw();
        }

        player_spider.draw();

        set_default_camera();
        clear_background(BLACK);

        if use_shader {
            gl_use_material(crt_material);
        }

        draw_texture_ex(
            main_render_target.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                flip_y: true,
                ..Default::default()
            },
        );

        gl_use_default_material();

        if debug_ui {
            egui_macroquad::draw();
        }

        let mut to_spawn = vec![];

        spiders.retain(|spider| {
            let too_close = (spider.pos - player_spider.pos).length() < EAT_DISTANCE;

            if too_close {
                player_spider.scale += 0.15;
                to_spawn.push(Spider::new(
                    0.5,
                    screen_center + Vec2::new(rand::gen_range::<f32>(-200.0, 200.0), 0.0),
                    SpiderType::Left,
                ));

                match &nom_sfx {
                    Ok(ref nom_sfx) => {
                        macroquad::audio::play_sound(
                            nom_sfx.clone(),
                            macroquad::audio::PlaySoundParams {
                                looped: false,
                                volume: 1.0,
                            },
                        );
                    }

                    Err(err) => {
                        println!("Failed to load sound {}", err);
                    }
                }
            }

            !too_close
        });

        for spider in to_spawn.into_iter() {
            spiders.push(spider);
        }

        // draw_texture(main_render_target.texture, 0.0, 0.0, NICE_PINK);
        // clear_background(NICE_PINK);
        // draw_texture_ex()
        // let screen_data: Image = get_screen_data();

        next_frame().await;
    }
}
