// @generated automatically by Diesel CLI.

diesel::table! {
    body_measurements (id) {
        id -> Int4,
        user_id -> Int4,
        weight -> Nullable<Float8>,
        height -> Nullable<Float8>,
        weist -> Nullable<Float8>,
        neck -> Nullable<Float8>,
        shoulders -> Nullable<Float8>,
        chest -> Nullable<Float8>,
        left_bicep -> Nullable<Float8>,
        right_bicep -> Nullable<Float8>,
        left_forearm -> Nullable<Float8>,
        right_forearm -> Nullable<Float8>,
        abdomen -> Nullable<Float8>,
        hips -> Nullable<Float8>,
        left_thigh -> Nullable<Float8>,
        right_thigh -> Nullable<Float8>,
        left_calf -> Nullable<Float8>,
        right_calf -> Nullable<Float8>,
        timestamp -> Nullable<Int4>,
    }
}

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
        dob -> Nullable<Int4>,
    }
}

diesel::joinable!(body_measurements -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    body_measurements,
    muscle_group,
    users,
);
