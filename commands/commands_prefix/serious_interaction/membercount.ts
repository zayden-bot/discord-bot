import Discord from "discord.js"

module.exports = {
    commands: ["membercount"],
    callback: (message: Discord.Message) => {
        if (!message.guild) {
            return;
        }

        message.channel.send(`**${message.guild.memberCount}** total members`)
    },
}
