table! {
    b_domains (id) {
        id -> Int4,
        user_id -> Int4,
        hash -> Varchar,
        domain -> Varchar,
        flag -> Int2,
        state -> Int2,
        notes -> Varchar,
        create_time -> Int4,
    }
}

table! {
    domains (id) {
        id -> Int4,
        user_id -> Int4,
        hash -> Varchar,
        domain -> Varchar,
        flag -> Int2,
        state -> Int2,
        notes -> Varchar,
        create_time -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(b_domains, domains,);
