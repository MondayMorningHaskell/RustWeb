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
    users,
);
