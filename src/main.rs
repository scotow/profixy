use std::env;

use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::prelude::Member;
use serenity::prelude::*;

struct Handler(String);

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        let _ = new_member
            .user
            .direct_message(&ctx, |m| m.content(self.0.clone()))
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
    let owner = env::var("OWNER").expect("Expected a discord tag");
    let intents = GatewayIntents::GUILD_MEMBERS;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler(owner))
        .await
        .expect("Err creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
