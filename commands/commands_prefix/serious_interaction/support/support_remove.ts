import Discord, {MessageActionRow, MessageButton} from "discord.js";
import {IServer} from "../../../../models/server";

module.exports = {
    commands: ["support_remove"],
    expectedArgs: "<id>",
    minArgs: 1,
    callback: async (message: Discord.Message, server: IServer, args: string[], text: string) => {
        const guild = message.guild
        if (!guild) {
            return;
        }

        const id = text.toLowerCase()
        const answer = server.supportAnswers.get(id)
        if (!answer) {
            return message.reply("No support ID found");
        }

        // If ID already exists:
        const confirmButton = new MessageButton()
            .setCustomId("confirm")
            .setLabel("Confirm")
            .setStyle("SUCCESS");

        const declineButton = new MessageButton()
            .setCustomId("decline")
            .setLabel("Decline")
            .setStyle("DANGER");

        const row = new MessageActionRow()
            .addComponents(confirmButton, declineButton);

        const embed = new Discord.MessageEmbed()
            .setTitle(`Support ID: ${id}`)
            .setDescription(answer)
            .setColor("#ff0000")
            .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png");

        const msg = await message.channel.send({
            content: `**Confirm** deletion of \`${id}\`.\nCurrent support message:`,
            embeds: [embed],
            components: [row]
        });

        const filter = (interaction: Discord.MessageComponentInteraction) => (
            interaction.customId == confirmButton.customId
            || interaction.customId == declineButton.customId
            && interaction.user.id == message.author.id
        );

        let interaction;
        try {
            interaction = await msg.awaitMessageComponent({filter, time: 15_000})
        } catch {
            msg.edit({content: "Time Expired. Canceled command", embeds: [], components: []})
            return;
        }

        console.log(`Interaction "${interaction.customId}" was clicked`)

        if (interaction.customId == confirmButton.customId) {
            server.supportAnswers.delete(id)
            await server.save()

            interaction.update({content: "Successfully removed support option", embeds: [], components: []})
            return;
        }
        if (interaction.customId == declineButton.customId) {
            interaction.update({content: "Canceled", embeds: [], components: []})
            return;
        }
    },
}