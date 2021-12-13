import Discord from "discord.js"

export default async function (client: Discord.Client, channelId: string) {
    const channel = await client.channels.fetch(channelId)
    if (!channel || !channel.isText()) { return console.error("Invalid channel id") }

    const embed = new Discord.MessageEmbed()
        .setTitle("React below to join a role")
        .addField("Alert Roles", `:one: <@&805766527889440778>`)

        .addField("Character Roles", `:two: <@&805766587704803339>
:three: <@&805766682202079273>
:four: <@&805766763215847454>
:five: <@&805766896233742346>
:six: <@&809240184201412619>
:seven: <@&809240269622214676>
:eight: <@&880607567074721822>
:nine: <@&836659038666227722>`)

        .addField("Frat Roles", `:regional_indicator_a: <@&818365011554336809>
:regional_indicator_b: <@&818365035259887626>
:regional_indicator_c: <@&862417421167886357>`)

        .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566")

        channel.messages.fetch("805767878212452372").then((message) => { message.edit({embeds: [embed]}) })
}