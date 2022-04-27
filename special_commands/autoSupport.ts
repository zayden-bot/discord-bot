import Discord from "discord.js";
import {Server} from "../models/servers/server";

module.exports = async function (message: Discord.Message) {
    const guild = message.guild
    const messageFiles = [...message.attachments.values()]

    if (!guild ||
        (message.content.length == 0 && messageFiles.length == 0)
        || !message.member
        || message.channel.type !== "GUILD_TEXT") {
        return;
    }

    const server = await Server.findOne({id: guild.id}).exec()

    if (server.channels.supportChannel != message.channel.id
        || message.member.roles.cache.has(server.roles.moderationRole)
        || message.member.roles.cache.has(server.roles.supportRole)) {
        return;
    }


    if (!server.supportThreadId) {
        server.supportThreadId = 0
    }
    // noinspection TypeScriptValidateJSTypes
    const idNumber = server.supportThreadId.toLocaleString('en', {minimumIntegerDigits: 4, useGrouping: false})

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
    server.supportThreadId += 1

    await Promise.all([
        server.save(),
        message.delete(),
        message.channel.bulkDelete(1)
    ])
}