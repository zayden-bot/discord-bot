const Discord = require("discord.js");

module.exports = {
    commands: ["custom", "cm"],
    callback: (message, arguments, text) => {
        const embed = new Discord.MessageEmbed()
            .setTitle(`Who's Your Favourite Girl`)
            .setDescription(`
            :one: Riley

            :two: Lauren

            :three: Amber

            :four: Kim

            :five: Autumn

            <:penelope:774987041838334002> Penelope

            <:nora:774987041649721345> Nora

            <:julia:774987041888927784> Julia

            <:emily:774987041795604510> Emily

            <:chloe2:774987041922220042> Chloe

            <:aubrey:774987041796653066> Aubrey
            `)
            .setColor("0000ff")
            .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566")

        message.channel.send(embed).then(function(message) {
            message.react("1️⃣")
            message.react("2️⃣")
            message.react("3️⃣")
            message.react("4️⃣")
            message.react("5️⃣")
            message.react("<:penelope:774987041838334002>")
            message.react("<:nora:774987041649721345>")
            message.react("<:julia:774987041888927784>")
            message.react("<:emily:774987041795604510>")
            message.react("<:chloe2:774987041922220042>")
            message.react("<:aubrey:774987041796653066>")
        })
    },
    requiredRoles: "Security",
}
