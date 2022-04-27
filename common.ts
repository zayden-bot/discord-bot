module.exports = {
    parseId: function (id: string): string | undefined {
        const match = id.match(/\d+/)
        if (match) {
            return match[0];
        }
    }
}