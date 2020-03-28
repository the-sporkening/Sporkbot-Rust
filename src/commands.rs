use serenity::framework::standard::{
    help_commands,
    macros::{group, help},
    Args, CommandGroup, CommandResult, HelpOptions,
};

use serenity::model::prelude::*;
use serenity::prelude::Context;
use std::collections::HashSet;

pub mod anilist;
pub mod fun;
pub mod giphy;
pub mod meta;
pub mod nekoslife;
pub mod system;
pub mod urban;

pub mod test;
use self::test::TEST_COMMAND;

use self::giphy::GIPHY_COMMAND;

#[group("no_category")]
#[commands(giphy, test)]
struct Test;

#[help]
#[individual_command_tip = "Hello! こんにちは！Hola! Bonjour! 您好!\n\
If you want more information about a specific command, just pass the command as argument."]
#[command_not_found_text = "Could not find: `{}`."]
#[max_levenshtein_distance(3)]
#[indention_prefix = "+"]
#[lacking_permissions = "Hide"]
#[lacking_role = "Nothing"]
#[wrong_channel = "Strike"]
pub fn sporkbot_help(
    context: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, help_options, groups, owners)
}
