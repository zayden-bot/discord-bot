import Discord from "discord.js"

module.exports = {
    commands: ["spoiler", "spoilers"],
    callback: (message: Discord.Message) => {
        message.channel.send("Please keep all conversations about the new update to <#770621445637799946>\nIf you have any bugs or questions please post them in <#747428614500384788> and <#888586464814854194>")
    },
}
