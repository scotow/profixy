use std::env;

use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::prelude::{GuildId, Member};
use serenity::prelude::*;

struct Handler {
    guild: GuildId,
    owner: String,
}

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        if new_member.guild_id != self.guild {
            return
        }

        let _ = new_member
            .user
            .direct_message(&ctx, |m| m.content(self.owner.clone()))
            .await;
        let _ = new_member.kick(&ctx).await;
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let guild = env::var("GUILD").map(|g| g.parse::<u64>().expect("Invalid guild id")).expect("Expected a guild id in the environment").into();
    let owner = env::var("OWNER").expect("Expected a discord tag in the environment");

    let intents = GatewayIntents::GUILD_MEMBERS;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler {
            guild, owner
        })
        .await
        .expect("Err creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
