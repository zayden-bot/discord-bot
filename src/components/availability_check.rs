use std::collections::HashSet;

use serenity::all::{
    parse_user_mention, ComponentInteraction, Context, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage, Mentionable, UserId,
};

use crate::Result;

pub async fn availability_check(
    ctx: &Context,
    interaction: &ComponentInteraction,
    is_available: bool,
) -> Result<()> {
    let user_id = interaction.user.id;

    let fields = &interaction.message.embeds[0].fields;
    let mut available: HashSet<UserId> = fields[0]
        .value
        .split('\n')
        .filter_map(parse_user_mention)
        .collect();

    let mut unavailable: HashSet<UserId> = fields[1]
        .value
        .split('\n')
        .filter_map(parse_user_mention)
        .collect();

    if is_available {
        available.insert(user_id);
        unavailable.remove(&user_id);
    } else {
        unavailable.insert(user_id);
        available.remove(&user_id);
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
        .await
        .unwrap();

    Ok(())
}
