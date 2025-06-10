// @generated automatically by Diesel CLI.

diesel::table! {
    equations (id) {
        id -> Int4,
        #[max_length = 16]
        content -> Varchar,
        answer -> Float4,
    }
}
