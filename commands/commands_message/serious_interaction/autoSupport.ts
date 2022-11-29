import Discord from "discord.js";
import {getServer} from "../../../models/server";
import {ChannelType, ThreadAutoArchiveDuration} from 'discord-api-types/v10';

module.exports = {
    command: "questionMe",
    callback: async (message: Discord.Message) => {
        const guild = message.guild
        const messageFiles = [...message.attachments.values()]

        if (!guild ||
            (message.content.length == 0 && messageFiles.length == 0)
            || !message.member
            || message.channel.type !== ChannelType.GuildText) {
            return;
        }

        const server = await getServer(guild.id)

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
            autoArchiveDuration: ThreadAutoArchiveDuration.OneWeek,
        })

        thread.send(`<@&913374071239102504> ${message.author} wrote:`)

        const threadMessages = message.content.match(/(.|[\r\n]){1,2000}/g);

        if (threadMessages) {
            threadMessages.forEach(messageContent => {
                thread.send({content: messageContent})
            })
        }

        if (message.embeds.length != 0 || messageFiles.length != 0) {
            thread.send({
                embeds: message.embeds,
                files: messageFiles
            })
        }

        server.supportThreadId += 1

        Promise.all([
            server.save(),
            message.delete(),
            message.channel.bulkDelete(1)
        ]).then()
    }
}
