use crate::core::{framework::SporkFramework, handler::SporkHandler, store::*};
use serenity::{prelude::*, Client};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub struct SporkClient(Client);

impl Default for SporkClient {
    fn default() -> Self {
        SporkClient::new()
    }
}

impl SporkClient {
    pub fn new() -> Self {
        // Load token from environment variables or .env file
        let token: String = dotenv::var("DISCORD_TOKEN").expect("token");
        let mut client =
            Client::new(&token, SporkHandler::default()).expect("Error creating client");

        let owner = match client.cache_and_http.http.get_current_application_info() {
            Ok(info) => info.owner,
            Err(why) => panic!("Couldn't get application info: {:?}", why),
        };

        let mut owner_ids = HashSet::new();
        owner_ids.insert(owner.id);

        {
            let mut data = client.data.write();
            data.insert::<CommandLogger>(HashMap::default());
            data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
            data.insert::<MessagePaginator>(HashMap::default());
            data.insert::<BotOwnerContainer>(owner);
        }

        client.with_framework(SporkFramework::with_owners(owner_ids));

        SporkClient(client)
    }

    pub fn start(&mut self) -> Result<(), SerenityError> {
        self.0.start()
    }
    pub fn start_autosharded(&mut self) -> Result<(), SerenityError> {
        self.0.start_autosharded()
    }
}
