mod game;
mod command_handler;

use std::{sync::RwLock};
use game::game::{Shard};

use serenity::{prelude::*, async_trait, model::{application::{command::Command, interaction::Interaction}, prelude::{Ready, Message, interaction::InteractionResponseType}}, framework::{StandardFramework}, builder::CreateEmbed};
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
                "*ch farm" | "*ch farm info" => {
                    println!("[🌽] User {:?} requested farm info", message.author.name);
                    command_handler::command_handler::farm_info(&self.shard, &message, context).await;
                }
                _ => {}
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) =  interaction {
            let content: Option<CreateEmbed> = match command.data.name.as_str() {
                "farm" => Some(command_handler::command_handler::execute_farm(&self.shard, &command)),
                _ => None,
            };
            if let Some(em) = content {
                command.create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.add_embed(em))
                }).await.unwrap();
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        Command::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| command_handler::command_handler::register_farm(command))
        }).await.unwrap();
    }
}

#[tokio::main]
async fn main() {

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
}