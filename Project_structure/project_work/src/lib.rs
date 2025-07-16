mod diet;
mod weightlifting;
mod cardio;

use cardio::{CardioTool, Exercise as CardioExercise};

use weightlifting::Exercise as WeightliftingExercise;

#[derive(Debug)]
pub struct GymWorkout{
    cardio: CardioExercise,
    weightlifting: WeightliftingExercise,
}

impl GymWorkout {
    pub fn new() -> Self{
        diet::ask_about_program();
        weightlifting::ask_about_program();
        cardio::ask_about_program();

        Self {
            cardio: CardioExercise::new(String::from("poorvanshi"), CardioTool::Treadmill, 23),
            weightlifting: WeightliftingExercise::new(String::from("weight name"), 34)
        }
    }
}