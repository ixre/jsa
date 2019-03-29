table! {
    d_domain (id) {
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
    u_user (id) {
        id -> Int4,
        user -> Varchar,
        name -> Varchar,
        pwd -> Varchar,
        flag -> Int2,
        email -> Varchar,
        create_time -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(d_domain, u_user,);
