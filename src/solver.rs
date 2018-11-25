use model::Clause;
use model::LiteralSet;
use model::ClauseVec;
use model::Decision;
use solver::Constant::SAT;
use solver::Constant::UNSAT;
use solver::Constant::CONFLICT;
use model::Literal;
use decider::VSIDSDecider;

#[derive(Debug, PartialEq)]
enum Constant {
    SAT,
    UNSAT,
    CONFLICT,
}

pub struct Solver {
    clauses: ClauseVec,
    decision_stack: Vec<Decision>,
    decider: VSIDSDecider,
}

impl Solver {
    pub fn new() -> Self {
        Solver {
            clauses: Vec::new(),
            decision_stack: Vec::new(),
            decider: VSIDSDecider::new()
        }
    }

    pub fn add_clause(&mut self, clause: Clause) {
        self.decider.add_clause(&clause);
        self.clauses.push(clause);
    }

    pub fn solve(&mut self) -> Option<LiteralSet> {
        match self.dpll() {
            SAT => Some(self.decision_stack.iter().map(|d| d.lit()).collect()),
            UNSAT => None,
            other => panic!("DPLL must return either SAT or UNSAT. got: {:?}", other),
        }
    }

    fn dpll(&mut self) -> Constant {
        loop {
            let lit = self.decide_next_literal();

            if lit.is_none() {
                return SAT;
            }

            if self.deduce(lit.unwrap()) == CONFLICT {
                let bt_lvl = self.analyze_conflicts();

                if bt_lvl == 0 {
                    return UNSAT;
                }

                self.backtrack_to(bt_lvl);
            }
        }
    }

    fn decide_next_literal(&mut self) -> Option<Literal> {
        self.decider.next_literal()
    }

    fn deduce(&mut self, literal: Literal) -> Constant {
        unimplemented!()
    }

    fn analyze_conflicts(&mut self) -> u32 {
        unimplemented!()
    }

    fn backtrack_to(&mut self, level: u32) {
        unimplemented!()
    }
}
