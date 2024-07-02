table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        email -> Text,
        password -> Text,
        created_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>
    }
}
