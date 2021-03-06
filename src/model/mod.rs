mod literal;
mod clause;
mod decision;

use fnv::FnvHashSet;

pub use self::literal::Literal;
pub use self::clause::Clause;
pub use self::clause::ClauseId;
pub use self::decision::Decision;

pub type LiteralSet = FnvHashSet<Literal>;
pub type LiteralVec = Vec<Literal>;
pub type ClauseSet = FnvHashSet<Clause>;
pub type ClauseVec = Vec<Clause>;

pub fn clause_vec_to_string(clause_set: &ClauseVec, filter_out: &FnvHashSet<ClauseId>) -> String {
    let formatted_clauses: Vec<_> = clause_set.iter().enumerate()
        .filter(|(id, _clause)| !filter_out.contains(id))
        .map(|(id, clause)| format!("\t{}: {}", id, clause)).collect();

    String::from(format!("{{\n{}\n}}", formatted_clauses.join("\n")))
}
