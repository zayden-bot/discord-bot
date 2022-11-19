import Discord from "discord.js";
import {getImage} from "./functions";

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("goodnight")
        .setDescription("Send good night message")
        .addUserOption(option =>
            option.setName("member")
                .setDescription("Member to say good night too")),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        if (!(interaction.member instanceof Discord.GuildMember)) {
            return;
        }

        const member = interaction.options.getMember("member") || interaction.member
        if (!(member instanceof Discord.GuildMember)) {
            return interaction.reply("Unknown member mentioned");
        }

        const image = await getImage(interaction.member.id, "goodNight")
        if (!image) {
            return interaction.reply("No \"good night\" image found")
        }

        const embed = new Discord.EmbedBuilder()
            .setTitle(`Good Night, ${member.displayName}`)
            .setImage(image)

        interaction.reply({embeds: [embed]}).then()
    }
}
