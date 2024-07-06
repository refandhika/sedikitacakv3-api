// @generated automatically by Diesel CLI.

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
