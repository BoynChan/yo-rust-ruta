// @generated automatically by Diesel CLI.

diesel::table! {
    hackernews (id) {
        id -> Int4,
        title -> Varchar,
        source -> Text,
        rank -> Int4,
        publish_date -> Timestamp,
    }
}

diesel::table! {
    names (id) {
        id -> Int8,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    hackernews,
    names,
);
