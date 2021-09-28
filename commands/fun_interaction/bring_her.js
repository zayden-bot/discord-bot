module.exports = {
    commands: ["bringhertome"],
    minArgs: 0,
    maxArgs: 0,
    callback: (message, arguments, text) => {
        if (message.member.id != "211486447369322506") { return; }

        message.channel.send("<@204737300159004672> your Master needs a foot massage!")
    }
}