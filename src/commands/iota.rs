use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};
use anyhow::Result;

use crate::utils::random_seed;

#[command]
fn generate_seed(ctx: &mut Context, msg: &Message) -> CommandResult {


    let seed = random_seed::new();

    let _ = msg.channel_id.say(&ctx.http, seed);

    Ok(())
}

#[command]
fn get_node_info(ctx: &mut Context, msg: &Message) -> CommandResult {


    let _ = msg.channel_id.say(&ctx.http, "Pong!");

    _get_node_info();

    Ok(())
}


async fn _get_node_info() -> Result<()>  {
    iota::Client::add_node("https://nodes.comnet.thetangle.org:443")?;
    let node_info = iota::Client::get_node_info().await?;
    println!("{:#?}", node_info);

    Ok(())
}