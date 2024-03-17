use std::time::Duration;

use crate::commands::message_commands::levels::user_levels::{
    get_user_row_number, get_users, UserLevelData, LIMIT,
};
use serenity::{
    all::{
        CommandInteraction, ComponentInteractionCollector, Context, CreateActionRow, CreateButton,
        CreateCommand, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage,
        EditInteractionResponse, Message, UserId,
    },
    futures::StreamExt,
};

pub async fn run(
    ctx: Context,
    interaction: &CommandInteraction,
) -> Result<Message, serenity::Error> {
    interaction.defer(&ctx).await?;

    let mut page_number = 1;
    let mut users = get_users(page_number).await.unwrap();

    let mut embed = CreateEmbed::default().title("Leaderboard");
    embed = add_user_to_embed(&ctx, embed, &users).await;

    let response = interaction
        .edit_response(
            &ctx,
            EditInteractionResponse::new()
                .embed(embed)
                .components(build_components(
                    page_number,
                    is_user_visable(&users, interaction.user.id),
                )),
        )
        .await?;

    let mut collector = ComponentInteractionCollector::new(&ctx)
        .message_id(response.id)
        .timeout(Duration::from_secs(3600))
        .stream();

    while let Some(c) = StreamExt::next(&mut collector).await {
        let mut embed = CreateEmbed::default().title("Leaderboard");

        match c.data.custom_id.as_str() {
            "previous" => {
                page_number -= 1;

                users = get_users(page_number).await.unwrap();
                embed = add_user_to_embed(&ctx, embed, &users).await;
            }
            "user" => {
                let row_number = get_user_row_number(c.user.id.get()).await.unwrap().unwrap();
                let page_number = row_number / LIMIT + 1;

                users = get_users(page_number).await.unwrap();
                embed = add_user_to_embed(&ctx, embed, &users).await;
            }
            "next" => {
                page_number += 1;

                users = get_users(page_number).await.unwrap();
                embed = add_user_to_embed(&ctx, embed, &users).await;
            }
            _ => unreachable!(),
        };

        c.create_response(
            &ctx,
            CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::default()
                    .embed(embed)
                    .components(build_components(
                        page_number,
                        is_user_visable(&users, c.user.id),
                    )),
            ),
        )
        .await?;
    }

    Ok(Message::default())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("levels").description("Get the leaderboard")
}

fn build_components(page_number: i32, is_user_visable: bool) -> Vec<CreateActionRow> {
    let mut previous_button = CreateButton::new("previous").label("<");
    if page_number == 1 {
        previous_button = previous_button.disabled(true);
    }

    let mut user_button = CreateButton::new("user").emoji('🎯');
    if is_user_visable {
        user_button = user_button.disabled(true);
    }

    let buttons = vec![
        previous_button,
        user_button,
        CreateButton::new("next").label(">"),
    ];

    vec![CreateActionRow::Buttons(buttons)]
}

fn is_user_visable(users: &[UserLevelData], user_id: UserId) -> bool {
    users.iter().any(|user| user.id == user_id.get() as i64)
}

async fn add_user_to_embed(
    ctx: &Context,
    mut embed: CreateEmbed,
    users: &Vec<UserLevelData>,
) -> CreateEmbed {
    for level_user in users {
        let user = ctx
            .http
            .get_user(UserId::new(level_user.id as u64))
            .await
            .unwrap();
        embed = embed.field(
            user.name,
            format!(
                "Msgs: {} | Total XP: {} | Level: {}",
                level_user.message_count, level_user.total_xp, level_user.level
            ),
            false,
        );
    }
    embed
}
