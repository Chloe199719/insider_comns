// @generated automatically by Diesel CLI.

diesel::table! {
    user_sessions (id) {
        id -> Text,
        user_id -> Text,
        session_token -> Text,
        refresh_token -> Text,
        expires_at -> Timestamptz,
        ip_address -> Text,
        user_agent -> Text,
        device_info -> Text,
        location -> Text,
        country -> Text,
        city -> Text,
        region -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        revoked_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        email -> Text,
        verified_at -> Nullable<Timestamptz>,
        password_hash -> Text,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(user_sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(user_sessions, users,);
