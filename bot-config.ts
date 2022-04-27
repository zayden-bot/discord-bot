import {BotConfig} from "./models/bot-config";

export async function init() {
    const botConfig = await BotConfig.findOne().exec() || new BotConfig()
    botConfig.save()
}