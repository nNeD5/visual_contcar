use raylib::prelude::*;
use std::fs;

fn parse_poscar(path: &str) {
    let content = fs::read_to_string(path)
        .expect("Can't read {path}");
    println!("{content}");
}

fn main() {
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
