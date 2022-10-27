/// The api module provides routes and handlers for all the non-WebAuthn functions, such as
/// User and credential CRUD routines.
///
pub use handlers::*;
pub use routes::*;

pub mod handlers;
pub mod routes;
