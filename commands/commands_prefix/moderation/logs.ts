import {IModeration, IServer} from "../../../models/server";
import Discord, {ActionRowBuilder, ButtonBuilder, ButtonStyle} from "discord.js";
import {setup} from "./functions";

function getPageLogs(allLogs: IModeration[], pageNumber: number) {
    return allLogs.slice((pageNumber - 1) * 5, pageNumber * 5)
}

module.exports = {
    commands: ["logs", "log"],
    expectedArgs: "<user>",
    minArgs: 1,
    callback: async (message: Discord.Message, server: IServer, args: string[]) => {
        const {member} = await setup(message, args)

        if (!member) {
            message.reply("Please mention a valid member").then()
            return
        }

        const logs = server.moderation.filter(log => log.userId == member.id)

        if (logs.length == 0) {
            message.reply("No logs found for that user.").then()
            return
        }

        const numberOfPages = Math.ceil(logs.length / 5)
        let pageNumber = 1

        let logMsg = ""
        for (const log of getPageLogs(logs, pageNumber)) {
            logMsg += `**Case ${log.caseNumber}**\n**Type:** ${log.logType}\n**User:** <@${log.userId}>\n**Moderator:** <@${log.moderatorId}>\n**Reason:** ${log.reason}\n\n`
        }

        const embed = new Discord.EmbedBuilder()
            .setTitle(`Logs for ${member.user.username}#${member.user.discriminator}`)
            .setDescription(logMsg)
            .setColor("#ff0000")

        const nextPageButton = new ButtonBuilder()
            .setCustomId("nextPage")
            .setLabel("Next Page")
            .setStyle(ButtonStyle.Primary)
            .setDisabled(true);

        if (numberOfPages > 1) {
            nextPageButton.setDisabled(false)
        }

        const previousPageButton = new ButtonBuilder()
            .setCustomId("previousPage")
            .setLabel("Previous Page")
            .setStyle(ButtonStyle.Primary)
            .setDisabled(true);

        const row = new ActionRowBuilder<ButtonBuilder>()
            .addComponents(nextPageButton, previousPageButton);

        const msg = await message.channel.send({embeds: [embed], components: [row]})

        const filter = (interaction: Discord.MessageComponentInteraction) => (
            interaction.customId == "nextPage"
            || interaction.customId == "previousPage"
            && interaction.user.id == message.author.id
        );

        const collector = msg.createMessageComponentCollector({filter});

        collector.on("collect", i => {
            console.log(`Interaction "${i.customId}" was clicked`)

            // Next Page Interaction
            if (i.customId == "nextPage") {
                pageNumber += 1
            }

            // Previous Page Interaction
            if (i.customId == "previousPage") {
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

            logMsg = ""
            for (const log of getPageLogs(logs, pageNumber)) {
                logMsg += `**Case ${log.caseNumber}**\n**Type:** ${log.logType}\n**User:** <@${log.userId}>\n**Moderator:** <@${log.moderatorId}>\n**Reason:** ${log.reason}\n\n`
            }

            embed.setDescription(logMsg)
            i.update({embeds: [embed], components: [row]})
        })
    },
    permissions: ["MANAGE_MESSAGES"],
}
