pub mod command_handler {
    use serenity::{model::prelude::{Message, interaction::application_command::ApplicationCommandInteraction, Embed}, prelude::Context, builder::{CreateApplicationCommand, CreateEmbed, EditMessage}, utils::{Colour, MessageBuilder}};

    use crate::game::game::Shard;

    const COLOUR: Colour = Colour::from_rgb(255, 190, 0);

    // Register`
    pub fn register_farm(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        return command.name("farm").description("CornHub Farm Command");
    }

    pub fn execute_farm(shard: &Shard, interaction: &ApplicationCommandInteraction) -> CreateEmbed {
        if let Some(farm) = shard.info(interaction.user.id.as_u64()) {

            return CreateEmbed::default()
                .colour(COLOUR)
                .title(format!("🌽 **{} info** 🌽", &farm.name))
                .field("Time since last interaction", format!("⏱️ {:?}s", farm.last_interaction.elapsed().as_secs()), false)
                .field("Corn Type", format!("{} {}", "💊", "Medical Corn"), false) // TODO: Make me dynamic
                .field("CornCash", format!("💸 {:?}", 5012), false) // TODO: Make me dynamic
                .field("NSFW Mode", format!("{} {}", "🍆", "True"), false) // TODO: Make me dynamic
                .to_owned();
        }
        else {
            return CreateEmbed::default()
                .colour(COLOUR)
                .title(format!("🌽 **Farm Info** 🌽"))
                .description("You don't have a farm! Create one using */farm new*.")
                .to_owned();
        }
    }

    pub async fn new_farm(shard: &Shard, message: &Message, context: Context) {

        // User has a farm
        if let Some(farm) = shard.info(message.author.id.as_u64()) {
            println!("{} tried to create a new farm, but they already have one", message.author.name);
            message.reply(context.http, String::from("You already have a farm!")).await.unwrap();
        }

        // Create the user a farm
        else {
            message.reply(context.http, String::from("Created you a new farm. Use *ch options to edit it's properties")).await.unwrap();
            shard.create_farm(&message.author);
        }
    }

    pub async fn farm_info(shard: &Shard, message: &Message, context: Context) {
        if let Some(farm) = shard.info(message.author.id.as_u64()) {
            message.reply(context.http, format!("🌽 **FARM INFO** 🌽 \nOwner: {:?}\nName: {:?} \nElapsed time: {:?}", farm.owner, farm.name, farm.last_interaction.elapsed())).await.unwrap();
        }
        else {
            message.reply(context.http, "You don't have a farm. Use *ch farm new to create one!").await.unwrap();
        }
    }
}