const Discord = require("discord.js");
const common = require("../../common");

module.exports = {
    commands: ["give_pack", "givepack"],
    expectedArgs: "<user>",
    minArgs: 1,
    maxArgs: 1,
    callback: (message, arguments, text) => {
        let member = message.mentions.members.first();
        if (!member) {
            message.channel.send("Error: Please mention a member.");
            return;
        }

        common.user_config_setup(message);
        member_config = require(`../../user_configs/${member.id}`)

        try { member_config["unopen_card_packs"] += 1; }
        catch {member_config["unopen_card_packs"] = 1; }

        common.update_configs(message, member_config);

        message.channel.send(`<@${member.id}> has received a card pack`)
    },
    requiredRoles: ["Staff"]
}
