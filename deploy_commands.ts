import Discord from "discord.js";
import {Zayden} from "./client";

const commands: string[] = [];

export default async function deploy_commands(client: Zayden) {
    const rest = new Discord.REST({version: '10'}).setToken(process.env.TOKEN!);

    for (const command of client.slashCommands.values()) {
        commands.push(command.data.toJSON());
    }

    try {
        const data: any = await rest.put(
            Discord.Routes.applicationCommands(client.user!.id),
            {body: commands},
        );

        console.log(`Successfully reloaded ${data.length} application (/) commands.`);
    } catch (error) {
        console.error(error);
    }
}
