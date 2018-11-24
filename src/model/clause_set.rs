use model::Clause;
use fnv::FnvHashSet;

#[derive(Debug, Clone)]
pub struct ClauseSet {
    clauses: FnvHashSet<Clause>,
}

impl ClauseSet {
    pub fn from
}