diesel::table! {
    users (id) {
        id -> Int4,
        firstname -> Nullable<VarChar>,
        lastname -> Nullable<VarChar>,
        username -> Varchar,
        email -> Varchar,
        hash -> Nullable<VarChar>,
        phone_number -> Nullable<Int4>,
        dob -> Nullable<VarChar>,
    }
}
