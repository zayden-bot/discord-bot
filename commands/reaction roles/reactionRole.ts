export let reactionRole: ReactionRole;

export class ReactionRole {
    channelId: string;
    messageId: string;
    roleId: string;
    emoji: string;

    constructor(channelId: string, messageId: string, roleId: string, emoji: string) {
        this.channelId = channelId;
        this.messageId = messageId;
        this.roleId = roleId;
        this.emoji = emoji;
    }
}
