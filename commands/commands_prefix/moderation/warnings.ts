// noinspection DuplicatedCode

import {IModeration, IServer} from "../../../models/server";
import Discord, {MessageActionRow, MessageButton} from "discord.js";
import {LogType, setup} from "./functions";

function getPageLogs(allLogs: IModeration[], pageNumber: number) {
    return allLogs.slice((pageNumber - 1) * 5, pageNumber * 5)
}

module.exports = {
    commands: ["warnings"],
    expectedArgs: "<user>",
    minArgs: 1,
    callback: async (message: Discord.Message, server: IServer, args: string[]) => {
        const {member} = await setup(message, args)

        if (!member) {
            await message.reply("Please mention a valid member")
            return
        }

        const warnings = server.moderation.filter(log => log.userId == member.id && log.logType == LogType.Warn.toString())

        if (warnings.length == 0) {
            message.reply("No warnings found for that user.").then()
            return
        }

        const numberOfPages = Math.ceil(warnings.length / 5)
        let pageNumber = 1

        let warningMsg = ""
        for (const log of getPageLogs(warnings, pageNumber)) {
            warningMsg += `**Case ${log.caseNumber}**\n**Type:** ${log.logType}\n**User:** <@${log.userId}>\n**Moderator:** <@${log.moderatorId}>\n**Reason:** ${log.reason}\n\n`
        }

        const embed = new Discord.MessageEmbed()
            .setTitle(`Logs for ${member.user.username}#${member.user.discriminator}`)
            .setDescription(warningMsg)
            .setColor("#ff0000")

        const nextPageButton = new MessageButton()
            .setCustomId("nextPage")
            .setLabel("Next Page")
            .setStyle("PRIMARY")
            .setDisabled(true);

        if (numberOfPages > 1) {
            nextPageButton.setDisabled(false)
        }

        const previousPageButton = new MessageButton()
            .setCustomId("previousPage")
            .setLabel("Previous Page")
            .setStyle("PRIMARY")
            .setDisabled(true);

        const row = new MessageActionRow()
            .addComponents(nextPageButton, previousPageButton);

        const msg = await message.channel.send({embeds: [embed], components: [row]})

        const filter = (interaction: Discord.MessageComponentInteraction) => (
            interaction.customId == nextPageButton.customId
            || interaction.customId == previousPageButton.customId
            && interaction.user.id == message.author.id
        );

        const collector = msg.createMessageComponentCollector({filter});

        collector.on("collect", i => {
            console.log(`Interaction "${i.customId}" was clicked`)

            // Next Page Interaction
            if (i.customId == nextPageButton.customId) {
                pageNumber += 1
            }

            // Previous Page Interaction
            if (i.customId == previousPageButton.customId) {
                pageNumber += 1
            }

            if (pageNumber + 1 > numberOfPages) {
                nextPageButton.setDisabled(true)
            } else {
                nextPageButton.setDisabled(false)
            }

            if (pageNumber == 1) {
                previousPageButton.setDisabled(true)
            } else {
                previousPageButton.setDisabled(false)
            }

            row.setComponents(nextPageButton, previousPageButton)

            warningMsg = ""
            for (const log of getPageLogs(warnings, pageNumber)) {
                warningMsg += `**Case ${log.caseNumber}**\n**Type:** ${log.logType}\n**User:** <@${log.userId}>\n**Moderator:** <@${log.moderatorId}>\n**Reason:** ${log.reason}\n\n`
            }

            embed.setDescription(warningMsg)
            i.update({embeds: [embed], components: [row]})
        })
    },
    permissions: ["MANAGE_MESSAGES"],
}
