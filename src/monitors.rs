use serenity::model::{
    channel::{GuildChannel, Message},
    guild::{Guild, Member},
    id::ChannelId,
    id::GuildId,
    voice::VoiceState,
};

use serenity::prelude::Context;

use crate::core::consts::PREFIX;

mod anilist;
mod message_id;

pub fn message_monitors(context: &Context, message: &Message) {
    // TODO Check if user exists in DB
    // TODO ADD user if no record found
    // TODO Add user xp on message
    // TODO Notify on level up
    if !message.author.bot
        && !message
            .content_safe(&context.cache)
            .as_str()
            .starts_with(PREFIX.as_str())
    {
        anilist::anilist_links_monitor(context, message);
        //anilist::rem_monitor(&context, &message);
        message_id::message_id_monitor(context, message);
    }
}

pub fn new_member_monitors(_context: &Context, _guild_id: GuildId, _new_member: &Member) {
    // Greet the user?
}

pub fn voice_state_update(
    ctx: Context,
    guild_id: Option<GuildId>,
    _old_state: Option<VoiceState>,
    new_state: VoiceState,
) {
    // TODO Check if user connected to room creator channel
    // TODO Create new voice channel in category
    // TODO Store channel in db for persistance
    // TODO Set room owner to user who joined channel
    // TODO Move user to voice channel
    // TODO Check if channel is empty
    // TODO If channel is empty remove it
    // ! Make sure its not the creation channel
    // TODO Remove Channel from DB

    // info!("Old: {:?}", old);
    // info!("New: {:?}", new_state);
    let user_id = new_state.user_id;

    let guild = match guild_id.unwrap().to_guild_cached(&ctx) {
        Some(guild) => guild.read().clone(),
        None => {
            info!("Guild not found in cache.");
            return;
        }
    };

    let channel_id = match new_state.channel_id {
        Some(channel_id) => info!("Joined: {}", channel_id),
        None => {
            info!("Disconnected");
            let _data = &ctx.data.read();
            // TODO Check if user is last one to leave a channel
            return;
        }
    };

    let _guild_id = match guild_id {
        Some(guild_id) => guild_id,
        None => {
            info!("Guild id not found.");
            return;
        }
    };
}

fn _voice_channel_is_empty(ctx: &Context, guild: Guild, channel_id: ChannelId) -> bool {
    let mut is_empty = true;
    for state in guild
        .voice_states
        .values()
        .filter(|state| state.channel_id == Some(channel_id))
    {
        let user = match state.user_id.to_user(ctx) {
            Ok(user) => user,
            Err(err) => {
                error!("Error retrieving user: {:?}", err);
                return is_empty;
            }
        };
        is_empty &= user.bot;
    }
    return is_empty;
}
