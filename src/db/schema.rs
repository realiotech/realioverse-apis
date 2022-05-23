table! {
    ethbalances (id) {
        id -> Int4,
        account -> Varchar,
        balance -> Numeric,
        holder -> Bool,
        last_updated -> Timestamp,
    }
}



