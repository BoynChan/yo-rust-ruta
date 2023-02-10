#[macro_use]
extern crate lazy_static;

mod entity;
pub use entity::name_table;

mod db;
pub use db::DB;
