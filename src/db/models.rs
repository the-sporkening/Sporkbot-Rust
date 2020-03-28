use chrono::{DateTime, TimeZone, Utc};
use serenity::model::id::{UserId, RoleId};
use std::fmt::{Display, Formatter, Result as FmtResult};
use super::schema::*;

// QUERYABLES

#[derive(Queryable, Identifiable, AsChangeset, Debug)]
#[primary_key(id)]
pub struct Guild {
    pub id: i64,
    pub admin_roles: Vec<i64>,
    pub audit: bool,
    pub audit_channel: i64,
    pub autorole: bool,
    pub autoroles: Vec<i64>,
    pub ignored_channels: Vec<i64>,
    pub introduction: bool,
    pub introduction_channel: i64,
    pub introduction_message: String,
    pub introduction_type: String,
    pub mod_roles: Vec<i64>,
    pub modlog: bool,
    pub modlog_channel: i64,
    pub prefix: String,
    pub welcome: bool,
    pub welcome_channel: i64,
    pub welcome_message: String,
    pub welcome_type: String,
    pub commands: Vec<String>,
    pub logging: Vec<String>,
}

#[derive(Queryable, Identifiable, AsChangeset, Debug)]
#[primary_key(id, guild_id)]
pub struct User<Tz: TimeZone> {
    pub id: i64,
    pub guild_id: i64,
    pub user_id: i64,
    pub xp: i64,
    pub last_message: DateTime<Tz>,
}
