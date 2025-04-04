// @generated automatically by Diesel CLI.

diesel::table! {
    budget (id) {
        id -> Integer,
        amount -> Integer,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        name -> Text,
        password -> Text,
        session -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    budget,
    users,
);
