import Discord from "discord.js"

const field1 = `This server is about the game "College Kings". The game is still in active development. Supporting the game on patreon helps us a lot, so if you have the resources, consider joining the patreon.
You can always get the newest version on patreon.
Get Act 1 for free here:
https://store.steampowered.com/app/1463120/
Get the lastest steam version here:
https://store.steampowered.com/dlc/1463120/
Get the latest patreon version here:
https://patreon.com/collegekings`

const field2a = `
<#830927865784565800> ~ Welcome!
<#747430712617074718> ~ Make sure you are fully familiar with the rules. Ignorance is not a defence.
<#747426032453156924> ~ General announcement about the game or the discord server
<#797859157562359888> ~ All discord server events are announced here
<#803728389500174378> ~ Public patreon posts are announced here.
<#885713583948836874> ~ Get to know the team behind College Kings and the discord server
<#806504327307853826> ~ New patrons will be announced here, thanks for the support!
<#867385605553389618> ~ A live development log to see what's happening
<#805765564504473641> ~ You can get your custom roles here, from favourite characters club to event announcements.`

const field2b = `<#745662813036609548> ~ Highly moderated and regulated chat
<#787774961850646559> ~ [NSFW] Feel free to discuss anything provided it doesn't break the rules
<#789831288558452746> ~ [NSFW] Post your favourite pictures or videos here. NSFW media has to be 2d/3d, no NSFW real life imagery. Message <@615128589918011393> if you're unsure
<#880870369198751764> ~ Discussions about Sports usually happen here
<#832695012709892107> ~ 54 61 6c 6b 20 61 62 6f 75 74 20 63 6f 64 65 20 77 69 74 68 20 74 68 65 20 43 6f 64 65 20 4d 6f 6e 6b 65 79 73 2e
<#747428461391380532> ~ [SPOILERS] Theories about the game including content from the latest updates
<#770621445637799946> ~ [SPOILERS] Here you can discuss College Kings without having to spoiler mark you messages
<#817235460467720194> ~ [SPOILERS] Like to rank the ck characters? Let the world know about your ranking here
<#772516507041005618> ~ [NSFW] Discussion around games that aren't College Kings
<#776139754408247326> ~ Please only use bot commands in this channel!`

const field2c = `
<#779064182259449896> ~ Find community suggestions here
<#829463308629180447> ~ You want to ask a question and are not sure if someone else already asked? Take a look in here
<#747428614500384788> ~ Do you need help or did you find any bugs? Make sure to ask about it here.
<#888586464814854194> ~ The launcher gives some weird error or just doesn't work at all? Our Support Team will help you here`

const field3 = `**Staff Roles:**
<@&746717374761402438>, <@&807370330388693082>, <@&839484117895610378>, <@&764860044977504318>, <@&803393475440541727>, <@&787003873839022081>, <@&804524527191195668>, <@&913374071239102504>

**Patreon Roles:**
<@&745663432560345218>, <@&745663409932206112>, <@&745663394543304704>, <@&745663375496708127>, <@&745663351756947656>, <@&768568151343497257>

**Fan/Activity Roles:**
<@&787443819024220210>, <@&787445571539304510>, <@&787445900992577556>, <@&787446715057831976>, <@&787447090728796191>, <@&787447252783202326>

**Custom Roles:** <#805765564504473641>
To be added :)`

module.exports = async function (client: Discord.Client, channelId: string) {
    const channel = await client.channels.fetch(channelId) as Discord.TextChannel
    if (!channel || channel.type !== "GUILD_TEXT") { return console.error("Invalid channel id") }

    const embed = new Discord.MessageEmbed()
    .setAuthor(channel.guild.name, channel.guild.iconURL() as string)
    .addField("College Kings Game", field1)
    .addField("Information channels", field2a)
    .addField("Discussion channels", field2b)
    .addField("Support channels", field2c)
    .addField("Roles", field3)
    .setFooter(`Server Created: ${channel.guild.createdAt.getFullYear()}-${channel.guild.createdAt.getMonth()}-${channel.guild.createdAt.getDate()}`)
    .setThumbnail(channel.guild.iconURL() as string)
    
    channel.messages.fetch("830931135780880415").then((message) => { message.edit({embeds: [embed]}) })
}
