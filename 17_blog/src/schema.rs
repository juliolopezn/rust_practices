// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Uuid,
        title -> Varchar,
        slug -> Varchar,
        body -> Varchar,
        created_at -> Timestamp,
    }
}
