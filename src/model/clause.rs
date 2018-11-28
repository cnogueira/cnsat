use model::Literal;
use model::LiteralVec;
use std::fmt;
use model::LiteralSet;

pub type ClauseId = usize;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Clause {
    lits: LiteralVec,
    lit_a: Literal,
    lit_b: Option<Literal>,
}

impl Clause {
    pub fn from_dimacs_clause(clause: &dimacs::Clause) -> Self {
        let clause_lits: LiteralVec = clause.lits().iter()
            .map(|&lit| Literal::from_dimacs_lit(lit))
            .collect();
        let lit_a = clause_lits.get(0).cloned().expect("A clause cannot be empty");
        let lit_b = clause_lits.get(1).cloned();

        Clause {
            lits: clause_lits,
            lit_a,
            lit_b,
        }
    }

    pub fn new_asserting_clause(last_lit: Literal, mut strengthen_lits: LiteralVec) -> Self {
        strengthen_lits.push(last_lit);

        Clause {
            lits: strengthen_lits,
            lit_a: last_lit,
            lit_b: None,
        }
    }

    pub fn lits(&self) -> &[Literal] {
        &self.lits
    }

    #[inline]
    pub fn first_watched_lit(&self) -> Literal {
        self.lit_a
    }

    pub fn second_watched_lit(&self) -> Option<Literal> {
        self.lit_b
    }

    pub fn is_unary(&self) -> bool {
        self.lit_b.is_none()
    }

    pub fn strengthen(&mut self, lit: Literal, assigned_lits: &LiteralSet) -> Option<Literal> {
        if self.lit_a == lit {
            self.lit_a = self.lit_b.expect("Cannot strengthen unary clauses");
        } else if self.lit_b.is_none() || self.lit_b.unwrap() != lit {
            panic!("strengthen must be called based on 2-watched literals");
        }

        self.lit_b = self.find_new_second(assigned_lits);

        self.lit_b
    }

    pub fn un_strengthen(&mut self, assigned_lits: &LiteralSet) -> Option<Literal> {
        self.lit_b = self.find_new_second(assigned_lits);

        self.lit_b
    }

    #[inline]
    fn find_new_second(&self, assigned_lits: &LiteralSet) -> Option<Literal> {
        self.lits.iter().cloned()
            .filter(|&lit| lit != self.lit_a)
            .find(|lit| !assigned_lits.contains(&lit.complementary()))
    }
}

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted_lit_list: Vec<_> = self.lits.iter()
            .map(|lit| format!("{}", lit)).collect();

        write!(f, "{{{}}}", formatted_lit_list.join(", "))
    }
}
