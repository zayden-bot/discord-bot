const fs = require('fs');
const fileName = "botConfig.json";
const file = require("../../botConfig.json");

module.exports = {
    commands: "status",
    expectedArgs: "<status>",
    callback: (message, arguments, text) => {

      if (!text) {
        message.reply(`Current Status: Playing ${text}`)
        return
      }

        message.client.user.setPresence({
            activity: {
                name: text,
                type: 0,
            },
        })
            
        file.customStatus = text;
            
        fs.writeFile(fileName, JSON.stringify(file, null, 4), function writeJSON(err) {
          if (err) return console.log(err);
        //   console.log(JSON.stringify(file));
          console.log('Writing to ' + fileName);
        });

        message.reply(`Status Changed to: Playing ${text}`)
    },
    permissions: [],
    requiredRoles: ["Security"],
}