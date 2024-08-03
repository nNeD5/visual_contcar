use std::{env, fs, process::ExitCode};
use raylib::prelude::*;
use crate::light::Light;

pub mod light;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("{} usage: visual_poscar path_to_poscar", args[0]);
        return ExitCode::from(1);
    }
    let poses = parse_contcar(&args[1]);
    run_raylib(&poses);

    ExitCode::SUCCESS
}

fn parse_contcar(path: &str) -> Vec<Vec<f32>> {
    fs::read_to_string(path)
        .expect("Unable to read {path}")
        .lines()
        .skip(5)
        .skip_while(|l| !l.starts_with(['d', 'c', 'D', 'C']))
        .skip(1)
        .take_while(|l| l.trim().len() > 0)
        .map(|l| {
            l.split_whitespace()
                .filter_map(|i| i.trim().parse::<f32>().ok())
                .collect()
        })
        .collect()
}

fn get_min(poses: &Vec<Vec<f32>>) -> (f32, f32, f32) {
    let (mut min_x, mut min_y, mut min_z) = (f32::INFINITY, f32::INFINITY, f32::INFINITY);
    for pos in poses {
        let (x, y, z) = (pos[0], pos[1], pos[2]);
        if x < min_x {
            min_x = x
        };
        if y < min_y {
            min_y = y
        };
        if z < min_z {
            min_z = z
        };
    }
    println!("min: {min_x}, {min_y}, {min_z}");
    (min_x, min_y, min_z)
}

fn get_max(poses: &Vec<Vec<f32>>) -> (f32, f32, f32) {
    let (mut max_x, mut max_y, mut max_z) =
        (f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);
    for pos in poses {
        let (x, y, z) = (pos[0], pos[1], pos[2]);
        if x > max_x {
            max_x = x
        };
        if y > max_y {
            max_y = y
        };
        if z > max_z {
            max_z = z
        };
    }
    println!("max: {max_x}, {max_y}, {max_z}");
    (max_x, max_y, max_z)
}

fn center(poses: &Vec<Vec<f32>>) -> Vec<f32> {
    let (max_x, max_y, max_z) = get_max(poses);
    let (min_x, min_y, min_z) = get_min(poses);
    let center = vec![
        (min_x + max_x) / 2.0,
        (min_y + max_y) / 2.0,
        (min_z + max_z) / 2.0,
    ];
    println!("cetner: {:?}", center);
    center
}

fn run_raylib(poses: &Vec<Vec<f32>>) {
    let w = 1200;
    let h = 800;
    let r = 0.05;
    let c = center(poses);
    let centered_poses = {
        let mut cp = Vec::with_capacity(poses.len());
        for pos in poses {
            cp.push(Vec::from([pos[0] - c[0], pos[1] - c[1], pos[2] - c[2]]));
        }
        cp
    };
    println!("centered_poses: {:?}", centered_poses);

    let (mut rl, thread) = raylib::init().size(w, h).title("Model").build();
    rl.set_target_fps(60);
    rl.disable_cursor();

    let mut camera = raylib::camera::Camera::perspective(
        rvec3(1.5, 1.5, 1.5),
        rvec3(0, 0, 0),
        Vector3::up(),
        45.0,
    );

    let mut point_model = unsafe {
        rl.load_model_from_mesh(
            &thread,
            Mesh::gen_mesh_sphere(&thread, r, 32, 32).make_weak(),
        )
        .expect("Unable to gen meh sphere")
    };

    let mut shader = unsafe {
        rl.load_shader(
            &thread,
            Some("resources/lighting.vs"),
            Some("resources/lighting.fs"),
        )
        .unwrap()
        .make_weak()
    };
    shader.locs_mut()[raylib::consts::ShaderLocationIndex::SHADER_LOC_MATRIX_MODEL as usize] =
        shader.get_shader_location("matModel");
    shader.locs_mut()[raylib::consts::ShaderLocationIndex::SHADER_LOC_VECTOR_VIEW as usize] =
        shader.get_shader_location("viewPos");

    let ambient_loc = shader.get_shader_location("ambient");
    shader.set_shader_value(ambient_loc, Vector4::new(0.2, 0.2, 0.2, 1.0));
    point_model.materials_mut()[0].shader = *shader;
    let mut light = Light::new(
        rvec3(camera.position.x, camera.position.y + 0.5, camera.position.z),
        rvec3(c[0], c[1], c[2]),
        Color::WHITE,
        &mut shader,
    );
    while !rl.window_should_close() {
        rl.update_camera(&mut camera, CameraMode::CAMERA_THIRD_PERSON);
        light.position = camera.position;
        light.update_light_values(&mut shader);

        let mut draw = rl.begin_drawing(&thread);
        draw.clear_background(Color::BLACK);
        let mut draw = draw.begin_mode3D(&camera);
        draw.draw_grid(7, 0.5);

        for pos in &centered_poses {
            draw.draw_model(&point_model, rvec3(pos[0], pos[1], pos[2]), 1.0, Color::RED);
        }
   }
}
