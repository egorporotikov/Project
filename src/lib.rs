use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate_program(minutes: u32) -> String {
    let mut program = format!("Your {}-minute warm-up program:\n", minutes);

    match minutes {
        5 => {
            program.push_str("1. Deep breathing: 1 minute\n");
            program.push_str("2. Neck tilts: 1 minute\n");
            program.push_str("3. Arm stretches: 1 minute\n");
            program.push_str("4. Gentle torso twists: 1 minute\n");
            program.push_str("5. Light squats: 1 minute\n");
        }
        10 => {
            program.push_str("1. Deep breathing: 1 minute\n");
            program.push_str("2. Neck tilts: 1 minute\n");
            program.push_str("3. Arm stretches: 2 minutes\n");
            program.push_str("4. Torso twists: 2 minutes\n");
            program.push_str("5. Light squats: 2 minutes\n");
            program.push_str("6. Jumping in place: 2 minutes\n");
        }
        15 => {
            program.push_str("1. Deep breathing: 2 minutes\n");
            program.push_str("2. Neck tilts: 2 minutes\n");
            program.push_str("3. Arm stretches: 3 minutes\n");
            program.push_str("4. Torso twists: 3 minutes\n");
            program.push_str("5. Squats with extended arms: 3 minutes\n");
            program.push_str("6. Jumping in place or jogging in place: 2 minutes\n");
        }
        _ => program.push_str("Error: invalid number of minutes.\n"),
    }

    program
}
