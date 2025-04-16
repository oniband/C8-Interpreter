use raylib::prelude::*;
use std::fs::File;

mod cpu;
use crate::cpu::{Cpu, Instruction};

mod util;
use crate::util::validate_args;

const WINDOW_WIDTH: i32 = 1920;
const WINDOW_HEIGHT: i32 = 1080;

const GAME_WIDTH: f32 = 640.0;
const GAME_HEIGHT: f32 = 320.0;
const GAME_PIXEL_SCALE_FACTOR: i32 = 10;

const GAME_POSITION_OFFSET: i32 = 345;
const TOP_BAR_OFFSET: f32 = 30.0;

fn main() -> std::io::Result<()> {
    let mut cpu = Cpu::new();
    match validate_args() {
        Ok(value) => {
            let mut program = File::open(value)?;
            cpu.load_program_into_memory(&mut program);
        }
        Err(err) => {
            panic!("{err}");
        }
    }

    // GUI INITIAL VALUES //
    let memory_container_box_bounds = Rectangle::new(
        0.0,
        TOP_BAR_OFFSET,
        (WINDOW_WIDTH as f32 / 2.0) - (GAME_WIDTH / 2.0),
        WINDOW_HEIGHT as f32 - TOP_BAR_OFFSET,
    );

    let instruction_container_box_bounds = Rectangle::new(
        (WINDOW_WIDTH as f32 / 2.0) + (GAME_WIDTH / 2.0),
        TOP_BAR_OFFSET,
        (WINDOW_WIDTH as f32 / 2.0) - (GAME_WIDTH / 2.0),
        WINDOW_HEIGHT as f32 - TOP_BAR_OFFSET,
    );

    let game_container_box_bounds = Rectangle::new(
        (WINDOW_WIDTH as f32 / 2.0) - (GAME_WIDTH / 2.0),
        (WINDOW_HEIGHT as f32 / 2.0) - 510.0,
        640.0,
        320.0,
    );
    // GUI INITIAL VALUES //

    let (mut rl, thread) = raylib::init()
        .width(WINDOW_WIDTH)
        .height(WINDOW_HEIGHT)
        .title("C8-Emu")
        .build();
    rl.set_target_fps(cpu.clock_speed);
    // rl.set_exit_key(Some(KeyboardKey::KEY_X));
    while !rl.window_should_close() {
        if !cpu.should_halt {
            let instruction: Instruction = cpu.fetch();
            cpu.decode_and_execute(instruction);
        }

        let mut d = rl.begin_drawing(&thread);

        // GUI RENDERING //
        d.gui_set_style(GuiControl::DEFAULT, GuiDefaultProperty::TEXT_SIZE, 20);
        d.gui_group_box(memory_container_box_bounds, "MEMORY");
        d.gui_group_box(game_container_box_bounds, "GAME");
        d.gui_group_box(instruction_container_box_bounds, "INSTRUCTIONS");
        // GUI RENDERING //

        d.clear_background(Color::BLACK);
        for y in 0..32 {
            for x in 0..64 {
                if cpu.pixel_buffer[y][x] {
                    d.draw_rectangle(
                        (x as i32 * GAME_PIXEL_SCALE_FACTOR) + (WINDOW_WIDTH / 2)
                            - (GAME_WIDTH as i32 / 2),
                        (y as i32 * GAME_PIXEL_SCALE_FACTOR) + (WINDOW_HEIGHT / 2)
                            - (GAME_HEIGHT as i32 / 2)
                            - GAME_POSITION_OFFSET,
                        GAME_PIXEL_SCALE_FACTOR,
                        GAME_PIXEL_SCALE_FACTOR,
                        Color::WHITE,
                    );
                }
            }
        }
    }
    Ok(())
}
