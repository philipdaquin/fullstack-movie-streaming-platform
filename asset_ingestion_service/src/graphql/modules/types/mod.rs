pub mod prod_company;
pub mod people_module;
pub mod movies;
pub mod tmdb_test;

pub use prod_company::schema::{ProductionCompanyQuery, ProductionCompanyMutation};
pub use movies::schema::{MovieMutation};
pub use people_module::schema::{PersonMutation, PersonQuery};