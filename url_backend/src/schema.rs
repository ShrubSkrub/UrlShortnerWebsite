// @generated automatically by Diesel CLI.

diesel::table! {
    urls (id) {
        id -> Int4,
        long_url -> Text,
        short_url -> Text,
        clicks -> Nullable<Int4>,
        creation_date -> Timestamp,
        expiration_date -> Nullable<Timestamp>,
    }
}
