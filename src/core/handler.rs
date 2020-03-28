use crate::core::consts::PREFIX;
use serenity::model::{
    channel::Reaction,
    event::{ResumedEvent, VoiceEvent},
    gateway::{Activity, Ready},
    guild::Member,
    id::{GuildId, UserId},
    voice::VoiceState,
};
use std::{thread, time};
use std::sync::{Once, ONCE_INIT};
use serenity::prelude::{Context, EventHandler};
use std::collections::HashSet;

use crate::{menu, monitors};

#[derive(Default)]
pub struct SporkHandler {
    _blacklist: HashSet<String>,
}

impl EventHandler for SporkHandler {
    fn guild_member_addition(&self, context: Context, guild_id: GuildId, new_member: Member) {
        monitors::new_member_monitors(&context, guild_id, &new_member);
    }

    fn reaction_add(&self, context: Context, add_reaction: Reaction) {
        menu::handle_reaction(&context, &add_reaction);
    }

    fn voice_state_update(
        &self, ctx: Context, guild_id: Option<GuildId>, old: Option<VoiceState>, new: VoiceState,
    ) {
        monitors::voice_state_update(ctx, guild_id, old, new);
    }

    fn ready(&self, ctx: Context, ready: Ready) {
        // TODO Create an array of status messages to cycle through
        // TODO Cycle through them every 5 minutes

        static INIT: Once = ONCE_INIT;
        let guilds_count = ready.guilds.len();
        
        INIT.call_once(|| {
            debug!("Spawning stats thread");
            thread::spawn(move || loop {
                // sleep before checking, allows some time for guild lazy loading
                let five_min = time::Duration::from_secs(300);
                thread::sleep(five_min);
    
                // Update stats for bot, probably should rename this file or something
                {
                    // let cache = CACHE.read();
                    let activity = format!("{} Guilds", guilds_count);
                    // pool.update_stat("bot", "guilds_count", None, Some(guilds_count as i64));
                    // pool.update_stat("bot", "channels_count", None, Some(channels_count as i64));
                    // pool.update_stat("bot", "users_count", None, Some(users_count as i64));
                    ctx.set_activity(Activity::listening(&activity));
                    debug!("updated cached bot info: guilds: {}",
                        guilds_count);
                }
            });
        });

        info!("Connected as {}", ready.user.name);
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }

}
