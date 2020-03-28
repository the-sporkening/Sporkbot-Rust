use serenity::framework::standard::StandardFramework;
use serenity::model::id::UserId;
use std::collections::HashSet;

use crate::commands::{self, anilist, fun, meta, nekoslife, system, urban};
use crate::core::consts::{DB, BOT_ID, PREFIX, PREFIXES};
use crate::core::store::{Command, CommandLogger};
use crate::monitors;

pub struct SporkFramework;

impl SporkFramework {
    pub fn with_owners(owners: HashSet<UserId>) -> StandardFramework {
        StandardFramework::new()
            .configure(|c| {
                c.with_whitespace(true)
                    .allow_dm(true)
                    .on_mention(Some(UserId(BOT_ID)))
                    .ignore_bots(true)
                    .case_insensitivity(true)
                    .delimiters(vec![",", " "])
                    .owners(owners)
                    .prefix(PREFIX.as_str())
                    .prefixes(PREFIXES.to_vec())
            })
            .before(|ctx, msg, cmd| {
                if cmd != "shutdown" {
                    let _ = msg.channel_id.broadcast_typing(&ctx.http);
                }

                {
                    let mut data = ctx.data.write();
                    let cmd_logger = data.get_mut::<CommandLogger>().unwrap();
                    cmd_logger.insert(
                        msg.id,
                        Command {
                            name: cmd.to_string(),
                            message: msg.content.clone(),
                            user_id: msg.author.id,
                            time: msg.timestamp,
                        },
                    );
                }

                true
            })
            .normal_message(|ctx, msg| {
                monitors::message_monitors(ctx, msg);
            })
            .bucket("stats_limit", |b| b.delay(6 * 3600))
            .help(&commands::SPORKBOT_HELP)
            .group(&anilist::ANILIST_GROUP)
            .group(&urban::KNOWLEDGE_GROUP)
            .group(&fun::FUN_GROUP)
            .group(&meta::META_GROUP)
            .group(&nekoslife::NEKOSLIFE_GROUP)
            .group(&system::SYSTEM_GROUP)
        // .group(&commands::NO_CATEGORY_GROUP)
    }
}
