// @generated automatically by Diesel CLI.

diesel::table! {
    equations (id) {
        id -> Integer,
        content -> VarChar,
        answer -> Float,
    }
}
