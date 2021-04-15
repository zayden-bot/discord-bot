const fs = require("fs")

module.exports = {
    writeToServerConfig: function (serverId) {
        serverConfig = require(`./serverConfigs/${serverId}.json`)
        fs.writeFile(`./serverConfigs/${serverId}.json`, JSON.stringify(serverConfig, null, 4), function writeJSON(err) {
            if (err) { return console.log(err); }
        });
    }
}