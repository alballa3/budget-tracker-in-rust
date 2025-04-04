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
    pub created_at: NaiveDateTime,
}
#[derive(Insertable)]
#[diesel(table_name = crate::schema::budget)]
pub struct NewBudget {
    pub amount: i32,
}
