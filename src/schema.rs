// @generated automatically by Diesel CLI.

diesel::table! {
    contacts (id) {
        id -> Int4,
        #[max_length = 255]
        subject -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        content -> Text,
        created_at -> Timestamp,
        #[max_length = 45]
        ip_address -> Nullable<Varchar>,
    }
}

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
        order -> Int4,
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
        category_id -> Int4,
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
        order -> Int4,
    }
}

diesel::table! {
    projects_techs (project_id, tech_id) {
        project_id -> Int4,
        tech_id -> Int4,
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
    settings (id) {
        id -> Int4,
        #[max_length = 255]
        param -> Varchar,
        #[max_length = 255]
        value -> Varchar,
        note -> Nullable<Text>,
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
        role_id -> Int4,
    }
}

diesel::joinable!(posts -> post_categories (category_id));
diesel::joinable!(posts -> users (author_id));
diesel::joinable!(projects_techs -> projects (project_id));
diesel::joinable!(projects_techs -> techs (tech_id));
diesel::joinable!(users -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    contacts,
    hobbies,
    post_categories,
    posts,
    projects,
    projects_techs,
    roles,
    settings,
    techs,
    users,
);
