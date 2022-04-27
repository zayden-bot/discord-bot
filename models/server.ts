import mongoose from "mongoose";

export interface IReactionRole {
    channelId: string,
    messageId: string,
    roleId: string,
    emoji: string
}

export interface IQuestion {
    text: string,
    userId: string,
    questionId: number,
    messageId: string | null,
    answer: { text: string; username: string; } | null
}


interface IModeration {
    caseNumber: number,
    guildId: string,
    userId: string,
    type: string,
    moderatorId: string,
    reason: string
}


export interface IServer {
    id: string,
    reactionRoles: IReactionRole[],
    disabledCommands: string[],
    roles: {
        moderationRole: string,
        supportRole: string
    },
    channels: {
        suggestionChannel: string,
        logsChannel: string,
        patreonChannel: string,
        questionChannel: string,
        supportChannel: string
    },
    questions: IQuestion[],
    supportThreadId: number
    gameVersions: {
        patreonVersion: string,
        steamVersion: string,
        patreonUpdate: string,
        steamUpdate: string
    },
    serverRules: string[],
    hidden: {
        rules: Record<string, string>
    },
    moderation: IModeration[]
    supportAnswers: Record<string, string>,

    save(): Promise<IServer>;
}


const ServerSchema = new mongoose.Schema({
    id: String,
    reactionRoles: [{
        channelId: String,
        messageId: String,
        roleId: String,
        emoji: String
    }],
    disabledCommands: [String],
    roles: {
        staffRole: String,
        moderationRole: String,
        supportRole: String
    },
    channels: {
        suggestionChannel: String,
        logsChannel: String,
        patreonChannel: String,
        questionChannel: String,
        supportChannel: String
    },
    questions: [{
        text: String,
        userId: String,
        questionId: Number,
        messageId: String,
        answer: {
            text: String,
            userId: String,
        }
    }],
    supportThreadId: Number,
    gameVersions: {
        patreonVersion: String,
        steamVersion: String,
        patreonUpdate: String,
        steamUpdate: String
    },
    serverRules: [String],
    hidden: {
        rules: {type: Map, of: String}
    },
    moderation: [{
        caseNumber: Number,
        guildId: String,
        userId: String,
        type: String,
        moderatorId: String,
        reason: String
    }],
    supportAnswers: {type: Map, of: String},
})

export const Server = mongoose.model("Server", ServerSchema)