use serenity::all::{
    ComponentInteraction, Context, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage, Mentionable, UserId,
};

use crate::Result;

pub async fn availability_check(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
    let user_id = interaction.user.id;

    let fields = &interaction.message.embeds[0].fields;
    let mut available: Vec<UserId> = fields[0]
        .value
        .split('\n')
        .filter_map(|v| v.parse().ok().map(UserId::new))
        .collect();
    let mut unavailable: Vec<UserId> = fields[1]
        .value
        .split('\n')
        .filter_map(|v| v.parse().ok().map(UserId::new))
        .collect();

    match interaction.data.custom_id.as_str() {
        "cron_available" => {
            if !available.contains(&user_id) {
                available.push(user_id);
            }
            unavailable.retain(|&x| x != user_id);
        }
        "cron_unavailable" => {
            if !unavailable.contains(&user_id) {
                unavailable.push(user_id);
            }
            available.retain(|&x| x != user_id);
        }
        _ => unreachable!("Invalid custom_id"),
    }

    interaction
        .create_response(
            &ctx,
            CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::default().embed(
                    CreateEmbed::default()
                        .title("Are you available for tomorrow's meeting?")
                        .field(
                            "Attending",
                            available.iter().fold(String::new(), |mut output, user_id| {
                                output.push_str(&format!("\n{}", user_id.mention()));
                                output
                            }),
                            true,
                        )
                        .field(
                            "Unavailable",
                            unavailable
                                .iter()
                                .fold(String::new(), |mut output, user_id| {
                                    output.push_str(&format!("\n{}", user_id.mention()));
                                    output
                                }),
                            true,
                        ),
                ),
            ),
        )
        .await?;

    Ok(())
}
