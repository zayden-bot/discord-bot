import Discord from "discord.js"

module.exports = {
    commands: ["membercount"],
    minArgs: 0,
    maxArgs: 0,
    callback: (message: Discord.Message, args: string[], text: string) => {
        if (!message.guild) { return; }

        message.channel.send(`**${message.guild.memberCount}** total members`)
    },
}
