table! {
    guilds (id) {
        id -> Int8,
        admin_roles -> Array<Int8>,
        audit -> Bool,
        audit_channel -> Int8,
        audit_threshold -> Int2,
        autorole -> Bool,
        autoroles -> Array<Int8>,
        ignored_channels -> Array<Int8>,
        introduction -> Bool,
        introduction_channel -> Int8,
        introduction_message -> Text,
        introduction_type -> Text,
        mod_roles -> Array<Int8>,
        modlog -> Bool,
        modlog_channel -> Int8,
        mute_setup -> Bool,
        prefix -> Text,
        welcome -> Bool,
        welcome_channel -> Int8,
        welcome_message -> Text,
        welcome_type -> Text,
        commands -> Array<Text>,
        logging -> Array<Text>,
    }
}

table! {
    users (id, guild_id) {
        id -> Int8,
        guild_id -> Int8,
        user_id -> Int8,
        xp -> Int8,
        last_message -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(
    guilds,
    users,
);
