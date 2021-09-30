const Discord = require("discord.js")

module.exports = {
    commands: ["giveaway"],
    callback: (message, arguments, text) => {

        message.channel.send("🔥 College Kings Act III Steam GIVEAWAY! 🔥\n\nIn the run up to the release of the next chapter of College Kings on Steam, we're giving away 50 copies for FREE!\n\nYou can enter the giveaway here: <https://giv.gg/wdElPj>\n\nGood luck! 💪").then(msg => {
            msg.delete({ timeout: 60000 });
        });
        message.delete();
        return;
    },
}