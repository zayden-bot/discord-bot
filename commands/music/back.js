const music = require("../../musicFunctions")

module.exports = {
    commands: ["back", "b", "previous", "prev"],
    maxArgs: 0,
    callback: (message, arguments, text) => {
        music.back(message)
    },
}