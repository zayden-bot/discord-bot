module.exports = {
    commands: ["bringhertome"],
    expectedArgs: "",
    minArgs: 0,
    maxArgs: 0,
    callback: (message, arguments, text) => {
        if (message.member.id != "211486447369322506") { return; }

        message.channel.send("<@204737300159004672> you're Master needs a foot massage!")
    }
}