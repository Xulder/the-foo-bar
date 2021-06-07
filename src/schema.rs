table! {
    posts (id) {
        id -> Int4,
        slug -> Text,
        title -> Text,
        body -> Text,
        author -> Int4,
        published -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Text,
        hash -> Text,
    }
}

joinable!(posts -> users (author));

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
