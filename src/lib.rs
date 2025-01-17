use wasm_bindgen::prelude::*;
use rand::seq::SliceRandom;

#[wasm_bindgen]
pub fn generate_program(minutes: u32) -> String {
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

    // Ensure valid input
    if ![5, 10, 15].contains(&minutes) {
        return "Please select one of the available options: 5, 10, or 15 minutes.".to_string();
    }

    let num_exercises = exercises.len() as u32; // Get the total number of available exercises
    let max_exercises = minutes.min(num_exercises); // Pick the smaller of minutes or total exercises

    let mut rng = rand::thread_rng();
    let mut shuffled_exercises = exercises.clone();
    shuffled_exercises.shuffle(&mut rng); // Shuffle the exercises

    // Pick the first `max_exercises` exercises after shuffling
    let selected_exercises = shuffled_exercises.into_iter().take(max_exercises as usize).collect::<Vec<_>>();

    let mut program = format!("Your {}-minute warm-up program:\n", minutes);
    for (i, exercise) in selected_exercises.iter().enumerate() {
        program.push_str(&format!("{}. {}: 1 minute{}\n", i + 1, exercise, if minutes == 1 { "" } else { "s" }));
    }

    program.push_str("\nWe hope you start your day feeling great!!! ☀️");
    program
}
