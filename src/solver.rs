use model::Clause;
use model::LiteralSet;

pub struct Solver {

}

impl Solver {
    pub fn new() -> Self {
        Solver {}
    }

    pub fn add_clause(&mut self, clause: &Clause) {
        unimplemented!();
    }

    pub fn solve(&mut self) -> Option<LiteralSet> {
        unimplemented!();
    }
}