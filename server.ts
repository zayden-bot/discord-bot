import { ReactionRole } from "./reactionRole"

export class Server {
    id: string;
    reactionRoles: ReactionRole[]

    constructor(id: string) {
        this.id = id;
        this.reactionRoles = []

        servers[this.id] = this
    }
}

export let servers: Record<string, Server> = {};