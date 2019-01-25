table! {
    pictures (id) {
        id -> Int8,
        name -> Text,
        image -> Text,
        created_at -> Timestamp,
        description -> Nullable<Text>,
    }
}
