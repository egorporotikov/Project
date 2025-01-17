use wasm_bindgen::prelude::*;
use rand::seq::SliceRandom;  // Optional, if you decide to use rand for shuffling

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

    if ![5, 10, 15].contains(&minutes) {
        return "Please select one of the available options: 5, 10, or 15 minutes.".to_string();
    }

    let mut rng = rand::thread_rng();
    let mut shuffled_exercises = exercises.clone();
    shuffled_exercises.shuffle(&mut rng); // Shuffle the exercises

    // Pick the first `minutes` exercises after shuffling
    let selected_exercises = shuffled_exercises.into_iter().take(minutes as usize).collect::<Vec<_>>();

    let mut program = format!("Your {}-minute warm-up program:\n", minutes);
    for (i, exercise) in selected_exercises.iter().enumerate() {
        program.push_str(&format!("{}. {}: {} minute{}\n", i + 1, exercise, 1, if minutes == 1 { "" } else { "s" }));
    }

    program.push_str("\nWe hope you start your day feeling great! ☀️");
    program
}
