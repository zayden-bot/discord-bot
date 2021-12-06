const Discord = require("discord.js");
const common = require("../../common");

function open_card_pack(config) {
    config["unopen_card_packs"] -= 1;
    config["openned_card_packs"] += 1;
}

module.exports = {
    commands: ["open_pack", "openpack"],
    expectedArgs: "<user>",
    minArgs: 0,
    maxArgs: 0,
    callback: (message, arguments, text) => {
        const member = message.member;

        common.user_config_setup(message);
        member_config = require(`../../user_configs/${member.id}`)

        if (!member_config["unopen_card_packs"] || member_config["unopen_card_packs"] <= 0) {
            message.channel.send("Error: No unopened card packs.");
            return;
        }

        open_card_pack(member_config)
        common.update_configs(message, member_config);

        message.channel.send(`<@${member.id}> has opened a card pack. This is a test message while assets are being created.`)
    },
}
