import Discord from "discord.js";
import {getUserConfig, IUserConfig} from "../../models/user-config";

module.exports = {
    commands: ["give_pack", "givepack"],
    expectedArgs: "<user>",
    minArgs: 1,
    callback: async (message: Discord.Message) => {
        let member = message.mentions.members?.first();
        if (!member) {
            message.channel.send("Error: Please mention a member.");
            return;
        }

        const member_config: IUserConfig = await getUserConfig(member.id)

        member_config.tradingCards.unopenedCardPacks += 1;

        message.channel.send(`<@${member.id}> has received a card pack`)
        await member_config.save()
    },
    requiredRoles: ["Staff"]
}
