use good_lp::{Constraint, default_solver, Expression, Solution, SolverModel, Variable, VariableDefinition, variables};
use good_lp::solvers::coin_cbc::{CoinCbcProblem, CoinCbcSolution};

fn main() {
    // Define the number of workers, days, and shifts
    let num_workers = 7;
    let num_days = 5;
    let num_shifts = 2;
    const MAX_HOURS_PER_WEEK: i32 = 10;
    const MIN_HOURS_PER_WEEK: i32 = 1; //could be 4 or more. Ensure that everyone gets a turn

    let mut model = variables!();
    let rem_hours = [128, 126, 127, 127, 128, 146, 131];
    let preference = [
        [[true, true], [false, false], [true, false], [false, true], [true, false]],//io
        [[false, false], [false, false], [true, false], [false, true], [true, false]],//luca
        [[false, false], [false, false], [false, false], [true, false], [false, true]],//daniele
        [[false, false], [false, false], [false, false], [true, false], [false, true]],//giovanni
        [[false, false], [true, false], [false, true], [false, false], [false, false]],//vincenzo
        [[false, false], [false, false], [true, false], [false, false], [false, false]],//niko
        [[false, false], [true, true], [false, true], [true, true], [true, true]],//domenico
    ];

    //sanity checks must be performed before building the model (all turns can be covered?)

    // Choice variable => indicates which person is going to be working on each shift
    let mut p = vec![vec![Vec::<Variable>::new(); num_days]; num_workers];

    // Add all the variables to the model
    for i in 0..num_workers {
        for d in 0..num_days {
            for s in 0..num_shifts {
                let y =
                    VariableDefinition::new().binary().name(format!("p_{}_{}_{}", i, d, s));
                p[i][d].push(model.add(y));
            }
        }
    }

    //Constraint 1: One person per shift (skip if no preference was provided)
    let mut c1 = vec![];
    for d in 0..num_days {
        for s in 0..num_shifts {
            let mut e = Expression::default();
            for i in 0..num_workers {
                e += p[i][d][s].clone(); //it is just an index to a variable of the problem so we are ok
            }
            c1.push(e.eq(1)); //this restricts the shift to only one person at time (for now)
        }
    }

    // Constraint 2: Limit max and min hours per week per person (12 default)
    let mut c2 = vec![];

    for i in 0..num_workers {
        let mut e = Expression::default();
        for d in 0..num_days {
            for s in 0..num_shifts {
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
                e.add_mul(hours, p[i][d][s].clone());
            }
        }
        c2.push(e.clone().geq(MIN_HOURS_PER_WEEK));
        c2.push(e.leq(MAX_HOURS_PER_WEEK)); //optionally can be increased (per person or globally)
    }

    // // Constraint 3: No consecutive shifts
    // let mut c3 = vec![];
    // for i in 0..num_workers {
    //     for d in 0..num_days {
    //         let mut e = Expression::default();
    //         for s in 0..num_shifts {
    //             e += p[i][d][s].clone();
    //         }
    //         c3.push(e.clone().leq(1));
    //     }
    // }

    //Force no shifts without preference expressed

    //Objective function: min(SUM:hours_remaining*(hours_remaining-hours_this_week))
    let mut obj = Expression::default();
    for i in 0..num_workers {
        let mut wh = Expression::default();
        for d in 0..num_days {
            for s in 0..num_shifts {
                if preference[i][d][s] {
                    match s {
                        0 => { wh.add_mul(4, p[i][d][s].clone()) }
                        1 => {
                            match d {
                                0..=3 => {
                                    wh.add_mul(6, p[i][d][s]);
                                }
                                4 => {
                                    wh.add_mul(5, p[i][d][s].clone());
                                }
                                _ => unreachable!()
                            }
                        }
                        _ => { unreachable!() }
                    }
                }
            }
        }
        let e = rem_hours[i] - wh;
        // rem_hours * (rem_hours - this_week_hours) ==> the outer gives importance to who has less hours
        obj.add_mul(rem_hours[i], e);
    }

    let mut res = model.minimise(obj).using(default_solver);

    add_vec_contraints(&mut res, c1);
    add_vec_contraints(&mut res, c2);
    // add_vec_contraints(&mut res, c3);

    let solution = res.solve();
    match solution {
        Ok(sol) => {
            print_sol(&sol);
            print_person_caledar(&sol, p, &preference, &rem_hours);
        }
        Err(e) => {
            println!("{}", e)
        }
    }
}

fn add_vec_contraints(prob: &mut CoinCbcProblem, constraints: Vec<Constraint>) {
    for c in constraints.into_iter() {
        prob.add_constraint(c);
    }
}

fn print_sol(solution: &CoinCbcSolution) {
    solution.model().print_solution()
}

fn get_hours_by_day_turn(variable: f64, day: usize, turn: usize) -> usize {
    if variable != 1f64 { return 0; }
    match turn {
        0 => { 4 }
        1 => {
            match day {
                0..=3 => { 6 }
                4 => { 5 }
                _ => unreachable!()
            }
        }
        _ => { unreachable!() }
    }
}

fn print_person_caledar(solution: &CoinCbcSolution, variables: Vec<Vec<Vec<Variable>>>, preferences: &[[[bool; 2]; 5]; 7], remaining: &[i32]) {
    for i in 0..variables.len() {
        let mut tot_hours = 0usize;
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
        println!("REMAINING: {:02}h\n", remaining[i] as usize - tot_hours);
    }
}