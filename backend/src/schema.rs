// @generated automatically by Diesel CLI.

diesel::table! {
    cookies (id) {
        id -> Text,
        date -> Date,
        cookie_data -> Jsonb,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        username -> Text,
        password_hash -> Text,
        jwt_intra_epitech -> Nullable<Text>,
        jwt_expires_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    cookies,
    users,
);
