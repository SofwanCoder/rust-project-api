// @generated automatically by Diesel CLI.

diesel::table! {
    auths (id) {
        id -> Uuid,
        user_id -> Uuid,
        expires_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(auths -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    auths,
    users,
);
