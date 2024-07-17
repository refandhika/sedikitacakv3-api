// @generated automatically by Diesel CLI.

diesel::table! {
    hobbies (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        content -> Text,
        #[max_length = 255]
        image -> Nullable<Varchar>,
        item_order -> Int4,
        active -> Bool,
        published -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    post_categories (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        slug -> Varchar,
        description -> Nullable<Text>,
        published -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

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
        tags -> Nullable<Text>,
        author_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
        published -> Bool,
        category_id -> Nullable<Int4>,
    }
}

diesel::table! {
    projects (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        content -> Text,
        #[max_length = 255]
        source -> Nullable<Varchar>,
        #[max_length = 255]
        url -> Nullable<Varchar>,
        #[max_length = 255]
        demo -> Nullable<Varchar>,
        relevant -> Bool,
        published -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        level -> Varchar,
        can_modify_user -> Bool,
        can_edit -> Bool,
        can_view -> Bool,
        is_guest -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    techs (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        icon -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
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

diesel::joinable!(posts -> post_categories (category_id));
diesel::joinable!(posts -> users (author_id));

diesel::allow_tables_to_appear_in_same_query!(
    hobbies,
    post_categories,
    posts,
    projects,
    roles,
    techs,
    users,
);
