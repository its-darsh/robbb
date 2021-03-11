use super::*;
/// Unban a user.
#[command]
#[usage("unban <user>")]
pub async fn unban(ctx: &client::Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let config = data.get::<Config>().unwrap().clone();

    let guild = msg.guild(&ctx).await.context("Failed to load guild")?;

    let user_id = args
        .single::<UserId>()
        .invalid_usage(&UNBAN_COMMAND_OPTIONS)?;

    guild.unban(&ctx, user_id).await?;

    msg.reply_success(&ctx, format!("Succesfully deyote {}", user_id.mention()))
        .await?;

    config
        .log_bot_action(&ctx, |e| {
            e.author(|a| a.name(&msg.author.name).icon_url(msg.author.face()));
            e.description(format!("{} has been deyote", user_id.mention()));
        })
        .await;

    Ok(())
}