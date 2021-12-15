export let reactionRole: ReactionRole;

export class ReactionRole {
    channelId: string;
    messageId: string;
    roleId: string;
    emojiId: string;

    constructor(channelId: string, messageId: string, roleId: string, emojiId: string) {
        this.channelId = channelId;
        this.messageId = messageId;
        this.roleId = roleId;
        this.emojiId = emojiId;
    }
}
