table! {
    sessions (id) {
        id -> Int4,
        uuid -> Varchar,
        owner_type -> Varchar,
        owner_uuid -> Varchar,
        refresh_token -> Varchar,
        refresh_token_expires_at -> Timestamptz,
        access_token -> Varchar,
        access_token_expires_at -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    teachers (id) {
        id -> Int4,
        uuid -> Varchar,
        email -> Varchar,
        password_digest -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(sessions, teachers,);
