#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint; // entrypoint to the program
pub mod error;
pub mod instruction; // program API, (de)serializing instruction data
pub mod processor; // program logic
pub mod state; // program objects, (de)serializing state // program specific errors
