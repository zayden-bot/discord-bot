import { ReactionRole } from "./commands/reaction roles/reactionRole"

export class Server {
    id: string;
    reactionRoles: ReactionRole[]
    disabledCommands: string[];
    roles: {
        staffRole: string;
        moderationRole: string;
        supportRole: string;
    };
    channels: {
        supportChannels: string[]
    };
    gameVersions: {
        patreonUpdate: string;
        patreonVersion: string;
        steamUpdate: string;
        steamVersion: string;
    }
    serverRules: Record<string, string>
    serverGuidelines: Array<Array<string>>
    idNumber: number;
    hidden: {
        rules: Record<string, string>
    }
    moderation: Array<any>
    supportAnswers: Record<string, string>

    constructor(id: string) {
        this.id = id;
        this.reactionRoles = []
        this.disabledCommands = []
        this.roles = {
            staffRole: "",
            moderationRole: "",
            supportRole: ""
        }
        this.channels = {
            supportChannels: []
        };
        this.idNumber = 0;
        this.gameVersions = {
            patreonUpdate: "1st Jan",
            patreonVersion: "v0.0.0",
            steamUpdate: "1st Jan",
            steamVersion: "v0.0.0"
        }
        this.serverRules = {}
        this.serverGuidelines = [];
        this.hidden = {
            rules: {}
        }
        this.moderation = []
        this.supportAnswers = {}

        servers[this.id] = this
    }
}

export let servers: Record<string, Server> = {};