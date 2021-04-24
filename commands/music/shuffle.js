const { servers } = require("../../index")

function shuffle(array) {
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
    return array;
}

module.exports = {
    commands: ["shuffle", "shuf", "randomize", "randomise"],
    permissionError: "Command is currently in development. Limited to staff use only.",
    callback: (message, arguments, text) => {
        try {
            let queue = servers[message.guild.id].queue
            queue.currentQueue = shuffle(queue.currentQueue);
        } catch {
            message.reply("Cannot shuffle empty queue")
            return
        }

        message.reply("Shuffled Queue")
    },
    permissions: [],
    requiredRoles: [],
}