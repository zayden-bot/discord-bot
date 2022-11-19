import Discord from "discord.js";
import {getImage} from "./functions";

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("goodmorning")
        .setDescription("Send good morning message")
        .addUserOption(option =>
            option.setName("member")
                .setDescription("Member to say good morning too")),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        if (!(interaction.member instanceof Discord.GuildMember)) {
            return;
        }

        const member = interaction.options.getMember("member") || interaction.member
        if (!(member instanceof Discord.GuildMember)) {
            return interaction.reply("Unknown member mentioned");
        }

        const image = await getImage(interaction.member.id, "goodMorning")
        if (!image) {
            return interaction.reply("No \"good morning\" image found")
        }

        const embed = new Discord.EmbedBuilder()
            .setTitle(`Good Morning, ${member.displayName}`)
            .setImage(image)

        interaction.reply({embeds: [embed]}).then()
    }
}
