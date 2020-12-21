const sqlite3 = require("sqlite3").verbose(); // npm install sqlite3
const config = require("../../../config.json")

module.exports = {
    commands: ["listquestion", "listq"],
    callback: (message, arguments, text) => {
        let db = new sqlite3.Database(config.dbFile);
        let sql = `SELECT * FROM quiz`;
        db.all(sql, [], (err, rows) => {
            if (err) {
                console.log(err);
            }
            rows.forEach((row) => {
                console.log(row.name);
            });
        });

        // close the database connection
        db.close();
    },
    // requiredRoles: "Security",
}