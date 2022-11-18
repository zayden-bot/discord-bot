import Discord from "discord.js";
import {getUserConfig, IUserConfig} from "../../../models/user-config";
import {getServer, IServer} from "../../../models/server";

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("givestar")
        .setDescription("Give a user a star")
        .addUserOption(option =>
            option.setName("member")
                .setDescription("Member to give star too")
                .setRequired(true))
        .addStringOption(option =>
            option.setName("reason")
                .setDescription("Reason for the star")),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        const author = interaction.member;
        const member = interaction.options.getMember("member");
        const reason = interaction.options.getString("reason")

        if (!(author instanceof Discord.GuildMember) || !(member instanceof Discord.GuildMember) || !interaction.guild) {
            return;
        }

        if (author.id == member.id) {
            return interaction.reply({content: "You idiot...", ephemeral: true});
        }

        const server: IServer = await getServer(interaction.guild.id)
        const member_config: IUserConfig = await getUserConfig(member.id)
        const author_config: IUserConfig = await getUserConfig(author.id)

        if (author_config.stars.numberOfStars <= 0 && !author.roles.cache.has(server.roles.moderationRole)) {
            return interaction.reply("Unable. You have no gold stars to give.");
        }

        if (!author.roles.cache.has(server.roles.moderationRole)) {
            author_config.stars.numberOfStars -= 1;
        }
        author_config.stars.givenStars += 1;
        member_config.stars.numberOfStars += 1;
        member_config.stars.receivedStars += 1;

        const embed = new Discord.EmbedBuilder()
            .setTitle(`⭐ NEW GOLDEN STAR ⭐`)
            .setDescription(`<@${member.id}> received a golden star from <@${author.id}> for a total of ${member_config.stars.numberOfStars} stars\nReason: ${reason}`);

        await Promise.all([
            interaction.reply({embeds: [embed]}),
            member_config.save(),
            author_config.save()
        ])
    },
}
