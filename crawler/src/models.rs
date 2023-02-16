use diesel::prelude::*;

#[derive(Queryable)]
pub struct NewsEntity {
    pub id: i32,
    pub title: String,
    pub source: String,
    pub rank: i32,
    pub publist_date: i64,
}
