const common = require("../../common")
const music = require("../../musicFunctions")
const serverConfig = require("../../serverConfigs/745662812335898806.json")

function shuffle(array) {
    array = array.slice(1, array.length)
    var currentIndex = array.length, temporaryValue, randomIndex;
  
    // While there remain elements to shuffle...
    while (0 !== currentIndex) {
  
      // Pick a remaining element...
      randomIndex = Math.floor(Math.random() * currentIndex);
      currentIndex -= 1;
  
      // And swap it with the current element.
      temporaryValue = array[currentIndex];
      array[currentIndex] = array[randomIndex];
      array[randomIndex] = temporaryValue;
    }
    console.log(array)
    return array;
}

module.exports = {
    commands: ["shuffle", "shuf", "randomize", "randomise"],
    permissionError: "Command is currently in development. Limited to staff use only.",
    callback: (message, arguments, text) => {
        serverConfig.musicQueue = shuffle(serverConfig.musicQueue);
        common.writeToServerConfig("745662812335898806")
        message.reply("Shuffled Queue")
    },
    permissions: [],
    requiredRoles: [],
}