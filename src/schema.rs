// @generated automatically by Diesel CLI.

diesel::table! {
    equations (id) {
        id -> Integer,
        content -> Text,
        answer -> Float,
    }
}
