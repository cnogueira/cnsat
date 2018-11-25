use model::Literal;

#[derive(Debug)]
pub struct Decision {
    literal: Literal,
    level: u32,
}

impl Decision {
    pub fn from(literal: Literal, level: u32) -> Self {
        Decision { literal, level }
    }

    pub fn lit(&self) -> Literal {
        self.literal
    }

    pub fn lvl(&self) -> u32 {
        self.level
    }
}
