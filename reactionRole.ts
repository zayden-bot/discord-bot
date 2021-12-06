// const sql = require("./sql");
import Discord from "discord.js"

export let reactionRole: ReactionRole;

export class ReactionRole {
    channel: Discord.TextChannel;
    message: Discord.Message;
    role: Discord.Role;
    emoji: string;

    constructor(channel: Discord.TextChannel, message: Discord.Message, role: Discord.Role, emoji: string) {
        this.channel = channel;
        this.message = message;
        this.role = role;
        this.emoji = emoji;
    }
}
