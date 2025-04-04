use chrono::NaiveDateTime;
use diesel::{
    Selectable,
    prelude::{Insertable, Queryable},
};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::budget)]
#[allow(dead_code)]

pub struct Budget {
    pub id: i32,
    pub amount: i32,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
}
#[derive(Insertable)]
#[diesel(table_name = crate::schema::budget)]
pub struct NewBudget {
    pub amount: i32,
    pub user_id: i32,
    
}

#[derive(Queryable, Selectable)]
#[diesel(table_name=crate::schema::users)]
#[allow(dead_code)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub password: String,
    pub session: String,
    pub created_at: Option<NaiveDateTime>,
}
#[derive(Insertable)]
#[diesel(table_name=crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]

pub struct NewUser<'a> {
    pub name: &'a String,
    pub password: &'a String,
    pub session: &'a String,
}
