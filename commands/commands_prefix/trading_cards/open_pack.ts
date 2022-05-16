import Discord from "discord.js";
import {getUserConfig, IUserConfig} from "../../../models/user-config";

module.exports = {
    commands: ["open_pack", "openpack"],
    expectedArgs: "<user>",
    minArgs: 0,
    maxArgs: 0,
    callback: async (message: Discord.Message) => {
        const member = message.member;
        if (!member) {
            return;
        }

        const member_config: IUserConfig = await getUserConfig(member.id)

        if (member_config.tradingCards.unopenedCardPacks == 0) {
            message.channel.send("Error: No unopened card packs.");
            return;
        }

        member_config.tradingCards.unopenedCardPacks -= 1;
        member_config.tradingCards.openedCardPacks += 1;

        message.channel.send(`${member} has opened a card pack. This is a test message while assets are being created.`)
    },
}
