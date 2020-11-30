table! {
    todos (id) {
        id -> Int4,
        todotext -> Varchar,
        done -> Bool,
        username -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    todos,
    users,
);
