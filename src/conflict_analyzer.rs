use model::Decision;
use model::ClauseVec;
use std::collections::VecDeque;
use model::Clause;
use model::LiteralVec;
use model::LiteralSet;

pub fn learn_from_conflict(decision: &Decision, clause_db: &ClauseVec) -> Clause {
    let mut terminal_lits = LiteralSet::default();
    let mut to_explore = VecDeque::with_capacity(decision.propagated_lits_len());

    let conflict_lit = decision.get_conflict_lit().expect("Decision must contain a conflict!");
    to_explore.push_back(conflict_lit.complementary());
    to_explore.push_back(conflict_lit);

    while to_explore.len() > 1 {
        let lit = to_explore.pop_front().unwrap();

        match decision.implying_clause_of(lit) {
            Some(implying_clause_id) => {
                clause_db.get(implying_clause_id).unwrap().lits().iter()
                    .filter(|clause_lit| **clause_lit != lit)
                    .map(|clause_lit| clause_lit.complementary())
                    .for_each(|implying_lit| {
                        if !to_explore.contains(&implying_lit) {
                            to_explore.push_back(implying_lit);
                        }
                    });
            },
            None => { terminal_lits.insert(lit.complementary()); },
        }
    }

    let decision_lit = decision.lit().complementary();
    let last_lit = if terminal_lits.contains(&decision_lit) {
        decision_lit
    } else {
        let last_lit = to_explore.pop_front().unwrap().complementary();
        terminal_lits.insert(last_lit);
        last_lit
    };

    // Learnt clause
    Clause::new_asserting_clause(last_lit, terminal_lits.iter().cloned().collect())
}


