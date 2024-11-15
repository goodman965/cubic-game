#![feature(const_fn_floating_point_arithmetic)]

use macroquad::prelude::*;
use std::time::Instant;

mod grab;
use grab::*;

mod player;
use player::*;

mod world;
use crate::world::render::mesh::build_model_meshes;
use crate::world::render::model::{build_chunk_model, update_chunk_model};
use world::*;
use crate::world::render::WorldPos;

const SKY_COLOR: Color = Color { r: 0.3, g: 0.3, b: 0.5, a: 1.0 };

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


    let chunk_count = 144;
    // for i in 0..chunk_count {
    //     let mut chunk = Chunk::EMPTY;
    //     let mut dec = 0;
    //     for y in 0..CHUNK_SIZE_16 {
    //         for x in 0..CHUNK_SIZE_16 {
    //             for z in 0..CHUNK_SIZE_16 {
    //                 let r = dec..CHUNK_SIZE_16 - dec;
    //                 *chunk.get_mut(x, y, z) = if !r.contains(&x) || !r.contains(&z) {
    //                     BlockState::EMPTY
    //                 } else {
    //                     material[i % 4].clone()
    //                 };
    //             }
    //         }
    //         if y % 2 != 0 { dec += 1 };
    //     }
    //     chunks.push(chunk);
    // }
    //
    //
    //
    // let size = CHUNK_SIZE_16 as f32;
    // let mut y = 0.0 - size;
    // let mut x = 0.0;
    // let mut z = 0.0;
    // let mut i = 0;
    // if chunk_count >= 36 {
    //     while i < 36 {
    //         chunks[i].set_pos(x, y, z);
    //         chunks[i + 1].set_pos(x, y, 0.0 - size);
    //         chunks[i + 2].set_pos(x, y, size);
    //
    //         chunks[i + 3].set_pos(size, y, z);
    //         chunks[i + 4].set_pos(0.0 - size, y, z);
    //         chunks[i + 5].set_pos(size, y, size);
    //
    //         chunks[i + 6].set_pos(size, y, 0.0 - size);
    //         chunks[i + 7].set_pos(0.0 - size, y, 0.0 - size);
    //         chunks[i + 8].set_pos(0.0 - size, y, size);
    //
    //         i += 9;
    //         y += size;
    //     }
    // }
    //
    //
    // if chunk_count >= 72 {
    //     x = size * 3.0;
    //     y = 0.0 - size;
    //     while i < 72 {
    //         chunks[i].set_pos(x, y, z);
    //         chunks[i + 1].set_pos(x, y, z + 0.0 - size);
    //         chunks[i + 2].set_pos(x, y, z + size);
    //
    //         chunks[i + 3].set_pos(x + size, y, z);
    //         chunks[i + 4].set_pos(x + 0.0 - size, y, z);
    //         chunks[i + 5].set_pos(x + size, y, z + size);
    //
    //         chunks[i + 6].set_pos(x + size, y, 0.0 - size);
    //         chunks[i + 7].set_pos(x + 0.0 - size, y, z + 0.0 - size);
    //         chunks[i + 8].set_pos(x + 0.0 - size, y, z + size);
    //
    //         i += 9;
    //         y += size;
    //     }
    // }
    //
    //
    // if chunk_count >= 108 {
    //     z = size * 3.0;
    //     x = 0.0;
    //     y = 0.0 - size;
    //     while i < 108 {
    //         chunks[i].set_pos(x, y, z);
    //         chunks[i + 1].set_pos(x, y, z + 0.0 - size);
    //         chunks[i + 2].set_pos(x, y, z + size);
    //
    //         chunks[i + 3].set_pos(x + size, y, z);
    //         chunks[i + 4].set_pos(x + 0.0 - size, y, z);
    //         chunks[i + 5].set_pos(x + size, y, z + size);
    //
    //         chunks[i + 6].set_pos(x + size, y, z + 0.0 - size);
    //         chunks[i + 7].set_pos(x + 0.0 - size, y, z + 0.0 - size);
    //         chunks[i + 8].set_pos(x + 0.0 - size, y, z + size);
    //
    //         i += 9;
    //         y += size;
    //     }
    // }
    //
    //
    // if chunk_count >= 144 {
    //     x = size * 3.0;
    //     y = 0.0 - size;
    //     while i < 144 {
    //         chunks[i].set_pos(x, y, z);
    //         chunks[i + 1].set_pos(x, y, z + 0.0 - size);
    //         chunks[i + 2].set_pos(x, y, z + size);
    //
    //         chunks[i + 3].set_pos(x + size, y, z);
    //         chunks[i + 4].set_pos(x + 0.0 - size, y, z);
    //         chunks[i + 5].set_pos(x + size, y, z + size);
    //
    //         chunks[i + 6].set_pos(x + size, y, z + 0.0 - size);
    //         chunks[i + 7].set_pos(x + 0.0 - size, y, z + 0.0 - size);
    //         chunks[i + 8].set_pos(x + 0.0 - size, y, z + size);
    //
    //         i += 9;
    //         y += size;
    //     }
    // }

    let mut chunk_meshes = vec![];
    for x_ch in -20..20 {
        for z_ch in -20..20 {
            let mut chunk = Chunk::EMPTY;
            for x in 0..CHUNK_SIZE_16 {
                for z in 0..CHUNK_SIZE_16 {
                    *chunk.get_mut(x, 1, z) = BlockState::TILE
                }
            }
            build_chunk_model(&mut chunk);
            chunk.set_pos(x_ch as f32  * CHUNK_SIZE_16 as f32, 0.0, z_ch as f32 * CHUNK_SIZE_16 as f32);
            let mesh= build_model_meshes(&chunk.model, atlas.clone(), chunk.get_pos());
            // println!("{}",mesh.len());
            chunk_meshes.extend(mesh);
            chunks.push(chunk);

        }
    }

    setup_mouse_cursor();
    let mut fps_mean = vec![];
    // let mut frame_mean = vec![];
    // let mut math_mean = vec![];
    let push_to_mean = |arr: &mut Vec<usize>, val: usize| -> usize {
        arr.insert(0, val);
        arr.truncate(100);
        arr.iter().sum::<usize>() / 100_usize
    };

    // for mut chunk in chunks.iter_mut() {
    //     build_chunk_model(&mut chunk);
    // }

    let mut frame_scip = 0;
    loop {
        if is_key_pressed(KeyCode::Escape) { break; }

        update_grabbed_state_and_cursor_on_tab_press(&mut grabbed);

        *current_mouse_pos = mouse_position().into();

        // player_pos.x = -50.0;
        // player_pos.y = 42.0;
        // player_pos.z = -128.0;
        // pitch.0 = -0.3;
        // yaw.0 = -4.7;

        if *grabbed {
            update_yaw_pitch_after_mouse_pos_changed(&current_mouse_pos, &last_mouse_pos, &mut yaw, &mut pitch);
            update_player_pos_after_front_right_up_changed(&mut player_pos, front, right);
            update_front_right_up_vecs_after_yaw_pitch_changed(&yaw, &pitch, &mut front, &mut right, &mut up);
        }


        // grabbed.0 = false;

        clear_background(SKY_COLOR);

        set_camera(&Camera3D {
            position: *player_pos,
            up: *up,
            target: *player_pos + *front,
            ..Default::default()
        });

        draw_grid(20, 1., BLACK, GRAY);

        // let mut mesh_time = 0;
        // let mut draw_time = 0;
        let now = Instant::now();
        // for mut chunk in chunks.iter_mut() {
        //     chunk.check_visibility(player_pos.0, front.0);
        //     if chunk.is_visible {
        //         update_chunk_model(&mut chunk, player_pos.0);
        //         // let t2 = Instant::now();
        //         let chunk_meshes = build_model_meshes(&chunk.model, atlas.clone(), chunk.get_pos());
        //         // mesh_time += t2.elapsed().as_micros();
        //         // println!("mesh_time {}", mesh_time);
        //
        //         // let t3 = Instant::now();
        //         for chunk_mesh in &chunk_meshes {
        //             draw_mesh(&chunk_mesh);
        //         }
        //         // draw_time += t3.elapsed().as_micros();
        //     }
        // }

        for chunk_mesh in &chunk_meshes {
            draw_mesh(&chunk_mesh);
        }

        let math = now.elapsed().as_micros() as usize;
        /* Back to screen space */ set_default_camera();
        let fps = get_fps() as usize;
        let mean_fps = push_to_mean(&mut fps_mean, fps);


        // let mean_math = push_to_mean(&mut math_mean, math);
        let info_str = format!("X: {:.2} Y: {:.2} Z: {:.2}", player_pos.x, player_pos.y, player_pos.z);
        let fps_frame_str = format!("FPS: {} Math: {} mcs", mean_fps, math);
        // let fps_str = format!("FPS: {}", mean_fps);
        // let time_str = format!("mesh {} draw {}", mesh_time, draw_time);
        render_text_overlay(info_str.as_str(), 1);
        // render_text_overlay(fps_str.as_str(), 2);
        render_text_overlay(fps_frame_str.as_str(), 2);
        // render_text_overlay(time_str.as_str(), 3);
        last_mouse_pos.0 = mouse_position().into();

        // if frame_scip % 50000 == 0 {
        //     frame_scip = 0;
        //     println!("---\n");
        next_frame().await
        // } else {
        //     frame_scip += 1;
        // }

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

fn render_text_overlay(text: &str, at_line: usize) {
    draw_text(text,
              10.0,
              40.0 * at_line as f32,
              60.0,
              BLACK,
    );
}
