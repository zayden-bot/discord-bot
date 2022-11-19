import Discord, {ButtonBuilder, ButtonStyle} from "discord.js";
import {getServer} from "../../../../models/server";

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

        const server = await getServer(interaction.guild.id)

        const id = interaction.options.getString("id", true)
        const text = interaction.options.getString("text", true)

        if (!server.supportAnswers.get(id)) {
            server.supportAnswers.set(id, text);

            return await Promise.all([
                server.save(),
                interaction.reply("Successfully added support option")
            ])
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
            .setDescription(server.supportAnswers.get(id) ?? "")
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
                server.supportAnswers.set(id, text);
                return await Promise.all([
                    server.save(),
                    interaction.editReply({content: "Successfully added support option", embeds: [], components: []})
                ])
            case "decline":
                return interaction.editReply({content: "Canceled", embeds: [], components: []})
        }
    },
}
