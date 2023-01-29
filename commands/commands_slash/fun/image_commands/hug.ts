import Discord from "discord.js";
import {getConnection} from "../../../../servers";
import {IImageSchema} from "../../../../models/global/IImageSchema";

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("hug")
        .setDescription("Send a hug message")
        .addUserOption(option =>
            option.setName("member")
                .setDescription("Member to give a hug too")),


    async execute(interaction: Discord.ChatInputCommandInteraction) {
        const member = interaction.options.getMember("member") || interaction.member
        if (!(member instanceof Discord.GuildMember)) {
            return interaction.reply("Unknown member mentioned");
        }

        const conn = getConnection("Global")
        let images = await conn.model<IImageSchema>("HugImages").find({users: {$in: [member.id]}})
        if (images.length == 0) {
            images = await conn.model<IImageSchema>("HugImages").find()
        }

        const image = images[Math.floor(Math.random() * images.length)]

        const embed = new Discord.EmbedBuilder()
            .setTitle(`Sending hugs to ${member.displayName}`)
            .setImage(image.imageUrl)

        await interaction.reply({embeds: [embed]})
    }
}
