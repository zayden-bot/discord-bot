import Discord from "discord.js";
import {ChannelType, ThreadAutoArchiveDuration} from 'discord-api-types/v10';
import {IChannel} from "../../../models/server_settings/ChannelSchema";
import {IMiscellaneous} from "../../../models/server_settings/MiscellaneousSchema";
import {getConnection} from "../../../servers";

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

        const conn = getConnection(guild.id)
        const supportChannel = await conn.model("Channels").findOne<IChannel>({
            id: message.channel.id,
            category: "support"
        });
        if (!supportChannel)
            return;

        const miscellaneous = (await conn.model<IMiscellaneous>("Miscellaneous").find())[0]
        if (message.member.roles.cache.hasAny(...miscellaneous.supportRoles, ...miscellaneous.moderationRoles))
            return;

        // noinspection TypeScriptValidateJSTypes
        const idNumber = miscellaneous.supportThreadId.toLocaleString('en', {
            minimumIntegerDigits: 4,
            useGrouping: false
        })

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

        miscellaneous.supportThreadId += 1

        await Promise.all([
            miscellaneous.save(),
            message.delete(),
            message.channel.bulkDelete(1)
        ])
    }
}
