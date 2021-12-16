import { ReactionRole } from "./commands/reaction roles/reactionRole"

export class Server {
    id: string;
    reactionRoles: ReactionRole[]
    disabledCommands: string[];
    roles: Record<string, string>;
    channels: Record<string, string>;
    gameVersions: Record<string, string>
    serverRules: Record<string, string>
    serverGuidelines: Array<Array<string>>
    idNumber: number;
    hidden: Record<string, Record<string, string>>
    moderation: Record<string, Record<string, string>>
    supportAnswers: Record<string, string>

    constructor(id: string) {
        this.id = id;
        this.reactionRoles = []
        this.disabledCommands = []
        this.roles = {}
        this.channels = {};
        this.idNumber = 0;
        this.gameVersions = {}
        this.serverRules = {}
        this.serverGuidelines = [];
        this.hidden = {}
        this.moderation = {}
        this.supportAnswers = {};

        servers[this.id] = this
    }
}

export let servers: Record<string, Server> = {};