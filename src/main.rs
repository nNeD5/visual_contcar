//use raylib::prelude::*;
use std::fs;

fn parse_poscar(path: &str) -> Vec<Vec<f32>> {
    fs::read_to_string(path)
        .expect("Unable to read {path}")
        .lines()
        .skip(5)
        .skip_while(|&l| !l.starts_with(['d', 'c', 'D', 'C']))
        .skip(1)
        .map(|l|
            l.split_whitespace()
                .filter_map(|i| i.trim().parse::<f32>().ok())
                .collect::<Vec<_>>()
        )
        .collect()
}

fn main() {
    //parse_poscar("POSCAR");
    println!("{:?}", parse_poscar("POSCAR"));
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
}
