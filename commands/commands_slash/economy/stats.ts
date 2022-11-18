import Discord from "discord.js"
import {getUserConfig, IUserConfig} from "../../../models/user-config";

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("stats")
        .setDescription("View your gold star stats")
        .addUserOption(options =>
            options.setName("member")
                .setDescription("View stats of other members")),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        const member = interaction.options.getMember("member") || interaction.member

        if (!(member instanceof Discord.GuildMember)) {
            return interaction.reply({content: "Invalid member mentioned", ephemeral: true})
        }

        const member_config: IUserConfig = await getUserConfig(member.id);

        const embed = new Discord.EmbedBuilder()
            .setTitle(`${member.displayName} Stats`)
            .addFields([
                {name: "Number of Stars", value: member_config.stars.numberOfStars.toString(), inline: true},
                {name: "Given Stars", value: member_config.stars.givenStars.toString(), inline: true},
                {name: "Received Stars", value: member_config.stars.receivedStars.toString(), inline: true}
            ])

        interaction.reply({embeds: [embed]}).then()
    }
}
