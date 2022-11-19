// noinspection DuplicatedCode

import {getServer, IModeration} from "../../../models/server";
import Discord, {ActionRowBuilder, ButtonBuilder, ButtonStyle} from "discord.js";
import {LogType} from "./functions";

function getPageLogs(allLogs: IModeration[], pageNumber: number) {
    return allLogs.slice((pageNumber - 1) * 5, pageNumber * 5)
}

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("warnings")
        .setDescription("Check the warnings of a member")
        .setDefaultMemberPermissions(Discord.PermissionFlagsBits.ManageMessages)
        .addUserOption(option =>
            option.setName("member")
                .setDescription("Member to check the warnings")
                .setRequired(true)),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        if (!interaction.guild) {
            return;
        }

        const server = await getServer(interaction.guild.id)
        const member = interaction.options.getMember("member")

        if (!(member instanceof Discord.GuildMember)) {
            return interaction.reply({content: "Invalid member mention", ephemeral: true})
        }

        const warnings = server.moderation.filter(log => log.userId == member.id && log.logType == LogType.Warn.toString())

        if (warnings.length == 0) {
            return interaction.reply("No warnings found for that user.");
        }

        const numberOfPages = Math.ceil(warnings.length / 5)
        let pageNumber = 1

        let warningMsg = ""
        for (const log of getPageLogs(warnings, pageNumber)) {
            warningMsg += `**Case ${log.caseNumber}**\n**Type:** ${log.logType}\n**User:** <@${log.userId}>\n**Moderator:** <@${log.moderatorId}>\n**Reason:** ${log.reason}\n\n`
        }

        const embed = new Discord.EmbedBuilder()
            .setTitle(`Logs for ${member.user.username}#${member.user.discriminator}`)
            .setDescription(warningMsg)
            .setColor("#ff0000")
            .setFooter({text: `Page ${pageNumber} of ${numberOfPages}`})

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

        const response = await interaction.reply({embeds: [embed], components: [row]})

        const filter = (buttonInteraction: any) => (
            buttonInteraction.customId == "nextPage"
            || buttonInteraction.customId == "previousPage"
            && buttonInteraction.user.id == interaction.user.id
        );

        const collector = response.createMessageComponentCollector({filter});

        collector.on("collect", (i: Discord.ButtonInteraction) => {
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

            warningMsg = ""
            for (const log of getPageLogs(warnings, pageNumber)) {
                warningMsg += `**Case ${log.caseNumber}**\n**Type:** ${log.logType}\n**User:** <@${log.userId}>\n**Moderator:** <@${log.moderatorId}>\n**Reason:** ${log.reason}\n\n`
            }

            embed.setDescription(warningMsg)
            i.update({embeds: [embed], components: [row]})
        })
    }
}
