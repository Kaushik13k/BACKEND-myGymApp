diesel::table! {
    users (id) {
        id -> Int4,
        firstname -> Varchar,
        lastname -> Varchar,
        username -> Varchar,
        email -> Varchar,
        hash -> Varchar,
        phone_number -> Int4,
        age -> Int4,
    }
}
