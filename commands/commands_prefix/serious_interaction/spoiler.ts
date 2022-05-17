import Discord from "discord.js"
import {IServer} from "../../../models/server";

module.exports = {
    commands: ["spoiler", "spoilers"],
    callback: (message: Discord.Message, server: IServer) => {
        message.channel.send(`Please keep all conversations about the new update to <#770621445637799946>\nIf you have any bugs or questions please post them in <#${server.channels.supportChannel}>`)
    },
}
