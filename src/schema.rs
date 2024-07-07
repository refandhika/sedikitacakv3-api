// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        subtitle -> Nullable<Varchar>,
        #[max_length = 255]
        slug -> Varchar,
        content -> Text,
        #[max_length = 100]
        category -> Nullable<Varchar>,
        tags -> Nullable<Text>,
        author_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
        published -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 20]
        phone -> Nullable<Varchar>,
        birth -> Nullable<Date>,
        #[max_length = 255]
        github -> Nullable<Varchar>,
        #[max_length = 255]
        linkedin -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(posts -> users (author_id));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
