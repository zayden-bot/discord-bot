import Discord from "discord.js"
import {IServer} from "../../../models/server";

module.exports = {
    commands: ["save", "saves"],
    callback: (message: Discord.Message, server: IServer) => {
        message.channel.send(`We do our best to retain save integrity with every update however due to the dynamic nature of game development saves might break. If you experience a save problem please let us know in <#${server.channels.supportChannel}>\n\nReminder:\nWith the major changes in v0.6 saves before this version will not work. We are sorry for the inconvenience. Use CTRL/TAB to quickly skip through the content you have seen`)
    },
}