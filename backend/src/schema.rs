// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Text,
        username -> Text,
        password_hash -> Text,
        jwt_intra_epitech -> Nullable<Text>,
        jwt_expires_at -> Nullable<Timestamp>,
    }
}
