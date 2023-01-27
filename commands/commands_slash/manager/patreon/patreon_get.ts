import Discord from "discord.js";
import http from "http";
import {IPatreonMember} from "../../../../models/IPatreonMember"

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("patreon_get")
        .setDescription("View Patreon information")
        .setDefaultMemberPermissions(Discord.PermissionFlagsBits.MoveMembers)
        .addStringOption(x =>
            x.setName("email")
                .setDescription("Enter email to view patreon status")
                .setRequired(true)),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        let isServerOnline = true
        http.get("http://ec2-3-8-185-82.eu-west-2.compute.amazonaws.com/api/v1/server/status", (res) => {
            if (res.statusCode != 200)
                isServerOnline = false
        })

        if (!isServerOnline) {
            return interaction.reply({content: "Server is offline", ephemeral: true})
        }

        const email = encodeURIComponent(interaction.options.getString("email", true))

        http.get(`http://ec2-3-8-185-82.eu-west-2.compute.amazonaws.com/api/v1/patreon/users/${email}`, async (res) => {
            if (res.statusCode != 200) {
                return interaction.reply("Email not found. Api is updated once a day at 00:00 UTC");
            }

            let data = "";
            res.on("data", (chunk) => data += chunk);

            res.on("end", async () => {
                const patreonMember: IPatreonMember = JSON.parse(data)

                const embed = new Discord.EmbedBuilder()
                    .setTitle("Patreon Status")
                    .setDescription(`Lifetime Support (USD): **${patreonMember.attributes.campaign_lifetime_support_cents / 100}**\nEmail: ${patreonMember.attributes.email}\nPatreon Status: **${patreonMember.attributes.patron_status}**`);

                await interaction.reply({embeds: [embed]});
            })

        })
    }
}
