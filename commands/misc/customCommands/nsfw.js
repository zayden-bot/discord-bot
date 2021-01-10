module.exports = {
    commands: ["nsfw"],
    callback: (message, arguments, text) => {
        message.channel.send("Please keep conversations and other non-nsfw content in <#747428952577933424> to a minimum, use <#787774961850646559> to converse about content a member posted here.")
    },
}