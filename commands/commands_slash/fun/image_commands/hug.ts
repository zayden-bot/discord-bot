import Discord from "discord.js";
import {getImage} from "./image_functions";

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("hug")
        .setDescription("Send a hug message")
        .addUserOption(option =>
            option.setName("member")
                .setDescription("Member to give a hug too")),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        if (!(interaction.member instanceof Discord.GuildMember)) {
            return;
        }

        const member = interaction.options.getMember("member") || interaction.member
        if (!(member instanceof Discord.GuildMember)) {
            return interaction.reply("Unknown member mentioned");
        }

        const image = await getImage(interaction.member.id, "hug")
        if (!image) {
            return interaction.reply("No \"hug\" image found")
        }

        const embed = new Discord.EmbedBuilder()
            .setTitle(`Sending hugs to ${member.displayName}`)
            .setImage(image)

        interaction.reply({embeds: [embed]}).then()
    }
}
