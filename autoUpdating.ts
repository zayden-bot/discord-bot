import {Zayden} from "./client";
import {getConnection} from "./servers";
import {IAutoUpdating} from "./models/server_settings/AutoUpdating";
import updateRules from "./autoUpdating/updateRules";
import {IRule} from "./models/server_settings/RulesSchema";

async function updateGuildMessages(client: Zayden, guildId: string) {
    const conn = getConnection(guildId)
    const updatingMessages = await conn.model<IAutoUpdating>("AutoUpdating").find()
    for (const updatingMessage of updatingMessages) {
        const channel = await client.channels.fetch(updatingMessage.channelId)
        if (!channel?.isTextBased())
            continue

        if (!updatingMessage.messageId) {
            await channel.send({embeds: [updatingMessage]})
            continue
        }

        const msg = await channel.messages.fetch(updatingMessage.messageId)
        await msg.edit({embeds: [updatingMessage]})
    }

    if (guildId == "745662812335898806") { // College Kings
        const rules = await conn.model<IRule>("Rules").find({isHidden: false})
        const channel = await client.channels.fetch("747430712617074718")
        await updateRules(rules, channel!, "788539168980336701")
    }
}

export default async function autoUpdating(client: Zayden) {
    const guilds = await client.guilds.fetch()
    const tasks = []
    for (const [guildId] of guilds) {
        tasks.push(updateGuildMessages(client, guildId))
    }

    await Promise.all(tasks)
}
