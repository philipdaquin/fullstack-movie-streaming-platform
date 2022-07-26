table! {
    products (id) {
        id -> Int4,
        name -> Varchar,
        price -> Nullable<Int4>,
        weight -> Nullable<Int4>,
        category -> Nullable<Varchar>,
        created_by -> Nullable<Int4>,
        tags -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        description -> Nullable<Varchar>,
        image_url -> Nullable<Varchar>,
    }
}
