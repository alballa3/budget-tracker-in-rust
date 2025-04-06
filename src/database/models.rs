use chrono::NaiveDateTime;
use diesel::{
    Selectable,
    prelude::{AsChangeset, Associations, Insertable, Queryable},
};

#[derive(Queryable, Selectable, Associations)]
#[diesel(table_name = crate::schema::budget)]
#[allow(dead_code)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Budget {
    pub id: i32,
    pub amount: i32,
    pub user_id: i32,
    pub created_at: Option<NaiveDateTime>,
}
#[derive(Insertable)]
#[diesel(table_name = crate::schema::budget)]
pub struct NewBudget {
    pub amount: i32,
    pub user_id: i32,
}
#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::budget)]

pub struct UpdatedBudget {
    pub amount: i32,
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

#[derive(Selectable, Queryable)]
#[diesel(table_name=crate::schema::transactions)]
#[allow(dead_code)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Transaction {
    pub id: i32,
    pub amount: i32,
    pub description: String,
    pub created_at: Option<NaiveDateTime>,
    pub budget_id: i32,
}
#[derive(Insertable)]
#[diesel(table_name=crate::schema::transactions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewTransaction {
    pub amount: i32,
    pub description: String,
    pub budget_id: i32,
}
#[derive(AsChangeset)]
#[diesel(table_name=crate::schema::transactions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UpdatedTransaction {
    pub amount: i32,
}