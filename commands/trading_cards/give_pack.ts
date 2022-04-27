import Discord from "discord.js";

module.exports = {
    commands: ["give_pack", "givepack"],
    expectedArgs: "<user>",
    minArgs: 1,
    maxArgs: 1,
    callback: (message: Discord.Message) => {
        let member = message.mentions.members?.first();
        if (!member) {
            message.channel.send("Error: Please mention a member.");
            return;
        }

        const member_config = require(`../../user_configs/${member.id}`)

        try {
            member_config["unopen_card_packs"] += 1;
        } catch {
            member_config["unopen_card_packs"] = 1;
        }

        message.channel.send(`<@${member.id}> has received a card pack`)
    },
    requiredRoles: ["Staff"]
}
