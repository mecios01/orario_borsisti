use crate::types::timetable::{Day, Turn, TurnHours};

#[derive(Copy, Clone)]
pub struct Preference {
    pub day: Day,
    pub turn: Turn,
}

impl Preference {
    pub fn new(day: Day, turn: Turn) -> Self {
        Self {
            day,
            turn,
        }
    }
}

pub struct Person {
    pub name: String,
    pub surname: String,
    pub preferences: Vec<Preference>,
    pub tot_hours: f32,
    //amount of hours before timetable
    pub worked_hours: f32,
}

impl Person {
    pub fn new(name: &str, surname: &str) -> Self {
        Self {
            name: name.to_string(),
            surname: surname.to_string(),
            preferences: vec![],
            tot_hours: 150.0,
            worked_hours: 0.0,
        }
    }
    pub fn with_preferences(name: &str, surname: &str, preferences: Vec<Preference>, worked_hours: f32) -> Self {
        Self {
            name: name.to_string(),
            surname: surname.to_string(),
            preferences,
            tot_hours: 150.0,
            worked_hours,
        }
    }
}




