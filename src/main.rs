//use raylib::prelude::*;
use std::{process::ExitCode, fs, env};

fn parse_poscar(path: &str) -> Vec<Vec<f32>> {
    fs::read_to_string(path)
        .expect("Unable to read {path}")
        .lines()
        .skip(5)
        .skip_while(|l| !l.starts_with(['d', 'c', 'D', 'C']))
        .skip(1)
        .take_while(|l| l.trim().len() > 0)
        .map(|l|
            l.split_whitespace()
                .filter_map(|i| i.trim().parse::<f32>().ok())
                .collect::<Vec<_>>()
        )
        .collect()
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("{} usage: visual_poscar path_to_poscar", args[0]);
        return ExitCode::from(1);
    }
    println!("Points: {:?}", parse_poscar("POSCAR_2"));
    //let w = 1200;
    //let h = 800;
    //let r = 20.0;
    //let (mut rl, thread) = raylib::init()
    //    .size(w, h)
    //    .title("Hello from rust")
    //    .build();
    //
    //while !rl.window_should_close() {
    //    let mut d = rl.begin_drawing(&thread);
    //    d.clear_background(Color::BLACK);
    //    d.draw_circle(h/2, h/2, r, Color::RED);
    //}
    ExitCode::SUCCESS
}
