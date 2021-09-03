module.exports = {
    commands: ["bringhimtome"],
    minArgs: 0,
    maxArgs: 0,
    callback: (message, arguments, text) => {
        if (message.member.id != "204737300159004672") { return; }

        message.channel.send("<@211486447369322506>, I want attention!")
    }
}