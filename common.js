const fs = require("fs")

module.exports = {
    user_config_setup: function (message) {
        const default_config = {
            "number_of_stars": 0,
            "given_stars": 0,
            "received_stars": 0,
            "unopen_card_packs": 0,
            "openned_card_packs": 0,
        }

        const member = message.mentions.members.first();      
        const author = message.member;
        let member_config = `./user_configs/${member.id}.json`
        let author_config = `./user_configs/${author.id}.json`

        if (!fs.existsSync(member_config)) {
            console.log("Writing member config")
            fs.writeFileSync(member_config, JSON.stringify(default_config, null, 4), function writeJSON(err) {
                if (err) { return console.log(err); }
            });
        }

        if (!fs.existsSync(author_config)) {
            console.log("Writing author config")
            fs.writeFileSync(author_config, JSON.stringify(default_config, null, 4), function writeJSON(err) {
                if (err) { return console.log(err); }
            });
        }

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