import Discord from "discord.js";
import {getUserConfig} from "../../../models/user-config";
import {getConnection} from "../../../servers";
import {IMiscellaneous} from "../../../models/server_settings/MiscellaneousSchema";

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

        const conn = getConnection(interaction.guild.id)
        const misc = (await conn.model<IMiscellaneous>("Miscellaneous").findOne())!
        const member_config = await getUserConfig(member.id)
        const author_config = await getUserConfig(author.id)

        if (author_config.stars.numberOfStars <= 0 && !author.roles.cache.hasAny(...misc.moderationRoles)) {
            return interaction.reply("Unable. You have no gold stars to give.");
        }

        if (!author.roles.cache.hasAny(...misc.moderationRoles)) {
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
