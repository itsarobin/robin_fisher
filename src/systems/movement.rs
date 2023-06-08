use bevy::prelude::{Input, KeyCode, Res};

const MOVE_STEP: i8 = 10;

pub fn keyboard_input(input: Res<Input<KeyCode>>){
    println!("NEW FRAME");

    for key in input.get_pressed() {
        match key {
            KeyCode::W => {
                println!("W PRESSED");
            },
            KeyCode::S => {
                println!("S PRESSED");
            },
            KeyCode::A => {
                println!("A PRESSED");
            },
            KeyCode::D => {
                println!("D PRESSED");
            },
            _ => (),
        }
    }
}