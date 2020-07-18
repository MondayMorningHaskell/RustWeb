table! {
    articles (id) {
        id -> Int4,
        title -> Text,
        body -> Text,
        published_at -> Timestamptz,
        author_id -> Int4,
    }
}

table! {
    auth_infos (id) {
        id -> Int4,
        user_id -> Int4,
        password_hash -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
        age -> Int4,
    }
}

joinable!(articles -> users (author_id));

allow_tables_to_appear_in_same_query!(
    articles,
    auth_infos,
    users,
);
