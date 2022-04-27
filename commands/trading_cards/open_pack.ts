import Discord from "discord.js";

function open_card_pack(config: any) {
    config["unopen_card_packs"] -= 1;
    config["openned_card_packs"] += 1;
}

module.exports = {
    commands: ["open_pack", "openpack"],
    expectedArgs: "<user>",
    minArgs: 0,
    maxArgs: 0,
    callback: (message: Discord.Message) => {
        const member = message.member;
        if (!member) {
            return;
        }

        const member_config = require(`../../user_configs/${member.id}`)

        if (!member_config["unopen_card_packs"] || member_config["unopen_card_packs"] <= 0) {
            message.channel.send("Error: No unopened card packs.");
            return;
        }

        open_card_pack(member_config)

        message.channel.send(`<@${member.id}> has opened a card pack. This is a test message while assets are being created.`)
    },
}
