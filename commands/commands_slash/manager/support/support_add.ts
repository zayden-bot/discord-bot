import Discord, {ButtonBuilder, ButtonStyle} from "discord.js";
import {getConnection} from "../../../../servers";
import {ISupportFAQ} from "../../../../models/server_settings/SupportFAQSchema";

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("support_add")
        .setDescription("Add a support info")
        .setDefaultMemberPermissions(Discord.PermissionFlagsBits.ManageMessages)
        .addStringOption(option =>
            option.setName("id")
                .setDescription("Enter support ID")
                .setRequired(true))
        .addStringOption(option =>
            option.setName("text")
                .setDescription("Enter support text")
                .setRequired(true)),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        if (!interaction.guild) {
            return;
        }

        const id = interaction.options.getString("id", true)
        const text = interaction.options.getString("text", true)

        const conn = await getConnection(interaction.guild.id)
        const supportCollection = conn.model<ISupportFAQ>("SupportFAQ")
        const existingDocument = await supportCollection.findOne({supportId: id})

        if (!existingDocument) {
            const document = await supportCollection.create({
                id: id,
                answer: text
            })

            await interaction.reply("Successfully added support option")
            await document.save()
            return
        }

        // If ID already exists:
        const confirm_button = new Discord.ButtonBuilder()
            .setCustomId("confirm")
            .setLabel("Confirm")
            .setStyle(ButtonStyle.Success);

        const decline_button = new Discord.ButtonBuilder()
            .setCustomId("decline")
            .setLabel("Decline")
            .setStyle(ButtonStyle.Danger);

        const row = new Discord.ActionRowBuilder<ButtonBuilder>()
            .addComponents(
                confirm_button,
                decline_button
            );

        const embed = new Discord.EmbedBuilder()
            .setTitle(`Support ID: ${id}`)
            .setDescription(existingDocument.answer)
            .setColor("#ff0000")
            .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png");

        const response = await interaction.reply({
            content: `Support ID: \`${id}\` already has answer, **Confirm** to overwrite support answer.\nCurrent support message:`,
            embeds: [embed],
            components: [row]
        });

        const filter = (interaction: any) => (
            interaction.customId == "confirm"
            || interaction.customId == "decline"
            && interaction.user.id == interaction.user.id
        );

        let buttonInteraction;
        try {
            buttonInteraction = await response.awaitMessageComponent({
                filter,
                time: 15_000
            }) as Discord.MessageComponentInteraction;
        } catch {
            return interaction.editReply({content: "Time Expired. Canceled command", embeds: [], components: []})

        }

        console.log(`Interaction "${buttonInteraction.customId}" was clicked`)

        switch (buttonInteraction.customId) {
            case "confirm":
                existingDocument.answer = text
                await existingDocument.save()
                await interaction.editReply({content: "Successfully added support option", embeds: [], components: []})
                return

            case "decline":
                return interaction.editReply({content: "Canceled", embeds: [], components: []})
        }
    },
}
