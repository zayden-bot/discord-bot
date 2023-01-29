import Discord, {ButtonBuilder, ButtonStyle} from "discord.js"
import {ComponentType} from "discord-api-types/v10"
import {getConnection} from "../../../../servers";
import {ISupportFAQ} from "../../../../models/server_settings/SupportFAQSchema";

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("support_list")
        .setDescription("Get list of valid support IDs")
        .setDefaultMemberPermissions(Discord.PermissionFlagsBits.MoveMembers),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        if (!interaction.guild) {
            return;
        }

        const embed = new Discord.EmbedBuilder()
            .setTitle("List of support options")
            .setColor("#ff0000")
            .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png");

        const pageSize = 5
        let pageNumber = 1

        const conn = getConnection(interaction.guild.id)
        const supportCollection = await conn.model<ISupportFAQ>("SupportFAQ")
        let supportEntries = await supportCollection.find().limit(pageSize)

        for (const entry of supportEntries) {
            embed.spliceFields(-1, 0, {name: entry.id, value: entry.answer});
        }

        const nextPage = new Discord.ButtonBuilder()
            .setCustomId("next-page")
            .setLabel("Next Page")
            .setStyle(ButtonStyle.Primary)

        const previousPage = new Discord.ButtonBuilder()
            .setCustomId("prev-page")
            .setLabel("Previous Page")
            .setStyle(ButtonStyle.Primary)
            .setDisabled(true);

        if (pageNumber > Math.ceil((await supportCollection.count()) / pageSize))
            nextPage.setDisabled(true)

        const row = new Discord.ActionRowBuilder<ButtonBuilder>()
            .addComponents(nextPage, previousPage)

        const msg = await interaction.reply({embeds: [embed], components: [row]});

        const filter = (buttonInteraction: Discord.MessageComponentInteraction) => (
            ["next-page", "prev-page"].includes(buttonInteraction.customId) &&
            buttonInteraction.user.id == interaction.user.id);

        const collector = msg.createMessageComponentCollector({filter, componentType: ComponentType.Button})

        collector.on("collect", async (i) => {
            if (i.customId == "next-page") {
                pageNumber++;
                previousPage.setDisabled(false);

                const embed = new Discord.EmbedBuilder()
                    .setTitle("List of support options")
                    .setColor("#ff0000")
                    .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png");

                supportEntries = await supportCollection.find().skip((pageNumber - 1) * pageSize).limit(pageSize)

                for (const entry of supportEntries) {
                    embed.spliceFields(-1, 0, {name: entry.id, value: entry.answer});
                }

                if (pageNumber > Math.ceil((await supportCollection.count()) / pageSize))
                    nextPage.setDisabled(true)

                const row = new Discord.ActionRowBuilder<ButtonBuilder>()
                    .addComponents(nextPage, previousPage)

                i.update({embeds: [embed], components: [row]})
            }

            if (i.customId == "prev-page") {
                pageNumber--;

                const embed = new Discord.EmbedBuilder()
                    .setTitle("List of support options")
                    .setColor("#ff0000")
                    .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png");

                supportEntries = await supportCollection.find().skip((pageNumber - 1) * pageSize).limit(pageSize)
                for (const entry of supportEntries) {
                    embed.spliceFields(-1, 0, {name: entry.id, value: entry.answer});
                }
                
                if (pageNumber <= 1) {
                    previousPage.setDisabled(true);
                }

                const row = new Discord.ActionRowBuilder<ButtonBuilder>()
                    .addComponents(previousPage, nextPage)

                i.update({embeds: [embed], components: [row]})
            }
        })

        collector.on("end", (_collected, reason) => {
            console.log("Ended collector", reason)
        })
    }
}
