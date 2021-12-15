import Discord from "discord.js"

module.exports = async function (client: Discord.Client, channelId: string) {
    const channel = await client.channels.fetch(channelId)
    if (!channel || !channel.isText()) { return console.error("Invalid channel id") }

    const embed = new Discord.MessageEmbed()
        .setTitle("Joint a fanclub by clicking the icons below!")
        .addField("Get a notification for Discord Server Events!", `:bell: <@&805766527889440778>`)

        .addField("Join one of the San Vallejo Fraternities!", `:rr_apes: <@&818365011554336809>
:rr_wolves: <@&818365035259887626>`)

        .addField("Choose a girl's fanclub to show your affection!", `:rr_amber: <@&920365569432244264>
:rr_aubrey: <@&920365674591813693>
:rr_autumn: <@&920365776500822027>
:rr_candy: <@&920365867232014347>
:rr_chloe: <@&920365910387228793>
:rr_emily: <@&920365961507377162>
:rr_evelyn: <@&920366002200514621>
:rr_jenny: <@&920366043044671539>
:rr_lauren: <@&920366084232728706>
:rr_lindsey: <@&920366242773229598>
:rr_lorraine: <@&920366319436722206>
:rr_naomi: <@&920366504279670865>
:rr_nora: <@&920366553252380693>
:rr_penelope: <@&920366589239509002>
:rr_riley: <@&920366629462876252>
:rr_samantha: <@&920366671733092412>
:rr_satin: <@&920366732491776121>`)

        .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566")

        channel.messages.fetch("805767878212452372").then((message) => { message.edit({embeds: [embed]}) })
}
