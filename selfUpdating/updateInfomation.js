const Discord = require("discord.js")

const field1 = `"This server is about the game "College Kings". You can always  Supporting the game on patreon helps us a lot, so if you have the resources, consider joining the patreon.
You can always get the newest version on patreon.
https://store.steampowered.com/app/1463120/College_Kings__Act_I/
https://patreon.com/collegekings

__**Channels:**__`

const field2 = `<#830927865784565800>
<#747430712617074718>
<#747426032453156924> ~ Announcements about the game and the server
<#797859157562359888> ~ Information about events
<#803728389500174378> ~ New patreon posts are announced here
<#806504327307853826> ~ If someone joins the patreon server it is announced here to celebrate the awesomeness of that person!`

const field3 = `<#745662813036609548> ~ Highly moderated and regulated chat.
<#787774961850646559> ~ Everything that doesn't fit in another channel and memes go here
<#832695012709892107> ~ 0100111001100101011100100110010001110011 
<#747428461391380532> ~ (spoilers) for theories about College Kings
<#817235460467720194> ~ (spoilers) Make your own tierlist from the pinned links and post it here
<#789831288558452746> ~ (nsfw) Post your or other art. If it isn't your art, make sure to credit an official publication from the artist
<#770621445637799946> ~ (spoilers) Here you can discuss College Kings without having to spoiler mark you messages
<#772516507041005618> ~ (nsfw) You can talk about and send screenshots of other games here
<#776139754408247326> ~ Use bot commands here, unless they add to a conversation in another channel`

const field4 = `<#779064182259449896> ~ Vote for your favourite suggestions here
<#829463308629180447> ~ For any questions about the game or the developers
<#747428614500384788> ~ Do you need help or did you find any bugs? Make sure to ask about it here.`

const field5 = `<#789990024220377159> ~ a chat for talking about things that happen in the voice channels
<#796400464358670344> ~ a chat for music commands when listening to music in the Groovy VC
General ~ Talk about anything in here (without breaking the rules)
Groovy ~ Listen to some music together with <@234395307759108106>

__**Roles:**__`

const field6 = `The server staff and developers are having special roles so you can find them easily.
<@&746717374761402438>, <@&807370330388693082>, <@&807366743546396782>, <@&764860044977504318>, <@&803393475440541727>, <@&787003873839022081>, <@&804524527191195668>`

const field7 = `If you join the patreon (which you absolutely should), you can gain an awesome colourful role. 
<@&745663432560345218>, <@&745663409932206112>, <@&745663394543304704>, <@&745663375496708127>, <@&745663351756947656>, <@&745663316776714370>, <@&768568151343497257>`

const field8 = `Fan roles are pink and optained through activity in the server. You will get your first one at level 5, which allows you to post images in all the channels.
<@&787443819024220210>, <@&787445571539304510>, <@&787445900992577556>, <@&787446715057831976>, <@&787447090728796191>, <@&787447252783202326>`

const field9 = `Join a College Kings Club below!
<@&805766527889440778>, <@&805766587704803339>, <@&805766682202079273>, <@&805766763215847454>, <@&805766896233742346>, <@&809240184201412619>, <@&809240269622214676>, <@&825746784688668713>, <@&818365011554336809>, <@&818365035259887626>, <@&836659038666227722>`

module.exports = async (client, channelId) => {
    const channel = await client.channels.fetch(channelId)

    const updateInfomation = new Discord.MessageEmbed()
    .setAuthor(channel.guild.name, channel.guild.iconURL())
    .addField("College Kings Game:", field1)
    .addField("- Information", field2)
    .addField("- Discussion", field3)
    .addField("- Support", field4)
    .addField("- Voice Channels", field5)
    .addField("- Staff Roles", field6)
    .addField("- Patreon Roles", field7)
    .addField("- Fan roles", field8)
    .addField("- Clubs", field9)
    .setFooter(`Server Created: ${channel.guild.createdAt.getFullYear()}-${channel.guild.createdAt.getMonth()}-${channel.guild.createdAt.getDate()}`)
    .setThumbnail(channel.guild.iconURL())
    
    channel.messages.fetch("830931135780880415").then((message) => { message.edit(updateInfomation) })
}
