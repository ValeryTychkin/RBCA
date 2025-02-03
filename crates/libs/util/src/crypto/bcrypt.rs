use super::Hasher;
use bcrypt::{hash, verify};

pub struct Bcrypt {
    cost: u32,
}

impl Bcrypt {
    pub fn new() -> Self {
        Self { cost: 10 }
    }
}

impl Hasher for Bcrypt {
    fn hash(&self, input: &str) -> String {
        hash(input, self.cost).unwrap()
    }

    fn verify(&self, input: &str, hash: &str) -> bool {
        verify(input, hash).unwrap()
    }
}
