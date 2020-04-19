table! {
    notes (id) {
        id -> Integer,
        creator_id -> Integer,
        parent_id -> Nullable<Integer>,
        content -> Text,
        created_time -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
        bio -> Text,
        created_time -> Timestamp,
    }
}

table! {
    sessions (id) {
        id -> Integer,
        cookie -> Varchar,
        user_id -> Integer,
        created_time -> Timestamp,
    }
}

joinable!(sessions -> users (user_id));

allow_tables_to_appear_in_same_query!(sessions, users);