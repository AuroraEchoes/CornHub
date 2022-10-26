mod game;
mod command_handler;

use std::sync::RwLock;

use game::game::{Shard};

use serenity::{prelude::*, async_trait, model::prelude::{Message, Ready}, framework::StandardFramework};
use dotenv::dotenv;
struct CornhubBot {
    shard: Shard
}

#[async_trait]
impl EventHandler for CornhubBot {

    // Handling commands sloppily for the moment while I build out the internal logic [the fun stuff]
    async fn message(&self, context: Context, message: Message) {
        if message.content.contains("*ch") {
            // We know that this is a CornHub command
            println!("[🌽] Command recieved: {:?}", message.content);
            match message.content.as_str() {
                "*ch farm new" => {
                    println!("[🌽] User {:?} requested a new farm", message.author.name);
                    command_handler::command_handler::new_farm(&self.shard, &message, context).await;
                },
                "*ch farm" => {
                    println!("[🌽] User {:?} requested farm info", message.author.name);
                    command_handler::command_handler::farm_info(&self.shard, &message, context).await;
                }
                _ => {}
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error>{

    dotenv().ok();

    // Discord bot
    let token = std::env::var("CORNHUB_TOKEN").expect("Expected bot token");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("*"));

    let cornhub_bot = CornhubBot { shard: Shard { cached_farms: RwLock::new(Vec::new()) } };
    
    let mut client = Client::builder(&token, intents).event_handler(cornhub_bot).framework(framework).await.expect("Error creating client");

    if let Err(error) = client.start().await {
        println!("Client error: {:?}", error);
    }

    Ok(())
}