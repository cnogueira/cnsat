use model::Literal;
use model::LiteralVec;
use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Clause {
    lits: LiteralVec,
}

impl Clause {
    pub fn from_dimacs_clause(clause: &dimacs::Clause) -> Self {
        let clause_lits: LiteralVec = clause.lits().iter()
            .map(|&lit| Literal::from_dimacs_lit(lit))
            .collect();

        Clause { lits: clause_lits }
    }

    pub fn lits(&self) -> &[Literal] {
        &self.lits
    }
}

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted_lit_list: Vec<_> = self.lits.iter()
            .map(|lit| format!("{}", lit)).collect();

        write!(f, "{{{}}}", formatted_lit_list.join(", "))
    }
}
