// @generated automatically by Diesel CLI.

diesel::table! {
    budget (id) {
        id -> Integer,
        amount -> Integer,
        user_id -> Integer,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    transactions (id) {
        id -> Integer,
        amount -> Integer,
        description -> Text,
        created_at -> Nullable<Timestamp>,
        budget_id -> Integer,
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

diesel::joinable!(budget -> users (user_id));
diesel::joinable!(transactions -> budget (budget_id));

diesel::allow_tables_to_appear_in_same_query!(
    budget,
    transactions,
    users,
);
