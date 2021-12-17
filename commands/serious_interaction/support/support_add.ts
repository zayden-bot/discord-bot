import Discord from "discord.js";
import { servers } from "../../../server";

module.exports = {
    commands: ["support_add"],
    expectedArgs: "<id>, <support_answer>",
    minArgs: 1,
    callback: async (message: Discord.Message, args: string[], text: string) => {
        const guild = message.guild
        if (!guild) { return; }

        const server = servers[guild.id]

        args = text.split(',')
        const id = args.shift()?.toLowerCase() as string
        const answer = args.join(',');
        
        if (!answer) { return message.reply("Comma seperator not found.")}

        if (!server.supportAnswers[id]) {
            server.supportAnswers[id] = answer;

            const common = require("../../../common")
            common.updateConfig(guild, server)

            return message.reply("Successfully added support option");
        }

        // If ID already exists:
        const confirm_button = new Discord.MessageButton()
        .setCustomId("confirm")
        .setLabel("Confirm")
        .setStyle("SUCCESS");

        const decline_button = new Discord.MessageButton()
        .setCustomId("decline")
        .setLabel("Decline")
        .setStyle("DANGER");

        const row = new Discord.MessageActionRow()
        .addComponents(
            confirm_button,
            decline_button
        );
        
        const embed = new Discord.MessageEmbed()
        .setTitle(`Support ID: ${id}`)
        .setDescription(server.supportAnswers[id])
        .setColor("#ff0000")
        .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png");

        const msg = await message.channel.send({content: `Support ID: \`${id}\` already has answer, **Confirm** to overwrite support answer.\nCurrent support message:`, embeds: [embed], components: [row]});

        const filter = (interaction: Discord.MessageComponentInteraction) => (interaction.customId == confirm_button.customId || interaction.customId == decline_button.customId) && interaction.user.id == message.author.id;
        msg.awaitMessageComponent({ filter, time: 15_000 })
        .then(async (interaction) => {
            console.log(`Interaction "${interaction.customId}" was clicked`)

            if (interaction.customId == confirm_button.customId) {
                server.supportAnswers[id] = answer;

                const common = require("../../../common")
                common.updateConfig(guild, server)
                return interaction.update({ content: "Successfully added support option", embeds: [], components: [] })
            }
            else {
                return interaction.update({ content: "Canceled", embeds: [], components: [] })
            }
        })
        .catch((error) => {
            msg.edit({ content: "Time Expired. Canceled command", embeds: [], components: [] })
        });        
    },
    requiredRoles: ["Admin"]
}