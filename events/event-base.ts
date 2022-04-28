module.exports = (client, eventOptions) => {
    let {
        event,
        callback
    } = eventOptions

    if (event) {
        console.log(`Registering event "${event}"`);
        client.on(`${event}`, (...args) => {
            callback(args)
            return
        })

    } else {
        console.log(`Tried to register event "${event}" but failed!`);
    }
}