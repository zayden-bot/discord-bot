export function parseId(id: string) {
    const match = id.match(/\d+/);
    if (match) {
        return match[0];
    }

}