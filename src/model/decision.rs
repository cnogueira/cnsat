use model::Literal;
use fnv::FnvHashSet;
use model::ClauseId;

#[derive(Debug)]
pub struct Decision {
    literal: Literal,
    level: u32,
    satisfied_clauses: FnvHashSet<ClauseId>,
}

impl Decision {
    pub fn from(literal: Literal, level: u32) -> Self {
        Decision {
            literal,
            level,
            satisfied_clauses: FnvHashSet::default(),
        }
    }

    pub fn lit(&self) -> Literal {
        self.literal
    }

    pub fn lvl(&self) -> u32 {
        self.level
    }

    pub fn add_satisfied_clause(&mut self, clause_id: ClauseId) {
        self.satisfied_clauses.insert(clause_id);
    }
}
