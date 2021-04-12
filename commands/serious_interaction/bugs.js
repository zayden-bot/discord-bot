// Template Command
module.exports = {
    commands: ["bugs"],
    callback: (message, arguments, text) => {
        message.channel.send("Please report bugs via: <https://forms.clickup.com/f/4cqgb-379/UHNW2DK7DA66I4AG9X>\nYou can view the already reported bugs via: <https://share.clickup.com/b/h/6-52859322-2/8f7e5aff0c00177>\n\nAnd remember: **#BlameSteve!**")
    }
}