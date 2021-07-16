table! {
    api_keys (id) {
        id -> Integer,
        api_key -> Text,
        user_id -> Integer,
        expires -> Timestamp,
    }
}
