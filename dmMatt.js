module.exports = (client) => {
    client.on("message", (message) => {
        if (message.author.id == "530767446471344147" && message.content.includes("hehe")) {
            message.author.send("The Oscars have spanked you. You have been warned")
            message.author.send("https://media1.tenor.com/images/9c2d44a4fd540e5641f8a7e104c1259b/tenor.gif?itemid=9528172")
        }
    })
}