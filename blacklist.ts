let blacklistedUsers: number[] = [];


export function init() {
    const sql = require("./sql");
    sql.each("SELECT * FROM `blacklist`", (row: Record<string, string>) => {
        blacklistedUsers.push(Number(row.id));
    })
    console.log(`Loaded ${blacklistedUsers.length} blacklisted members!`);
}


export function isBlacklisted(id: string): boolean {
    return blacklistedUsers.includes(Number(id));

}


export function blacklist(id: string): boolean {
    if (blacklistedUsers.includes(Number(id))) { return false } // "User is already blacklisted"

    blacklistedUsers.push(Number(id));

    const sql = require("./sql");
    sql.run(`INSERT INTO 'blacklist' ('id') VALUES ('${Number(id)}');`)
    return true;
}


export function removeBlacklist(id: string): boolean{
    if (!blacklistedUsers.includes(Number(id))) { return false }

    const index = blacklistedUsers.indexOf(Number(id));
    if (index == -1) { return false }
    
    blacklistedUsers.splice(index, 1);

    const sql = require("./sql");
    sql.run(`DELETE FROM 'blacklist' WHERE id = '${Number(id)}';`);
    return true
}


export function isProtectedUser(id: string): boolean{
    const botConfig = require("./configs/bot_config.json");

    return !!botConfig.developers.includes(id);
}
