import mongoose from "mongoose";
import {ReactionRoleSchema} from "./reaction_role";
import {QuestionSchema} from "./question";
import {ModerationSchema} from "./moderation";

const ServerSchema = new mongoose.Schema({
    id: String,
    reactionRoles: [ReactionRoleSchema],
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
    questions: [QuestionSchema],
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
    moderation: [ModerationSchema],
    supportAnswers: {type: Map, of: String},
})

export const Server = mongoose.model("Server", ServerSchema)