const { MessageEmbed } = require("discord.js");
const { patreonChannel, logsChannel } = require("../../config.json");

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
                            .setTitle("New Patreon")
                            .setColor(`${newMember.guild.roles.cache.get(`${role}`).hexColor}`)
                            .setFooter(newMember.guild.name, newMember.guild.iconURL({ dynamic: true }))
                            .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566")
                            .addField("User", `<@${newMember.id}>`, true)
                            .addField("Amount", `$${patreonRoles[role]}`, true)
                            .setTimestamp();

                        client.channels.cache.get(patreonChannel).send(embed2);
                    }
                    embed1.setColor(`${newMember.guild.roles.cache.get(`${role}`).hexColor}`);
                    embed1.addField(`Information:`, `**❯ Role: ** ✅ ${oldMember.guild.roles.cache.get(role).name}`);
                }
            }

            client.channels.cache.get(logsChannel).send(embed1);
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
            client.channels.cache.get(logsChannel).send(embed);
        }
    }
}