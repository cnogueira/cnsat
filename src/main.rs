extern crate fnv;
extern crate dimacs;

use parser::parse_dimacs_file;
use std::env;
//use model::clause_vec_to_string;
use solver::Solver;

mod solver;
mod model;
mod decider;
mod conflict_analyzer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("must provide a .cnf filename!");
        return;
    }

    let parse_result = parse_dimacs_file(&args[1]);

    if parse_result.is_err() {
        println!("Error: {}", parse_result.err().unwrap());
        return;
    }

    let mut solver = Solver::new();

    parse_result.unwrap().into_iter().for_each(|clause| {
        solver.add_clause(clause);
    });

    let solution = solver.solve();

    match solution {
        Some(model) => println!("SAT\nmodel: {:?}", model),
        None => println!("UNSAT"),
    }
}
