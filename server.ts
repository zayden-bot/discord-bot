import { ReactionRole } from "./reactionRole"

export class Server {
    id: string;
    reactionRoles: ReactionRole[]
    disabledCommands: string[] | undefined;
    moderationRoles: string[] | undefined;
    channels: Record<string, string> | undefined;
    idNumber: number | undefined;
    config: {[key: string]: any} | undefined


    constructor(id: string, config=undefined) {
        this.id = id;
        this.reactionRoles = []
        this.config = config

        servers[this.id] = this
    }
}

export let servers: Record<string, Server> = {};