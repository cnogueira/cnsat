extern crate fnv;
extern crate dimacs;

use parser::parse_dimacs_file;
use std::env;
use model::clause_vec_to_string;
use model::ClauseVec;

mod solver;
mod model;
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

    let clause_set = parse_result.unwrap();
    let clause_vec: ClauseVec = clause_set.into_iter().collect();

    println!("clause_set: {}", clause_vec_to_string(&clause_vec));
}
