// @generated automatically by Diesel CLI.

diesel::table! {
    service_contexts (id) {
        id -> Int4,
        maintenance -> Bool,
    }
}

diesel::table! {
    todos (id) {
        id -> Int4,
        title -> Varchar,
        description -> Text,
        completed -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        full_name -> Varchar,
        profile_image -> Nullable<Varchar>,
        session_token_id -> Nullable<Uuid>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    service_contexts,
    todos,
    users,
);
