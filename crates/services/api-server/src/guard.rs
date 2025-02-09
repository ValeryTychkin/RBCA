pub mod staff;
pub mod user;

#[derive(Debug)]
pub enum GuardError {
    MissingAuthToken,
    WrongToken,
    MissingToken,
    MissingUser,
    MissingPermission,
}
