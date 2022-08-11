table! {
    medias (id) {
        id -> Int8,
        fk_user_id -> Int8,
        fk_type_id -> Int4,
        directory -> Varchar,
        media -> Text,
        other_data -> Nullable<Jsonb>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        token -> Nullable<Text>,
    }
}

table! {
    tweets (id) {
        id -> Int4,
        message -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    types (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Int8,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        token -> Nullable<Text>,
    }
}

joinable!(medias -> types (fk_type_id));
joinable!(medias -> users (fk_user_id));

allow_tables_to_appear_in_same_query!(
    medias,
    tweets,
    types,
    users,
);
