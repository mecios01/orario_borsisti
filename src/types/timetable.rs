use crate::types::person::Person;
use crate::types::scheduler;
use crate::types::scheduler::{ConstraintType, Scheduler};

#[derive(Copy, Clone)]
pub enum Day {
    Mon = 0,
    Tue = 1,
    Wed = 2,
    Thu = 3,
    Fri = 4,
}

impl Day {
    fn name(&self) -> &'static str {
        match self {
            Self::Mon => "MON",
            Self::Tue => "TUE",
            Self::Wed => "WED",
            Self::Thu => "THU",
            Self::Fri => "FRI"
        }
    }
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
    pub people: Vec<Person>,
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
        //SETUP MODEL
        let mut scheduler = Scheduler::new(12,
                                           1,
                                           7,
                                           5,
                                           2);

        scheduler.setup(&self);
        //CHECK FOR ERRORS/BAD SETUP

        //ADD CONSTRAINTS (enabled ones)
        scheduler.set_constraint(ConstraintType::MaxOnePersonPerShift);
        scheduler.set_constraint(ConstraintType::MinMaxWeekHoursPerPerson(1.0, 12.0));
        // scheduler.set_constraint(ConstraintType::NoConsecutiveShifts);

        //RUN SCHEDULER
        scheduler.schedule();



        //STORE RESULTS IF ALL GOOD ELSE SIGNAL IT


        self
    }

    pub fn get_people_preferences_and_rem_hours(&self) -> (Vec<Vec<Vec<bool>>>, Vec<f64>, ) {
        self.people.iter().map(|p| {
            let rem = p.tot_hours - p.worked_hours;
            let mut pref_def = vec![vec![false; 2]; 5];
            p.preferences.iter().for_each(|pre| {
                let d = pre.day as usize; //MON => 0, ...
                let s = pre.turn as usize; //morning=>0, afternoon=>1
                println!("d{d}.s{s}");
                assert!(d >= 0 && d < 5);
                assert!(s <= 1);
                pref_def[d][s] = true;
            });
            return (pref_def, rem);
        }).fold((vec![], vec![]), |mut acc, (pref, rem)| {
            acc.0.push(pref);
            acc.1.push(rem);
            acc
        })
    }
}