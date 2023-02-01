import mongoose from "mongoose";

interface IField {
    name: string
    value: string
}

interface IFooter {
    text?: string,
    icon_url?: string
}

interface IAuthor {
    name: string,
    url?: string,
    icon_url?: string
}

interface IThumbnail {
    url?: string
}

export interface IAutoUpdating {
    channelId: string
    messageId?: string
    title?: string
    fields?: IField[],
    footer?: IFooter,
    author?: IAuthor,
    thumbnail?: IThumbnail,
}

export const AutoUpdatingSchema = new mongoose.Schema<IAutoUpdating>({
    channelId: String,
    messageId: String,
    title: String,
    fields: [{name: String, value: String}],
    footer: {text: String, icon_url: String},
    thumbnail: {url: String},
})
