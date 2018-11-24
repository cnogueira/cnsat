use model::ClauseSet;
use dimacs::parse_dimacs as parse_file_content;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use dimacs::Instance;
use model::Clause;
use model::Literal;

pub fn parse_dimacs_file(filename: &str) -> Result<ClauseSet, String> {
    let content = read_file(filename)?;

    let instance = parse_file_content(&content)
        .map_err(|parse_err| format!("Error while parsing: {:?}", parse_err))?;

    let mut clause_set = ClauseSet::default();

    match instance {
        Instance::Cnf { clauses, .. } => {
            clauses.iter().map(|clause| Clause::from_dimacs_clause(clause))
                .for_each(|clause| {
                    clause_set.insert(clause);
                });

            Ok(clause_set)
        },
        _ => Err(String::from("Only .cnf instances are supported")),
    }
}

fn read_file(filename: &str) -> Result<String, String> {
    let file = File::open(filename)
        .map_err(|_err| format!("Cannot open file: {}", filename))?;

    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();

    buf_reader.read_to_string(&mut contents)
        .map_err(|_err| format!("Failed to read contents of file: {}", filename))?;

    Ok(contents)
}