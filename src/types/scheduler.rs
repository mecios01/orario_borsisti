use std::collections::HashMap;
use std::mem;
use std::ops::Sub;
use good_lp::{Constraint, constraint, default_solver, Expression, ProblemVariables, Solution, SolverModel, Variable, VariableDefinition, variables};
use good_lp::solvers::coin_cbc::{CoinCbcProblem, CoinCbcSolution};
use good_lp::variable::FormatWithVars;
use crate::types::person::Person;
use crate::types::timetable::Timetable;

pub enum ConstraintType {
    MaxOnePersonPerShift,
    MinMaxWeekHoursPerPerson(f64, f64),
    NoConsecutiveShifts,
}

pub struct Scheduler {
    ///model constants
    max_hours_per_week: usize,
    min_hours_per_week: usize,
    num_workers: usize,
    num_days: usize,
    num_shifts: usize,
    model: ProblemVariables,
    rem_hours: Vec<f64>,
    preferences: Vec<Vec<Vec<bool>>>,
    ///each constraint is, in general an array of constraints
    constraints: Vec<Vec<Constraint>>,
    // objective_function: Option<Expression>,
    people_var: Vec<Vec<Vec<Variable>>>,
}

impl Default for Scheduler {
    fn default() -> Self {
        Self {
            max_hours_per_week: 0,
            min_hours_per_week: 0,
            num_workers: 0,
            num_days: 0,
            num_shifts: 0,
            model: variables!(),
            constraints: vec!(),
            preferences: vec!(),
            rem_hours: vec!(),
            // objective_function: None,
            people_var: vec![],
        }
    }
}

impl Scheduler {
    pub fn new(max_hours_per_week: usize,
               min_hours_per_week: usize,
               num_workers: usize,
               num_days: usize,
               num_shifts: usize,
    ) -> Self {
        Self {
            max_hours_per_week,
            min_hours_per_week,
            num_workers,
            num_days,
            num_shifts,
            ..Default::default()
        }
    }
    pub fn setup(&mut self, timetable: &Timetable) -> &mut Self {
        //GET PREFERENCES AND REMAINING HOURS
        let (pref, rem) = timetable.get_people_preferences_and_rem_hours();
        self.add_preferences(pref);
        self.add_remaining_hours(rem);
        self.add_variables_to_model(&timetable.people);
        self
    }
    pub fn add_remaining_hours(&mut self, remaining_hours: Vec<f64>) -> &mut Self {
        self.rem_hours = remaining_hours;
        self
    }
    pub fn add_preferences(&mut self, preferences: Vec<Vec<Vec<bool>>>) -> &mut Self {
        self.preferences = preferences;
        self
    }

    fn add_constraint(&mut self, constraint: Vec<Constraint>) -> &mut Self {
        self.constraints.push(constraint);
        self
    }

    fn add_variables_to_model(&mut self, people: &Vec<Person>) -> &mut Self {
        // Choice variable => indicates which person is going to be working on each shift
        let mut p = vec![vec![Vec::<Variable>::new(); self.num_days]; self.num_workers];

        // Add all the variables to the model
        for i in 0..self.num_workers {
            for d in 0..self.num_days {
                for s in 0..self.num_shifts {
                    let y =
                        VariableDefinition::new().binary().name(format!("p_{}_{}_{}_{}", i, d, s, people[i].acronym()));
                    p[i][d].push(self.model.add(y));
                }
            }
        }
        self.people_var = p;
        self
    }

    // pub fn define_objective_function(&mut self, objective: Expression) -> &mut Self {
    //     self.objective_function = Some(objective);
    //     self
    // }
    pub fn set_constraint(&mut self, constraint: ConstraintType) {
        match constraint {
            ConstraintType::MaxOnePersonPerShift =>
                self.add_constraint(set_max_one_person_per_shift(&self)),
            ConstraintType::MinMaxWeekHoursPerPerson(min, max) =>
                self.add_constraint(set_min_max_hours_per_week(&self, min, max)),
            ConstraintType::NoConsecutiveShifts =>
                self.add_constraint(set_no_consecutive_shifts(&self))
        };
    }
    pub fn schedule(&mut self) -> &mut Self {
        let model = mem::take(&mut self.model);
        let mut problem =
            model.minimise(default_objective_function(&self))
                .using(default_solver);

        let constraints = mem::take(&mut self.constraints);
        for c in constraints.into_iter() {
            add_vec_contraints(&mut problem, c);
        }

        let people_var = mem::take(&mut self.people_var);
        let solution = problem.solve();
        match solution {
            Ok(sol) => {
                print_sol(&sol);
                print_person_caledar(&sol, people_var, &self.preferences, &self.rem_hours);

                //WE CAN DO BETTER THAN THE ABOVE
            }
            Err(e) => {
                println!("{}", e)
            }
        }
        self
    }
}

//constraints functions
fn set_max_one_person_per_shift(scheduler: &Scheduler) -> Vec<Constraint> {
    //Constraint 1: One person per shift (skip if no preference was provided)
    let mut c1 = vec![];
    for d in 0..scheduler.num_days {
        for s in 0..scheduler.num_shifts {
            let mut e = Expression::default();
            for i in 0..scheduler.num_workers {
                e += scheduler.people_var[i][d][s].clone(); //it is just an index to a variable of the problem so we are ok
            }
            c1.push(e.eq(1)); //this restricts the shift to only one person at time (for now)
        }
    }
    c1
}

fn set_min_max_hours_per_week(scheduler: &Scheduler, min: f64, max: f64) -> Vec<Constraint> {
    let mut c2 = vec![];

    for i in 0..scheduler.num_workers {
        let mut e = Expression::default();
        for d in 0..scheduler.num_days {
            for s in 0..scheduler.num_shifts {
                //build a gethours function to retrieve dynamically
                let hours = match s {
                    0 => 4,
                    1 => match d {
                        0..=3 => 6,
                        4 => 5,
                        _ => unreachable!()
                    }
                    _ => unreachable!()
                };
                e.add_mul(hours, scheduler.people_var[i][d][s].clone());
            }
        }
        c2.push(e.clone().geq(min));
        c2.push(e.leq(max)); //optionally can be increased (per person or globally)
    }
    c2
}

fn set_no_consecutive_shifts(scheduler: &Scheduler) -> Vec<Constraint> {
    let mut c3 = vec![];
    for i in 0..scheduler.num_workers {
        for d in 0..scheduler.num_days {
            let mut e = Expression::default();
            for s in 0..scheduler.num_shifts {
                e += scheduler.people_var[i][d][s].clone();
            }
            c3.push(e.clone().leq(1));
        }
    }
    c3
}

fn default_objective_function(scheduler: &Scheduler) -> Expression {
    //Objective function: min(SUM:hours_remaining*(hours_remaining-hours_this_week))
    let mut obj = Expression::default();
    for i in 0..scheduler.num_workers {
        let mut wh = Expression::default();
        for d in 0..scheduler.num_days {
            for s in 0..scheduler.num_shifts {
                if scheduler.preferences[i][d][s] {
                    //TODO: use timetable hours (some days might not be available or some hours can differ)
                    match s {
                        0 => { wh.add_mul(4, scheduler.people_var[i][d][s].clone()) }
                        1 => {
                            match d {
                                0..=3 => {
                                    wh.add_mul(6, scheduler.people_var[i][d][s]);
                                }
                                4 => {
                                    wh.add_mul(5, scheduler.people_var[i][d][s].clone());
                                }
                                _ => unreachable!()
                            }
                        }
                        _ => { unreachable!() }
                    }
                }
            }
        }
        let e = scheduler.rem_hours[i] - wh;
        // rem_hours * (rem_hours - this_week_hours) ==> the outer gives importance to who has less hours
        obj.add_mul(scheduler.rem_hours[i], e);
    }
    obj
}

fn add_vec_contraints(prob: &mut CoinCbcProblem, constraints: Vec<Constraint>) {
    for c in constraints.into_iter() {
        prob.add_constraint(c);
    }
}

fn print_sol(solution: &CoinCbcSolution) {
    solution.model().print_solution()
}

fn print_person_caledar(solution: &CoinCbcSolution, variables: Vec<Vec<Vec<Variable>>>,
                        preferences: &Vec<Vec<Vec<bool>>>, remaining: &Vec<f64>) {
    for i in 0..variables.len() {
        let mut tot_hours = 0f64;
        println!("person:{}", i);
        for d in 0..5 {
            let t1 = solution.eval(variables[i][d][0]);
            let t2 = solution.eval(variables[i][d][1]);
            let p1 = if preferences[i][d][0] { 'x' } else { ' ' };
            let p2 = if preferences[i][d][1] { 'x' } else { ' ' };
            println!("d{}: [{}][{}] - [{}][{}]", d, t1, t2, p1, p2);
            tot_hours += get_hours_by_day_turn(t1, d, 0);
            tot_hours += get_hours_by_day_turn(t2, d, 1);
        }
        println!("TOT WEEK HOURS: {:02}h", tot_hours);
        println!("REMAINING: {:02}h\n", remaining[i] - tot_hours);
    }
}

fn output_result(solution: &CoinCbcSolution, variables: Vec<Vec<Vec<Variable>>>,
                 preferences: &Vec<Vec<Vec<bool>>>, remaining: &Vec<f64>)
                 -> (Vec<(usize, usize)>, Vec<(f64, f64)>) {
    let mut tot_rem: Vec<(f64, f64)> = Vec::new();
    let mut caledar: Vec<(usize, usize)> = Vec::new();

    for i in 0..variables.len() {
        let mut tot_hours = 0f64;
        for d in 0..5 {
            let t1 = solution.eval(variables[i][d][0]);
            let t2 = solution.eval(variables[i][d][1]);
            tot_hours += get_hours_by_day_turn(t1, d, 0);
            tot_hours += get_hours_by_day_turn(t2, d, 1);
        }
        tot_rem.push((tot_hours, remaining[i] - tot_hours));
        //TODO: complete
        // caledar.push(())
    }
    (caledar, tot_rem)
}

fn get_hours_by_day_turn(variable: f64, day: usize, turn: usize) -> f64 {
    if variable != 1f64 { return 0.0; }
    match turn {
        0 => { 4.0 }
        1 => {
            match day {
                0..=3 => { 6.0 }
                4 => { 5.0 }
                _ => unreachable!()
            }
        }
        _ => { unreachable!() }
    }
}