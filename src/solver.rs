use model::Clause;
use model::LiteralSet;
use model::ClauseVec;
use model::Decision;
use solver::Constant::Sat;
use solver::Constant::Unsat;
use solver::Constant::Conflict;
use solver::Constant::NoConflict;
use model::Literal;
use decider::VSIDSDecider;
use model::ClauseId;
use fnv::FnvHashMap;
use fnv::FnvHashSet;
use conflict_analyzer::learn_from_conflict;
use std::prelude::v1::Vec;
use model::clause_vec_to_string;

#[derive(Debug, PartialEq)]
pub enum Constant {
    Sat,
    Unsat,
    Conflict,
    NoConflict,
}

pub struct Solver {
    clauses: ClauseVec,
    learnt_clauses: FnvHashSet<ClauseId>,
    lit_to_clause: FnvHashMap<Literal, Vec<ClauseId>>,
    watched_lit_to_clause: FnvHashMap<Literal, FnvHashSet<ClauseId>>,
    satisfied_clauses: FnvHashSet<ClauseId>,
    assigned_lits: LiteralSet,
    decision_stack: Vec<Decision>,
    decider: VSIDSDecider,
}

impl Solver {
    pub fn new() -> Self {
        Solver {
            clauses: ClauseVec::new(),
            learnt_clauses: FnvHashSet::default(),
            lit_to_clause: FnvHashMap::default(),
            watched_lit_to_clause: FnvHashMap::default(),
            satisfied_clauses: FnvHashSet::default(),
            assigned_lits: LiteralSet::default(),
            decision_stack: Vec::new(),
            decider: VSIDSDecider::new()
        }
    }

    pub fn add_clause(&mut self, clause: Clause) -> usize {
        let clause_id = self.clauses.len();

        clause.lits().iter().cloned().for_each(|lit| {
            self.lit_to_clause.entry(lit)
                .and_modify(|clauses| clauses.push(clause_id))
                .or_insert_with(|| vec![clause_id]);
        });

        self.add_watched_lit(clause_id, clause.first_watched_lit());
        if let Some(lit) = clause.second_watched_lit() {
            self.add_watched_lit(clause_id, lit);
        }

        self.decider.add_clause(&clause);
        self.clauses.push(clause);

        clause_id
    }

    #[inline]
    fn add_watched_lit(&mut self, clause_id: ClauseId, lit: Literal) {
        self.watched_lit_to_clause.entry(lit)
            .and_modify(|clauses| { clauses.insert(clause_id); })
            .or_insert_with(|| {
                let mut set = FnvHashSet::default();
                set.insert(clause_id);
                set
            });
    }

    pub fn solve(&mut self) -> Option<LiteralSet> {
        match self.dpll() {
            Sat => Some(self.assigned_lits.clone()),
            Unsat => None,
            other => panic!("DPLL must return either SAT or UNSAT. got: {:?}", other),
        }
    }

    fn dpll(&mut self) -> Constant {
        loop {
            self.print_status();

            let lit = self.decide_next_literal();

            if lit.is_none() {
                return Sat;
            }

            let lit = lit.unwrap();
            while self.deduce(lit) == Conflict {
                let (bt_lvl, lit) = self.analyze_conflict();

                if bt_lvl == 0 {
                    return Unsat;
                }

                self.backtrack_to(bt_lvl);
            }
        }
    }

    fn print_status(&self) {
        println!("***************** STATUS ********************");
        match self.decision_stack.last() {
            Some(decision) => decision.print_status(),
            None => println!("= No decisions"),
        }
        println!("- satisfied_clauses: {:?}", self.satisfied_clauses);
        println!("- assigned_lits: {:?}", self.assigned_lits);
        println!("- watched_lits: {:?}", self.watched_lit_to_clause);
        println!("- unsatisfied clauses:\n{}", clause_vec_to_string(&self.clauses, &self.satisfied_clauses));


        println!("*********************************************");
    }

    fn decide_next_literal(&mut self) -> Option<Literal> {
        let next_lit = self.decider.next_literal()?;
        let next_lvl = self.current_decision_level() + 1;

        self.decision_stack.push(Decision::from(next_lit, next_lvl));

        self.decider.assign_lit(&next_lit);
        self.assigned_lits.insert(next_lit);

        Some(next_lit)
    }

    fn deduce(&mut self, lit: Literal) -> Constant {
        let mut decision = self.decision_stack.pop().unwrap();
        let mut propagated_lits = LiteralSet::default();

        loop {
            // Satisfy clauses
            if let Some(clause_ids) = self.lit_to_clause.get(&lit).map(|c| c.as_slice()) {
                for &clause_id in clause_ids {
                    if !self.satisfied_clauses.contains(&clause_id) {
                        decision.add_satisfied_clause(clause_id);
                        self.satisfied_clauses.insert(clause_id);
                    }
                }
            }

            // Strengthen clauses
            let mut new_lits_to_watch = Vec::new();
            let complementary = lit.complementary();
            if let Some(clause_ids) = self.watched_lit_to_clause.get(&complementary) {
                for &clause_id in clause_ids.difference(&self.satisfied_clauses) {
                    let clause_ref: &mut Clause = &mut self.clauses[clause_id];

                    let next_watched_lit = clause_ref.strengthen(complementary, &self.assigned_lits);

                    if clause_ref.is_unary() {
                        if let Conflict = decision.add_propagated_lit(clause_ref.first_watched_lit(), clause_id) {
                            self.decision_stack.push(decision);
                            return Conflict;
                        }
                        propagated_lits.insert(clause_ref.first_watched_lit());
                    } else {
                        new_lits_to_watch.push((clause_id, next_watched_lit.unwrap()));
                    }
                }
            }

            new_lits_to_watch.into_iter().for_each(|(clause_id, lit)| {
                self.add_watched_lit(clause_id, lit);
            });

            self.watched_lit_to_clause.entry(complementary)
                .and_modify(|clause_ids| clause_ids.clear())
                .or_insert_with(|| FnvHashSet::default());

            self.decider.assign_lit(&lit);
            self.assigned_lits.insert(lit);

            if propagated_lits.is_empty() {
                break;
            }

            // No conflicts yet, prepare next lit to propagate
            let lit = propagated_lits.iter().next().cloned().unwrap();
            propagated_lits.remove(&lit);
        }

        self.decision_stack.push(decision);

        NoConflict
    }

    #[inline]
    fn current_decision_level(&self) -> u32 {
        match self.decision_stack.last() {
            Some(decision) => decision.lvl(),
            None => 0,
        }
    }

    fn analyze_conflict(&mut self) -> (u32, Literal) {

        let asserting_clause = learn_from_conflict(self.decision_stack.last().unwrap(), &self.clauses);
        println!("X conflict! learned: {}", asserting_clause);

        let lit = asserting_clause.first_watched_lit();
        let clause_id = self.add_clause(asserting_clause);
        self.learnt_clauses.insert(clause_id);


        (self.decision_stack.last().unwrap().lvl() - 1, lit)
    }

    fn backtrack_to(&mut self, level: u32) {
        loop {
            let last_decision = self.decision_stack.pop().unwrap();

            // undo lit assignments
            last_decision.propagated_lits_iter().for_each(|propagated_lit| {
                self.assigned_lits.remove(&propagated_lit);
                self.decider.un_assign_lit(propagated_lit);
            });

            self.assigned_lits.remove(&last_decision.lit());
            self.decider.un_assign_lit(last_decision.lit());

            // undo satisfied clauses
            last_decision.satisfied_clauses().iter().for_each(|clause_id| {
                self.satisfied_clauses.remove(clause_id);
            });

            // re-sync un strengthen clauses
            last_decision.implying_clauses_iter().for_each(|implying_clause_id| {
                self.clauses[implying_clause_id].un_strengthen(&self.assigned_lits);

                let new_watched_lit = self.clauses[implying_clause_id].second_watched_lit().unwrap();
                self.add_watched_lit(implying_clause_id, new_watched_lit);
            });

            if self.decision_stack.last().unwrap().lvl() == level {
                return;
            }
        }
    }
}
