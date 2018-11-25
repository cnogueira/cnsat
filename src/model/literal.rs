use std::fmt;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Literal(i32);

impl Literal {
    pub fn from_dimacs_lit(lit: dimacs::Lit) -> Self {
        match lit.sign() {
            dimacs::Sign::Pos => Literal(lit.var().to_u64() as i32),
            dimacs::Sign::Neg => Literal(-(lit.var().to_u64() as i32)),
        }
    }

    pub fn complementary(&self) -> Literal {
        Literal(-1 * self.0)
    }
}

impl fmt::Debug for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
