pub mod command_handler {
    use serenity::{model::prelude::Message, prelude::Context};

    use crate::game::game::Shard;

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