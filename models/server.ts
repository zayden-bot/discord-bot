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
    messageId: string | undefined,
    answer: { text: string; username: string; } | undefined
}


export interface IModeration {
    caseNumber: number,
    guildId: string,
    userId: string,
    logType: string,
    moderatorId: string,
    reason: string
}


export interface IServer {
    channels: {
        logsChannel: string,
        patreonChannel: string,
        questionChannel: string,
        suggestionChannel: string,
        supportChannel: string,
        supportChannels: string[]
    },
    disabledCommands: string[],
    gameVersions: {
        patreonVersion: string,
        steamVersion: string,
        patreonUpdate: string,
        steamUpdate: string
    },
    hidden: {
        rules: Map<string, string>
    },
    id: string,
    moderation: IModeration[],
    questions: IQuestion[]
    reactionRoles: IReactionRole[],
    roles: {
        moderationRole: string,
        supportRole: string
    },
    serverRules: string[]
    supportAnswers: Map<string, string>,
    supportThreadId: number;

    save(): Promise<IServer>,
}

export async function getServer(id: string): Promise<IServer> {
    return await Server.findOne({id: id}).exec() || new Server({id: id})
}

const ServerSchema = new mongoose.Schema<IServer>({
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
        supportChannel: String,
        supportChannels: [String]
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
    supportThreadId: {type: Number, default: 0},
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
        logType: String,
        moderatorId: String,
        reason: String
    }],
    supportAnswers: {type: Map, of: String},
})

export const Server = mongoose.model("Server", ServerSchema)
