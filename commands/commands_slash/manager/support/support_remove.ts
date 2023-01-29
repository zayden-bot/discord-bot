import Discord, {ActionRowBuilder, ButtonBuilder, ButtonStyle} from "discord.js";
import {getConnection} from "../../../../servers";
import {ISupportFAQ} from "../../../../models/server_settings/SupportFAQSchema";

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("support_remove")
        .setDescription("Remove an existing support ID")
        .setDefaultMemberPermissions(Discord.PermissionFlagsBits.ManageMessages)
        .addStringOption(option =>
            option.setName("id")
                .setDescription("Enter support ID")
                .setRequired(true)),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        if (!interaction.guild) {
            return;
        }

        const id = interaction.options.getString("id", true).toLowerCase();

        const conn = getConnection(interaction.guild.id)
        const answer = await conn.model<ISupportFAQ>("SupportFAQ").findOne({supportId: id})
        if (!answer) {
            return interaction.reply({content: "No support ID found", ephemeral: true});
        }

        // If ID already exists:
        const confirmButton = new ButtonBuilder()
            .setCustomId("confirm")
            .setLabel("Confirm")
            .setStyle(ButtonStyle.Success);

        const declineButton = new ButtonBuilder()
            .setCustomId("decline")
            .setLabel("Decline")
            .setStyle(ButtonStyle.Danger);

        const row = new ActionRowBuilder<ButtonBuilder>()
            .addComponents(confirmButton, declineButton);

        const embed = new Discord.EmbedBuilder()
            .setTitle(`Support ID: ${id}`)
            .setDescription(answer.answer)
            .setColor("#ff0000")
            .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png");

        const response = await interaction.reply({
            content: `**Confirm** deletion of \`${id}\`.\nCurrent support message:`,
            embeds: [embed],
            components: [row]
        });

        const filter = (buttonInteraction: any) => (
            buttonInteraction.customId == "confirm"
            || buttonInteraction.customId == "decline"
            && buttonInteraction.user.id == interaction.user.id
        );

        let buttonInteraction;
        try {
            buttonInteraction = await response.awaitMessageComponent({
                filter,
                time: 15_000
            }) as Discord.MessageComponentInteraction
        } catch {
            return interaction.editReply({content: "Time Expired. Canceled command", embeds: [], components: []})
        }

        console.log(`Interaction "${buttonInteraction.customId}" was clicked`)

        if (buttonInteraction.customId == "confirm") {
            await answer.remove()
            return interaction.editReply({content: "Successfully removed support option", embeds: [], components: []})
        } else if (buttonInteraction.customId == "decline") {
            return interaction.editReply({content: "Canceled", embeds: [], components: []})
        }
    },
}
