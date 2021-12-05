const { MessageEmbed } = require("discord.js");
const { patreonChannel, logsChannel } = require("../../serverConfigs/745662812335898806.json");

const patreonRoles = {
    ['745663316776714370'] : 1, // Freshman
    ['745663351756947656'] : 5, // Sophomore
    ['745663375496708127'] : 10, // Junior
    ['745663394543304704'] : 20, // Senior
    ['745663409932206112'] : 50, // President
    ['745663432560345218'] : 100 // King
}

module.exports = {
    log: function (client, oldMember, newMember) {
        if (oldMember.roles.cache.size < newMember.roles.cache.size) {
            const embed1 = new MessageEmbed()
                .setTitle(`Member Role Update`)
                .setThumbnail('https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566')
                .setFooter(newMember.guild.name, newMember.guild.iconURL({ dynamic: true }))
                .addField("Update For", `<@${newMember.id}>`, false)
                .setTimestamp();

            for (const role of newMember.roles.cache.map(x => x.id)) {
                if (!oldMember.roles.cache.has(role)) {
                    if (patreonRoles[role]) {
                        const embed2 = new MessageEmbed()
                            .setTitle("New Patron")
                            .setColor(`${newMember.guild.roles.cache.get(`${role}`).hexColor}`)
                            .setFooter(newMember.guild.name, newMember.guild.iconURL({ dynamic: true }))
                            .setThumbnail("https://images-wixmp-ed30a86b8c4ca887773594c2.wixmp.com/f/8f5967b9-fc84-45f6-a9c3-3938bfba7232/dbujg26-4865d57d-8dcc-435c-ac6e-0d0590f9de37.png/v1/fill/w_1683,h_475,q_70,strp/patreon_logo_by_laprasking_dbujg26-pre.jpg?token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1cm46YXBwOiIsImlzcyI6InVybjphcHA6Iiwib2JqIjpbW3siaGVpZ2h0IjoiPD01NzYiLCJwYXRoIjoiXC9mXC84ZjU5NjdiOS1mYzg0LTQ1ZjYtYTljMy0zOTM4YmZiYTcyMzJcL2RidWpnMjYtNDg2NWQ1N2QtOGRjYy00MzVjLWFjNmUtMGQwNTkwZjlkZTM3LnBuZyIsIndpZHRoIjoiPD0yMDQxIn1dXSwiYXVkIjpbInVybjpzZXJ2aWNlOmltYWdlLm9wZXJhdGlvbnMiXX0.95jfkKc4e-WyhcxKoiDGebItWvxmMPadhqYsh7gIsnQ")
                            .addField("User", `<@${newMember.id}>`, true)
                            .addField("Amount", `$${patreonRoles[role]}`, true)
                            .setTimestamp();

                        client.channels.cache.get(patreonChannel).send({embeds: [embed2]});
                    }
                    embed1.setColor(`${newMember.guild.roles.cache.get(`${role}`).hexColor}`);
                    embed1.addField(`Information:`, `**❯ Role: ** ✅ ${oldMember.guild.roles.cache.get(role).name}`);
                }
            }

            client.channels.cache.get(logsChannel).send({embeds: [embed1]});
        } else if (oldMember.roles.cache.size > newMember.roles.cache.size) {
            const embed = new MessageEmbed()
                .setTitle(`Member Role Update`)
                .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566")
                .setFooter(newMember.guild.name, newMember.guild.iconURL({ dynamic: true }))
                .addField("Update For", `<@${newMember.id}>`, false)
                .setTimestamp();

            for (const role of oldMember.roles.cache.map(x => x.id)) {
                if (!newMember.roles.cache.has(role)) {
                    embed.setColor(`${newMember.guild.roles.cache.get(`${role}`).hexColor}`);
                    embed.addField(`Information:`, `**❯ Role: ** ⛔ ${newMember.guild.roles.cache.get(role).name}`);
                }
            }
            client.channels.cache.get(logsChannel).send({embeds: [embed]});
        }
    }
}
