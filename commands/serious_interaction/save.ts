import Discord from "discord.js"

module.exports = {
    commands: ["save", "saves"],
    callback: (message: Discord.Message, args: string[], text: string) => {
        message.channel.send("We do our best to retain save integrety with every update however due to the dynamic nature of game development saves might break. If you experiance a save problem please let us know in <#747428614500384788>\n\nReminder:\nWith the major changes in v0.6 saves before this version will not work. We are sorry for the inconvenience. Use CTRL/TAB to quickly skip through the content you have seen")
    },
}