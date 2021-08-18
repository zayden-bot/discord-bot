const fs = require("fs")

module.exports = {
    user_config_setup: function (message) {
        const default_config = {
            "number_of_stars": 0,
            "given_stars": 0,
            "received_stars": 0
        }

        const member = message.mentions.members.first();      
        const author = message.member;
        let member_config = `./user_configs/${member.id}`
        let author_config = `./user_configs/${author.id}`

        if (!fs.existsSync(`${member_config}.json`)) {
            console.log("Writing member config")
            fs.writeFileSync(`${member_config}.json`, JSON.stringify(default_config, null, 4), function writeJSON(err) {
                if (err) { return console.log(err); }
            });
        }
        member_config = require(member_config)

        if (!fs.existsSync(`${author_config}.json`)) {
            console.log("Writing author config")
            fs.writeFileSync(`${author_config}.json`, JSON.stringify(default_config, null, 4), function writeJSON(err) {
                if (err) { return console.log(err); }
            });
        }
        author_config = require(author_config)
    },

    update_user_configs: function (message) {
        const member = message.mentions.members.first();      
        const author = message.member;
        let member_config = `./user_configs/${member.id}`
        let author_config = `./user_configs/${author.id}`

        fs.writeFile(`${member_config}.json`, JSON.stringify(member_config, null, 4), function writeJSON(err) {
            if (err) { return console.log(err); }
        });
        fs.writeFile(`${author_config}.json`, JSON.stringify(author_config, null, 4), function writeJSON(err) {
            if (err) { return console.log(err); }
        });
    }
}