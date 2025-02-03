pub mod bcrypt;

pub use bcrypt::Bcrypt;

pub trait Hasher {
    /// Hash the input string and return the hashed value.
    fn hash(&self, input: &str) -> String;

    /// Verify whether the given input matches the hashed value.
    fn verify(&self, input: &str, hash: &str) -> bool;
}
