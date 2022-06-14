use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::prelude::*;
use shards::cstr;
use shards::shard::Shard;
use shards::shstr;
use shards::types::RawString;

#[derive(Default)]
pub struct BotShard {}

impl Shard for BotShard {
    fn registerName() -> &'static str {
        cstr!("Discord.Bot")
    }

    fn hash() -> u32 {
        compile_time_crc32::crc32!("Discord.Bot-rust-0x20200101")
    }

    fn name(&mut self) -> &str {
        "Discord.Bot"
    }

    fn inputTypes(&mut self) -> &shards::types::Types {
        todo!()
    }

    fn outputTypes(&mut self) -> &shards::types::Types {
        todo!()
    }

    fn activate(
        &mut self,
        context: &shards::types::Context,
        input: &shards::types::Var,
    ) -> Result<shards::types::Var, &str> {
        todo!()
    }
}
