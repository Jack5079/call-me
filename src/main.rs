use std::env;

use serenity::all::CreateMessage;
use serenity::all::UserId;
use serenity::async_trait;
use serenity::model::voice::VoiceState;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn voice_state_update(
        &self,
        ctx: Context,
        old: Option<VoiceState>,
        new: VoiceState,
    ) -> () {
        let owner = UserId::new(
            env::var("OWNER")
                .expect("Expected a user id in the environment")
                .parse()
                .expect("Expected a valid user id"),
        );
        if let Some(channel) = new.channel_id {
            if old.is_none() && new.user_id != owner {
                let mention = new.user_id.mention();
                let owner = owner.mention();
                let builder =
                    CreateMessage::new().content(format!("{owner}, {mention} is calling you"));
                if let Err(why) = channel.send_message(ctx, builder).await {
                    println!("Error sending message: {why:?}");
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_VOICE_STATES;

    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
