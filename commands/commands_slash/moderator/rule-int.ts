import Discord from "discord.js";
import {getConnection} from "../../../servers";
import {IRule} from "../../../models/server_settings/RulesSchema";

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("rule")
        .setDescription("Get rule information with given ID")
        .addStringOption(option =>
            option.setName("id")
                .setDescription("Enter rule ID")
                .setRequired(true)),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        if (!interaction.guild) {
            return;
        }

        const id = interaction.options.getString("id", true);

        const conn = getConnection(interaction.guild.id)
        const rule = await conn.model<IRule>("Rules").findOne({ruleId: id})

        if (!rule) {
            return interaction.reply(`There is no rule with the id ${id}`);
        }

        const embed = new Discord.EmbedBuilder()
            .setTitle(`Rule ${id}`)
            .setDescription(`**${id}.** ${rule}\n\n**Please read the rest of the rules in <#747430712617074718>!**`)
            .setColor("#ff0000")
            .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png");

        interaction.reply({embeds: [embed]}).then();
    },
}
