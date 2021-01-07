module.exports = {
    commands: ["save", "saves"],
    callback: (message, arguments, text) => {
        message.channel.send(`With the major changes in v0.6 saves before this version will not work. We are sorry for the inconvenience. Use CTRL/TAB to quickly skip through the content you have seen`)
    },
}