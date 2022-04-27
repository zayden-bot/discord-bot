import Discord from "discord.js";
import {servers} from "../servers";

module.exports = async function (message: Discord.Message) {
    const guild = message.guild
    const messageFiles = [...message.attachments.values()]

    if (!guild ||
        (message.content.length == 0 && messageFiles.length == 0)
        || !message.member
        || message.channel.type !== "GUILD_TEXT"
        || message.author.id == "787490197943091211") {
        return;
    }

    const server = servers[message.guild.id]
    if (!server.channels.supportChannels
        || !server.channels.supportChannels.includes(message.channel.id)
        || message.member.roles.cache.has(server.roles.moderationRole)
        || message.member.roles.cache.has(server.roles.supportRole)) {
        return;
    }

    // noinspection TypeScriptValidateJSTypes
    const idNumber = server.idNumber.toLocaleString('en', {minimumIntegerDigits: 4, useGrouping: false})

    // Create channel thread and send mentions
    let threadName = `${idNumber} - ${message.content}`
    if (threadName.length > 100) {
        threadName = threadName.substring(0, 100)
    }

    const thread = await message.channel.threads.create({
        name: threadName,
        autoArchiveDuration: 'MAX',
    })

    thread.send(`<@&913374071239102504> ${message.author} wrote:`)

    const threadMessages = message.content.match(/(.|[\r\n]){1,2000}/g);

    if (threadMessages) {
        threadMessages.forEach(messageContent => {
            thread.send({
                content: messageContent,
                embeds: message.embeds,
                files: messageFiles
            })
        })
    } else {
        thread.send({
            embeds: message.embeds,
            files: messageFiles
        })
    }

    // Update json file.
    server.idNumber += 1
    require("../common").updateConfig(guild, server)

    await Promise.all([message.delete(), message.channel.bulkDelete(1)])
}