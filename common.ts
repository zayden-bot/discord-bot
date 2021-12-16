import Discord from "discord.js"
import fs from "fs"
import { Server } from "./server";

module.exports = {
    user_config_setup: function (message: Discord.Message) {
        const default_config = {
            "number_of_stars": 0,
            "given_stars": 0,
            "received_stars": 0,
            "unopen_card_packs": 0,
            "openned_card_packs": 0,
        }

        let member = message.mentions.members?.first();
        if (!member) { member = message.member as Discord.GuildMember }
        const author = message.member;
        let member_config = `./user_configs/${member.id}.json`
        let author_config = `./user_configs/${author?.id}.json`

        const fs = require("fs")
        if (!fs.existsSync(member_config)) {
            fs.writeFileSync(member_config, JSON.stringify(default_config, null, 4), function writeJSON(err: any) {
                if (err) { return console.log(err); }
            });
        }

        if (!fs.existsSync(author_config)) {
            fs.writeFileSync(author_config, JSON.stringify(default_config, null, 4), function writeJSON(err: any) {
                if (err) { return console.log(err); }
            });
        }

    },
    
    update_configs: function (message: Discord.Message, member_config=null, author_config=null, server_config=null) {
        let member = message.mentions.members?.first();
        if (!member) { member = message.member as Discord.GuildMember }
        const author = message.member;
        const server = message.guild;

        const fs = require("fs")
        if (member_config != null) {
            fs.writeFileSync(`./user_configs/${member.id}.json`, JSON.stringify(member_config, null, 4), function writeJSON(err: any) {
                if (err) { return console.log(err); }
            });
        }
        if (author_config != null) {
            fs.writeFileSync(`./user_configs/${author?.id}.json`, JSON.stringify(author_config, null, 4), function writeJSON(err: any) {
                if (err) { return console.log(err); }
            });
        }
        if (server_config != null) {
            fs.writeFileSync(`./server_configs/${server?.id}.json`, JSON.stringify(server_config, null, 4), function writeJSON(err: any) {
                if (err) { return console.log(err); }
            });
        }
    },

    parseId: function (id: string): string | undefined {
        const match = id.match(/\d+/)
        if (match) { return match[0]; }
    },

    updateConfig: function (guild: Discord.Guild, server: Server) {
        fs.writeFile(`./server_configs/${guild.id}.json`, JSON.stringify(server, null, 4), (error: any) => {
            if (error) { return console.log(error); }
        });
    },

    updateConfigSync: function (guild: Discord.Guild, server: Server) {
        fs.writeFileSync(`./server_configs/${guild.id}.json`, JSON.stringify(server, null, 4));
    },
}