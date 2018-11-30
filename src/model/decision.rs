use model::Literal;
use fnv::FnvHashSet;
use model::ClauseId;
use fnv::FnvHashMap;
use solver::Constant;

#[derive(Debug)]
pub struct Decision {
    literal: Literal,
    level: u32,
    satisfied_clauses: FnvHashSet<ClauseId>,
    propagated_lits: FnvHashMap<Literal, ClauseId>,
    conflict_lit: Option<Literal>,
}

impl Decision {
    pub fn from(literal: Literal, level: u32) -> Self {
        Decision {
            literal,
            level,
            satisfied_clauses: FnvHashSet::default(),
            propagated_lits: FnvHashMap::default(),
            conflict_lit: None,
        }
    }

    pub fn print_status(&self) {
        println!("= decision: {}@{}", self.lit(), self.lvl());
        println!("= satisfied_clauses: {:?}", self.satisfied_clauses);
        println!("= propagated_lits: {:?}", self.propagated_lits);
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

    pub fn satisfied_clauses(&self) -> &FnvHashSet<ClauseId> {
        &self.satisfied_clauses
    }

    pub fn add_propagated_lit(&mut self, lit: Literal, clause_id: ClauseId) -> Constant {
        self.propagated_lits.insert(lit, clause_id);

        if self.propagated_lits.contains_key(&lit.complementary()) {
            self.conflict_lit = Some(lit);

            return Constant::Conflict;
        }

        return Constant::NoConflict;
    }

    pub fn implying_clause_of(&self, lit: Literal) -> Option<ClauseId> {
        self.propagated_lits.get(&lit).cloned()
    }

    pub fn implying_clauses_iter(&self) -> impl Iterator<Item=ClauseId> + '_ {
        self.propagated_lits.iter().map(|a| *a.1)
    }

    #[inline]
    pub fn get_conflict_lit(&self) -> Option<Literal> {
        self.conflict_lit
    }

    #[inline]
    pub fn propagated_lits_iter(&self) -> impl Iterator<Item=Literal> + '_ {
        self.propagated_lits.iter().map(|a| *a.0)
    }

    #[inline]
    pub fn propagated_lits_len(&self) -> usize {
        self.propagated_lits.len()
    }
}
