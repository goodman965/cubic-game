#![feature(const_fn_floating_point_arithmetic)]

use macroquad::prelude::*;

mod grab;
use grab::*;

mod player;
use player::*;

mod world;
use world::render::*;
use world::*;

#[rustfmt::skip]
const SKY_COLOR: Color = Color { r: 0.3, g: 0.3, b: 0.5, a: 1.0 };

#[rustfmt::skip]
pub async fn run_client() {

    let atlas: Texture2D = load_texture("assets/atlas.png").await.unwrap();
    atlas.set_filter(FilterMode::Nearest);

    let mut yaw = Yaw::default();
    let mut pitch = Pitch::default();

    let mut front = Front::new(yaw, pitch);
    let mut right = Right::new(front);
    let mut up = Up::new(right, front);

    let mut player_pos = PlayerPos::default();

    let mut last_mouse_pos: LastMousePos = mouse_position().into();
    let mut current_mouse_pos: CurrentMousePos = mouse_position().into();

    let mut grabbed = Grabbed::default();

    let mut chunk = Chunk::EMPTY;

    for x in 0..16 {
        for z in 0..16 {
            for y in 0..=3 {
                let rand = rand::gen_range(0, 5);
                match rand {
                    0 | 1 => *chunk.get_mut(x, y, z) = BlockState::DIRT,
                    2 | 3 => *chunk.get_mut(x, y, z) = BlockState::STONE,
                    _ => *chunk.get_mut(x, y, z) = BlockState::SAND,
                }
            }
        }
    }

    setup_mouse_cursor();
    
    loop {
        if is_key_pressed(KeyCode::Escape) { break; }

        update_grabbed_state_and_cursor_on_tab_press(&mut grabbed);

        *current_mouse_pos = mouse_position().into();

        if *grabbed {
            update_yaw_pitch_after_mouse_pos_changed(&current_mouse_pos, &last_mouse_pos, &mut yaw, &mut pitch);
            update_player_pos_after_front_right_up_changed(&mut player_pos, front, right);
            update_front_right_up_vecs_after_yaw_pitch_changed(&yaw, &pitch, &mut front, &mut right, &mut up);
        }

        clear_background(SKY_COLOR);

        set_camera(&Camera3D {
            position: *player_pos,
            up: *up,
            target: *player_pos + *front,
            ..Default::default()
        });

        draw_grid(20, 1., BLACK, GRAY);

        let mut chunk_models: Vec<(ChunkPos, ChunkModel)> = Vec::with_capacity(100);

        for ch_x in -2..2 {
            let chunk_pos = ChunkPos { x: ch_x, y: 0, z: 0 };

            let chunk_model = build_chunk_model(
                player_pos.0, *front, chunk_pos, &chunk, &ConnectedChunks::EMPTY
            );

            chunk_models.push((chunk_pos, chunk_model));
        }

        let chunk_meshes: Vec<_> = build_chunk_meshes(chunk_models, Some(atlas.clone())).collect();

        for chunk_mesh in &chunk_meshes {
            draw_mesh(&chunk_mesh);
        }

        /* Back to screen space */ set_default_camera();

        print_n_meshes(&chunk_meshes);
        render_text_overlay(player_pos, get_fps());

        last_mouse_pos.0 = mouse_position().into();

        next_frame().await
    }
}

fn print_n_meshes(chunk_meshes: &Vec<Mesh>) {
    let len = chunk_meshes.len();
    let y = 40.0 + 40.0 * 2.0;
    for (n, mesh) in chunk_meshes.into_iter().enumerate() {
        draw_text(
            format!("N Meshes: {}, IND: {:?}", len, mesh.indices).as_str(),
            10.0,
            y + 40.0 * n as f32,
            60.0,
            BLACK,
        );
        draw_text(
            format!("VERT: {:?}", mesh.vertices.iter().map(|v| v.position).collect::<Vec<_>>()).as_str(),
            10.0,
            40.0 + y + 40.0 * n as f32,
            60.0,
            BLACK,
        );
    }
}

fn render_text_overlay(player_pos: PlayerPos, fps: i32) {
    draw_text(
        format!(
            "X: {:.2} Y: {:.2} Z: {:.2}",
            player_pos.x, player_pos.y, player_pos.z
        )
        .as_str(),
        10.0,
        40.0,
        60.0,
        BLACK,
    );
    draw_text(
        format!("FPS: {}", fps).as_str(),
        10.0,
        40.0 + 40.0,
        60.0,
        BLACK,
    );
}
