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
        timestamp -> Nullable<Int4>
    }
}
