use serenity::all::{Context, CreateEmbed};

use crate::{guilds::college_kings::INFORMATION_CHANNEL_ID, Result};

use super::send_or_update_message;

pub async fn run(ctx: &Context) -> Result<()> {
    let embed = CreateEmbed::new()
        .title("College Kings")
        .description(r#"This server is about the game "College Kings". The game is still in active development. Supporting the game on patreon helps us a lot, so if you have the resources, consider joining the patreon.
You can always get the newest version on patreon.
Get College Kings 1 for free here:
[College Kings 1](https://store.steampowered.com/app/1463120?utm_source=ck_discord_information&utm_medium=link&utm_term=ckact1)
Get the latest version here:
[College Kings 2](https://store.steampowered.com/app/1924480?utm_source=ck_discord_information&utm_medium=link&utm_term=ck2chapter1)
Get the latest patreon version here:
https://patreon.com/collegekings"#)
    .field("Information Channels", r"<#830927865784565800> ~ You're here!
<#747430712617074718> ~ Please make sure you are fully familiar with the rules.
<#747426032453156924> ~ Announcements about the game or the discord server.
<#803728389500174378> ~ Patreon link and information.
<#885713583948836874> ~ Get to know the team behind College Kings.
<#867385605553389618> ~ A live development log to see what's happening
<#805765564504473641> ~ You can get your custom roles here, from favourite characters club to event announcements.", false)
.field("Discussion Channels", "<#745662813036609548> ~ Primarily place for discussing College Kings and related topics.
<#787774961850646559> ~ [NSFW] Chat about anything else here. NSFW content is allowed, but no pornography.
<#1126255347309486110> ~ [NSFW] Post your favourite pictures or videos here. NSFW media has to be 2d/3d, no NSFW real life imagery. Message <@600056695586291762> if you're unsure
<#880870369198751764> ~ Discussions about Sports usually happen here
<#770621445637799946> ~ [SPOILERS] Here you can discuss College Kings without having to spoiler mark you messages
<#747428461391380532> ~ [SPOILERS] Theories about the game including content from the latest updates
<#817235460467720194> ~ [SPOILERS] Like to rank the ck characters? Let the world know about your ranking here
<#1020000314260205670> ~ [NSFW] Discussion around games that aren't College Kings
<#776139754408247326> ~ Please only use bot commands in this channel!", false)
.field("Support Channels", "<#1068790374996377671> ~ Find community suggestions here
<#829463308629180447> ~ You want to ask a question and are not sure if someone else already asked? Take a look in here
<#919950775134847016> ~ Do you need help or did you find any bugs? Make sure to ask about it here.", false)
    .field("Roles", r"**Staff Roles:**
<@&746717374761402438>, <@&807370330388693082>, <@&839484117895610378>, <@&945350072386859028>, <@&803393475440541727>, <@&787003873839022081>, <@&913374071239102504>

**Supporter Roles:**
<@&745663432560345218>, <@&745663409932206112>, <@&745663394543304704>, <@&745663375496708127>, <@&745663351756947656>, <@&768568151343497257>

Fan/Activity Roles:
<@&787443819024220210>, <@&787445571539304510>, <@&787445900992577556>, <@&787446715057831976>, <@&787447090728796191>, <@&787447252783202326>", false)
    .field(
        "Special Mentions",
        "Thank you to <@828728276193116181> for the College Kings' stickers",
        false,
    );

    send_or_update_message(ctx, INFORMATION_CHANNEL_ID, embed).await?;

    Ok(())
}
