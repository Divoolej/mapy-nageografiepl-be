table! {
    teachers (id) {
        id -> Int4,
        email -> Varchar,
        password_digest -> Varchar,
        auth_token -> Nullable<Varchar>,
    }
}
