use model::Clause;
use model::Literal;
use fnv::FnvHashMap;
use std::collections::BTreeMap;
use model::LiteralSet;

/**
 * A next literal generator based on VSIDS.
 *
 * see https://www.princeton.edu/~chaff/publication/DAC2001v56.pdf
 */
pub struct VSIDSDecider {
    lit_count: FnvHashMap<Literal, u64>,
    count_lit: BTreeMap<u64, LiteralSet>,
    unassigned_lits: LiteralSet,
    age: u64,
}

impl VSIDSDecider {
    pub fn new() -> Self {
        VSIDSDecider {
            lit_count: FnvHashMap::default(),
            count_lit: BTreeMap::new(),
            unassigned_lits: LiteralSet::default(),
            age: 0,
        }
    }

    pub fn add_clause(&mut self, clause: &Clause) {
        let increment = 1 + self.age_factor();

        clause.lits().iter().cloned().for_each(|lit| {
            let previous_count = self.lit_count.get(&lit).cloned();

            // Increment lit_count
            let current_count = self.lit_count.entry(lit)
                .and_modify(|count| *count += increment)
                .or_insert(increment).clone();

            // Remove lit from previous count_lit
            let mut delete_previous_entry = false;
            if previous_count.is_some() {
                let previous_count = previous_count.unwrap();
                let previous_entry = self.count_lit.entry(previous_count);

                let modified_previous_set = previous_entry
                    .and_modify(|lits| { lits.remove(&lit); })
                    .or_default();

                if modified_previous_set.is_empty() {
                    delete_previous_entry = true;
                }
            }

            if delete_previous_entry {
                self.count_lit.remove(&previous_count.unwrap());
            }

            // Add lit to current count_lit
            self.count_lit.entry(current_count)
                .and_modify(|lits| { lits.insert(lit); })
                .or_insert_with(|| {
                    let mut lits = LiteralSet::default();
                    lits.insert(lit);
                    lits
                });
        });
    }

    pub fn next_literal(&mut self) -> Option<Literal> {
        if self.unassigned_lits.is_empty() {
            if self.age != 0 {
                return None;
            }

            // Initialize unassigned_lits
            self.unassigned_lits = self.lit_count.keys().cloned().collect();
        }

        self.age += 1;

        self.count_lit.values().rev()
            .filter(|&lits| self.contains_unassigned_literal(lits))
            .map(|lits| {
                lits.iter().cloned()
                    .filter(|lit| self.unassigned_lits.contains(lit))
                    .collect::<Vec<_>>()
            })
            // TODO choose candidate at random! Disabled to allow proper performance comparisons
            .find_map(|candidates| candidates.last().cloned())
    }

    #[inline]
    pub fn assign_lit(&mut self, lit: Literal) {
        self.unassigned_lits.remove(&lit);
        self.unassigned_lits.remove(&lit.complementary());
    }

    #[inline]
    pub fn un_assign_lit(&mut self, lit: Literal) {
        self.unassigned_lits.insert(lit);
        self.unassigned_lits.insert(lit.complementary());
    }

    fn age_factor(&self) -> u64 {
        self.age >> 3 // self.age / 8
    }

    fn contains_unassigned_literal(&self, lits: &LiteralSet) -> bool {
        !lits.is_disjoint(&self.unassigned_lits)
    }
}
