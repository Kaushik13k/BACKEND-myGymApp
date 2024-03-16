// @generated automatically by Diesel CLI.

diesel::table! {
    muscle_group (id) {
        id -> Int4,
        #[max_length = 255]
        muscle_group_name -> Nullable<Varchar>,
        #[max_length = 255]
        muscle_group_image -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        firstname -> Nullable<Varchar>,
        lastname -> Nullable<Varchar>,
        username -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        hash -> Nullable<Varchar>,
        phone_number -> Nullable<Int4>,
        age -> Nullable<Int4>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    muscle_group,
    users,
);
