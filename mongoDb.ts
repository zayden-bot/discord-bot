import mongoose from "mongoose";
import {ChannelSchema, IChannel} from "./models/server_settings/ChannelSchema";
import {ModLogSchema} from "./models/server_settings/ModLogSchema";
import {QuestionSchema} from "./models/server_settings/QuestionSchema";
import {ReactionRoleSchema} from "./models/server_settings/ReactionRoleSchema";
import {MiscellaneousSchema} from "./models/server_settings/MiscellaneousSchema";
import {RulesSchema} from "./models/server_settings/RulesSchema";
import {SupportFAQSchema} from "./models/server_settings/SupportFAQSchema";
import {UserConfigSchema} from "./models/user-config";
import {
    FuckImageSchema,
    GoodMorningImageSchema,
    GoodNightImageSchema,
    HugImageSchema,
    KissImageSchema,
    SlapImageSchema
} from "./models/global/IImageSchema";
import {AutoUpdatingSchema} from "./models/server_settings/AutoUpdating";

export function connectionFactory(connectionId: string) {
    const conn = mongoose.createConnection(process.env.MONGODB_CONNECTION_STRING!.replace("{dbName}", connectionId))

    if (connectionId == "Zayden") {
        conn.model("UserConfig", UserConfigSchema)
        return conn
    }

    if (connectionId == "Global") {
        conn.model("FuckImages", FuckImageSchema)
        conn.model("GoodMorningImages", GoodMorningImageSchema)
        conn.model("GoodNightImages", GoodNightImageSchema)
        conn.model("HugImages", HugImageSchema)
        conn.model("KissImages", KissImageSchema)
        conn.model("SlapImages", SlapImageSchema)
        return conn
    }

    conn.model("AutoUpdating", AutoUpdatingSchema)
    conn.model("Channels", ChannelSchema)
    conn.model("Miscellaneous", MiscellaneousSchema)
    conn.model("ModLogs", ModLogSchema)
    conn.model("Questions", QuestionSchema)
    conn.model("ReactionRoles", ReactionRoleSchema)
    conn.model("Rules", RulesSchema)
    conn.model("SupportFAQ", SupportFAQSchema)

    return conn
}

async function saveAndCloseDbConnection(connection: mongoose.Connection) {
    async function saveChannels(connection: mongoose.Connection) {
        const tasks = []
        for (const channel of await connection.model<IChannel>("Channels").find()) {
            tasks.push(channel.save())
        }
        return tasks
    }

    const tasks = []
    tasks.push(saveChannels(connection))

    await Promise.all(tasks)
    await connection.close()
}

export async function saveAndCloseDbConnections() {


    const tasks = []
    for (const connection of mongoose.connections) {
        tasks.push(saveAndCloseDbConnection(connection))
    }
    await Promise.all(tasks)

    // async function saveUsers() {
    //     for (const user of await UserConfig.find()) {
    //         tasks.push(user.save())
    //     }
    // }
}
