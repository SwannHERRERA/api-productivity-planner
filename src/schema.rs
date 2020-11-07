table! {
    objective (uuid) {
        uuid -> Uuid,
        name -> Varchar,
        start_at -> Nullable<Timestamp>,
        date_created -> Timestamp,
    }
}

table! {
    users (uuid) {
        uuid -> Uuid,
        email -> Varchar,
        password -> Nullable<Bpchar>,
    }
}

allow_tables_to_appear_in_same_query!(
    objective,
    users,
);
