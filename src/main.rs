#![allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss
)]

use flip_18::FlipSimulation;
use fluid_sim_17::{FluidSimulation, SceneType};
use macroquad::prelude::*;
use softbodies_10::SoftBodiesSimulation;

mod cloth_14;
mod flip_18;
mod fluid_sim_17;
mod hashing_11;
mod mesh;
mod self_collision_15;
mod softbodies_10;
mod softbody_skinning_12;

#[macroquad::main("FLOATING")]
async fn main() {
    let mut sim = init_18();
    loop {
        step_18(&mut sim);

        next_frame().await
    }
}

pub fn init_18() -> FlipSimulation {
    FlipSimulation::new(480.0, 360.0)
}

pub fn step_18(sim: &mut FlipSimulation) {
    set_camera(&Camera2D {
        zoom: vec2(0.5, 0.5),
        target: vec2(2.1, 1.5),
        ..Default::default()
    });

    clear_background(LIGHTGRAY);

    draw_circle(
        sim.obstacle_pos.x,
        sim.obstacle_pos.y,
        sim.obstacle_radius,
        BLUE,
    );

    for (i, pos) in sim.particle_pos.iter().enumerate() {
        if i % 8 == 0 {
            draw_circle(pos.x, pos.y, 0.008, BLACK);
        }
    }

    sim.step();
}

pub fn init_17() -> FluidSimulation {
    let mut sim = FluidSimulation::new(SceneType::Paint, 480.0, 360.0);

    sim
}

pub fn step_17(sim: &mut FluidSimulation) {
    clear_background(LIGHTGRAY);

    draw_circle(
        sim.obstacle_pos.x,
        sim.obstacle_pos.y,
        sim.obstacle_radius,
        BLUE,
    );

    sim.step();
}

pub fn init_10() -> SoftBodiesSimulation {
    const DEFAULT_NUM_SOLVER_SUBSTEPS: u8 = 10;
    const DEFAULT_EDGE_COMPLIANCE: f32 = 100.0;
    const DEFAULT_VOL_COMPLIANCE: f32 = 0.0;

    let mut sim = SoftBodiesSimulation::new(
        DEFAULT_NUM_SOLVER_SUBSTEPS,
        DEFAULT_EDGE_COMPLIANCE,
        DEFAULT_VOL_COMPLIANCE,
    );

    sim
}

pub fn step_10(sim: &mut SoftBodiesSimulation) {
    clear_background(LIGHTGRAY);

    // Going 3d!

    set_camera(&Camera3D {
        position: vec3(-2., 1.5, 0.),
        up: vec3(0., 1., 0.),
        target: vec3(0., 0., 0.),
        ..Default::default()
    });

    draw_grid(20, 1., BLACK, GRAY);

    for body in &sim.bodies {
        for pos in &body.pos {
            draw_sphere(*pos, 0.008, None, BLACK);
        }
    }

    // Back to screen space, render some text
    set_default_camera();
    draw_text("WELCOME TO 3D WORLD", 10.0, 20.0, 30.0, BLACK);

    sim.step();
}

#[allow(clippy::many_single_char_names)]
fn get_sci_color(val: f32, min: f32, max: f32) -> [f32; 3] {
    let mut val = val.clamp(min, max - 0.0001);
    let d = max - min;
    val = if d == 0.0 { 0.5 } else { (val - min) / d };
    let m = 0.25;
    let num = f32::floor(val / m);
    let s = (val - num * m) / m;
    let (r, g, b) = match num as u8 {
        0 => (0.0, s, 1.0),
        1 => (0.0, 1.0, 1.0 - s),
        2 => (s, 1.0, 0.0),
        3 => (1.0, 1.0 - s, 0.0),
        _ => (1.0, 0.0, 0.0),
    };
    [r, g, b]
}

fn get_sci_color_255(val: f32, min: f32, max: f32) -> [f32; 3] {
    let [r, g, b] = get_sci_color(val, min, max);
    [255.0 * r, 255.0 * g, 255.0 * b]
}
