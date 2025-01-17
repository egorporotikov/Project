use wasm_bindgen::prelude::*;
use rand::seq::SliceRandom;

#[wasm_bindgen]
pub fn generate_program(total_minutes: u32) -> String {
    let exercises = vec![
        "Deep breathing",
        "Neck tilts",
        "Arm stretches",
        "Gentle torso twists",
        "Light squats",
        "Jumping in place",
        "Jogging in place",
        "Lunges",
        "Side stretches",
        "Hip rotations",
        "High knees",
        "Calf raises",
        "Push-ups (knee or standard)",
        "Wall sit",
        "Plank",
        "Side planks",
        "Leg swings",
        "Cat-cow stretch",
        "Child's pose",
        "Butterfly stretch",
        "Shoulder rolls",
        "Wrist stretches",
        "Forward bends",
        "Ankle rotations",
    ];

    if ![5, 10, 15].contains(&total_minutes) {
        return "Please select one of the available options: 5, 10, or 15 minutes.".to_string();
    }

    let mut rng = rand::thread_rng();
    let mut shuffled_exercises = exercises.clone();
    shuffled_exercises.shuffle(&mut rng);

    let mut remaining_time = total_minutes;
    let mut program = format!("Your {}-minute warm-up program:\n", total_minutes);
    let mut i = 0;

    while remaining_time > 0 && i < shuffled_exercises.len() {
        let exercise_time = if remaining_time >= 2 { 2 } else { 1 }; // Assign 2 minutes if possible, else 1
        program.push_str(&format!(
            "{}. {}: {} minute{}\n",
            i + 1,
            shuffled_exercises[i],
            exercise_time,
            if exercise_time == 1 { "" } else { "s" }
        ));
        remaining_time -= exercise_time;
        i += 1;
    }

    program.push_str("\nWe hope you start your day feeling great!!! ☀️");
    program
}
