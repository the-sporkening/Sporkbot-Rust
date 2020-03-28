use serenity::framework::standard::macros::group;

mod avatar;
mod botinfo;
mod ping;
mod stats;

use self::avatar::AVATAR_COMMAND;
use self::botinfo::BOT_INFO_COMMAND;
use self::ping::PING_COMMAND;
use self::stats::STATS_COMMAND;

// group!({
//     name: "",
//     commands: []
// });
#[group("Meta")]
#[commands(avatar, bot_info, ping, stats)]
struct Meta;
