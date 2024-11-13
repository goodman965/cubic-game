#![feature(const_fn_floating_point_arithmetic)]

use macroquad::prelude::*;
use macroquad::telemetry::Frame;
use std::time::Instant;

mod grab;
use grab::*;

mod player;
use player::*;

mod world;
use crate::world::render::mesh::build_model_meshes;
use crate::world::render::model::build_chunk_model;
use world::*;

#[rustfmt::skip]
const SKY_COLOR: Color = Color { r: 0.3, g: 0.3, b: 0.5, a: 1.0 };

#[rustfmt::skip]
pub async fn run_client() {
    let atlas_data = include_bytes!("../assets/atlas.png");
    let atlas: Texture2D = Texture2D::from_file_with_format(atlas_data, Some(ImageFormat::Png));
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

    let mut chunks = vec![];
    let material = vec![BlockState::GRASS, BlockState::STONE, BlockState::SAND, BlockState::DIRT];

    for i in 0..24 {
        let mut chunk = Chunk::EMPTY;
        let mut dec = 0;
        for y in 0..CHUNK_SIZE_16 {
            for x in 0..CHUNK_SIZE_16 {
                for z in 0..CHUNK_SIZE_16 {
                    let r = dec..CHUNK_SIZE_16-dec;
                    *chunk.get_mut(x, y, z) = if !r.contains(&x) || !r.contains(&z) {
                        BlockState::EMPTY
                    } else {
                        //BlockState::GRASS
                        material[i % 4].clone()
                    };
                }
            }
            if y % 2 != 0 { dec += 1 };
        }
        chunks.push(chunk);
    }

    let size = CHUNK_SIZE_16 as f32;
    let mut y = 0.0;
    chunks[0].set_pos(0.0, y, 0.0);
    chunks[1].set_pos( 0.0 - size, y, 0.0);
    chunks[2].set_pos(0.0, y, 0.0 - size);
    chunks[3].set_pos(0.0 - size, y, 0.0 - size);

    y = size;
    chunks[4].set_pos(0.0, y, 0.0);
    chunks[5].set_pos( 0.0 - size, y, 0.0);
    chunks[6].set_pos(0.0, y, 0.0 - size);
    chunks[7].set_pos(0.0 - size, y, 0.0 - size);

    y = 0.0 - size;
    chunks[8].set_pos(0.0, y, 0.0);
    chunks[9].set_pos(0.0 - size, y, 0.0);
    chunks[10].set_pos(0.0, y, 0.0 - size);
    chunks[11].set_pos(0.0 - size, y, 0.0 - size);

    y = 0.0 - 2.0 * size;
    chunks[12].set_pos(0.0, y, 0.0);
    chunks[13].set_pos( 0.0 - size, y, 0.0);
    chunks[14].set_pos(0.0, y, 0.0 - size);
    chunks[15].set_pos(0.0 - size, y, 0.0 - size);

    y = 0.0 - 3.0 * size;
    chunks[16].set_pos(0.0, y, 0.0);
    chunks[17].set_pos( 0.0 - size, y, 0.0);
    chunks[18].set_pos(0.0, y, 0.0 - size);
    chunks[19].set_pos(0.0 - size, y, 0.0 - size);

    y = 2.0 * size;
    chunks[20].set_pos(0.0, y, 0.0);
    chunks[21].set_pos( 0.0 - size, y, 0.0);
    chunks[22].set_pos(0.0, y, 0.0 - size);
    chunks[23].set_pos(0.0 - size, y, 0.0 - size);


    setup_mouse_cursor();
    let mut fps_mean = vec![];
    // let mut frame_mean = vec![];
    let mut math_mean = vec![];
    let push_to_mean = |arr: &mut Vec<usize>, val: usize| -> usize {
        arr.insert(0, val);
        arr.truncate(100);
        arr.iter().sum::<usize>() /100_usize
    };

    loop {
        let now = Instant::now();
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

        for chunk in &chunks {
            let chunk_model = build_chunk_model( player_pos.0, front.0, chunk);
            let chunk_meshes = build_model_meshes(chunk_model, Some(atlas.clone()), chunk.get_pos());
            for chunk_mesh in &chunk_meshes {
                draw_mesh(&chunk_mesh);
            }
        }

        /* Back to screen space */ set_default_camera();
        let fps = get_fps()  as usize;
        let mean_fps = push_to_mean(&mut fps_mean, fps);
        
        let math = now.elapsed().as_micros() as usize;
        let mean_math = push_to_mean(&mut math_mean, math);
        let fps_frame_str = format!("FPS: {} Math: {} mcs", mean_fps, mean_math);
        render_text_overlay(player_pos, fps_frame_str.as_str());
        last_mouse_pos.0 = mouse_position().into();

        next_frame().await
    }
}

#[allow(dead_code)]
fn print_n_meshes(chunk_meshes: &Vec<Mesh>) {
    let y = 40.0 + 40.0 * 2.0;
    for (n, mesh) in chunk_meshes.into_iter().enumerate() {
        draw_text(
            format!("Mesh#{}, IND ({})", n, mesh.indices.len()).as_str(),
            10.0,
            y + 40.0 * n as f32,
            20.0,
            BLACK,
        );
        draw_text(
            format!("VERT ({})", mesh.vertices.len()).as_str(),
            10.0,
            20.0 + y + 40.0 * n as f32,
            20.0,
            BLACK,
        );
    }
}

fn render_text_overlay(player_pos: PlayerPos, fps: &str) {
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
        // format!("FPS: {}", fps).as_str(),
        fps,
        10.0,
        40.0 + 40.0,
        60.0,
        BLACK,
    );
}
