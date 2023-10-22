use std::future::IntoFuture;
use crate::types::person::Person;

#[derive(Copy, Clone)]
pub enum Day {
    Mon = 0,
    Tue = 1,
    Wed = 2,
    Thu = 3,
    Fri = 4,
}

#[derive(Copy, Clone)]
pub enum Turn {
    Morning,
    Afternoon,
}

#[derive(Copy, Clone)]
pub struct TurnHours(f32);

pub struct Timetable {
    computed: Option<Vec<(Person, Person)>>,
    //stats per person
    stats: Option<()>,
    //hours per turn
    base: Vec<(TurnHours, TurnHours)>,
    //people with preferences
    people: Vec<Person>,
}

impl Default for Timetable {
    fn default() -> Self {
        Self {
            people: vec![],
            base: vec![(TurnHours(4f32), TurnHours(6f32)),//LUN
                       (TurnHours(4f32), TurnHours(6f32)),//MAR
                       (TurnHours(4f32), TurnHours(6f32)),//MER
                       (TurnHours(4f32), TurnHours(6f32)),//GIO
                       (TurnHours(4f32), TurnHours(5f32)),//VEN
            ],
            stats: None,
            computed: None,
        }
    }
}

impl Timetable {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn with_turnhours(base: &[(TurnHours, TurnHours)]) -> Self {
        Self {
            base: base.to_vec(),
            ..Default::default()
        }
    }
    pub fn add_person(&mut self, p: Person) -> &mut Self {
        self.people.push(p);
        self
    }

    pub fn add_people(&mut self, people: Vec<Person>) -> &mut Self {
        self.people = people;
        self
    }

    pub fn set_turnhours(&mut self, turnhours: Vec<(TurnHours, TurnHours)>) -> &mut Self {
        self.base = turnhours;
        self
    }
    pub fn calc(&mut self) -> &mut Self {
        //objectives:
        //  place people in their preferences
        //  make sure they have a nice amount of hours (not too high/not too low)
        //  avoid turns all day long
        //  consider the current amount of hours / missing hours



        self
    }
}