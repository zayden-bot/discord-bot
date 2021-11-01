const Discord = require("discord.js")

const field1 = `This server is about the game "College Kings". Supporting the game on patreon helps us a lot, so if you have the resources, consider joining the patreon.
You can always get the newest version on patreon.
Get Act 1 for free here:
https://store.steampowered.com/app/1463120/College_Kings__Act_I/
Get the lastest steam version here:
https://store.steampowered.com/dlc/1463120/
Get the latest patreon version here:
https://patreon.com/collegekings`

const field2 = `
<#747430712617074718> ~ Make sure you are fully familiar with the rules. Ignorance is not a defence.
<#803728389500174378> ~ Public patreon posts are announced here.
<#805765564504473641> ~ You can get your custom roles here, from favourite character to event announcements.

<#745662813036609548> ~ Highly moderated and regulated chat.
<#787774961850646559> ~ Feel free to discuss anything provided it doesn't break the rules.
<#747428461391380532> ~ For theories about College Kings [SPOILERS]
<#789831288558452746> ~ Post your favourite pictures or videos here. NSFW media has to be 2d/3d, no NSFW real life imagery. Ping or message <@615128589918011393> if you're unsure. [NSFW]
<#770621445637799946> ~ Here you can discuss College Kings without having to spoiler mark you messages [SPOILERS]
<#772516507041005618> ~ You can talk about and discuss other games here [NSFW]
<#776139754408247326> ~ Use bot commands here!

<#747428614500384788> ~ Do you need help or did you find any bugs? Make sure to ask about it here.`

const field3 = `**Staff Roles:**
<@&746717374761402438>, <@&807370330388693082>, <@&839484117895610378>, <@&764860044977504318>, <@&803393475440541727>, <@&787003873839022081>, <@&804524527191195668>

**Patreon Roles:**
<@&745663432560345218>, <@&745663409932206112>, <@&745663394543304704>, <@&745663375496708127>, <@&745663351756947656>, <@&745663316776714370>, <@&768568151343497257>

**Fan/Activity Roles:**
<@&787443819024220210>, <@&787445571539304510>, <@&787445900992577556>, <@&787446715057831976>, <@&787447090728796191>, <@&787447252783202326>

**Custom Roles:** <#805765564504473641>
<@&805766527889440778>, <@&805766587704803339>, <@&805766682202079273>, <@&805766763215847454>, <@&805766896233742346>, <@&809240184201412619>, <@&809240269622214676>, <@&836659038666227722>, <@&818365035259887626>, <@&862417421167886357>
Members with level 40+ get access to the latest steam act for free!`

module.exports = async (client, channelId) => {
    const channel = await client.channels.fetch(channelId)

    const updateInfomation = new Discord.MessageEmbed()
    .setAuthor(channel.guild.name, channel.guild.iconURL())
    .addField("College Kings Game", field1)
    .addField("Channels", field2)
    .addField("Roles", field3)
    .setFooter(`Server Created: ${channel.guild.createdAt.getFullYear()}-${channel.guild.createdAt.getMonth()}-${channel.guild.createdAt.getDate()}`)
    .setThumbnail(channel.guild.iconURL())
    
    channel.messages.fetch("830931135780880415").then((message) => { message.edit(updateInfomation) })
}
