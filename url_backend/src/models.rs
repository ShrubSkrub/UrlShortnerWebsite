use diesel::prelude::*;

#[derive(Queryable)]
pub struct urls {
    pub id: i32,
    pub long_url: String,
    pub short_url: String,
    pub clicks: i32,
    pub creation_date: String,
    pub expiration_date: String
}
