import sqlite3 from "sqlite3"; // npm install sqlite3

let db: sqlite3.Database;

export function init() { // Create connection
    if (db) {
        console.log("There is already a database connection active!");
    } else {
        const config = require("./configs/bot_config.json")
        db = new sqlite3.Database(config.dbFile, sqlite3.OPEN_READWRITE, (err) => {
            if (err) { console.error(err.message); }
            console.log("Database connection was established.");
        });
    }
}

export function each(data: string, callback: (row: any) => void) {
        if (data) {
            if (callback) {
                if (db) {
                    db.serialize(function() {
                        db.each(String(data), function(err, row) {
                            if (err) {
                                console.error(err.message);
                            }
                            console.log("Executed query: " + String(data));
                            callback(row);
                        });
                    });
                } else {
                    console.log("There is no database connection active.");
                }
            } else {
                console.log("Please provide a callback function.");
            }
        } else {
            console.log("Please provide a query string to execute.");
        }
    }

export function run(data: string) { // Run a query
    if (data) {
        if (db) {
            db.serialize(function() {
                db.run(String(data), (err) => {
                    if (err) {
                        console.error(err.message);
                    }
                    console.log("Executed query: " + String(data));
                });
            });
        } else {
            console.log("There is no database connection active.");
        }
    } else {
        console.log("Please provide a query string to execute.");
    }
}

// export function each(data: string, callback: (row: any) => void) { // Used for SELECT query
//     if (data) {
//         if (callback) {
//             if (db) {
//                 db.serialize(function() {
//                     db.each(String(data), function(err, row) {
//                         if (err) {
//                             console.error(err.message);
//                         }
//                         console.log("Executed query: " + String(data));
//                         callback(row);
//                     });
//                 });
//             } else {
//                 console.log("There is no database connection active.");
//             }
//         } else {
//             console.log("Please provide a callback function.");
//         }
//     } else {
//         console.log("Please provide a query string to execute.");
//     }
// }

export function end() { // Destroy connection
    if (db) {
        db.close((err) => {
            if (err) {
                console.error(err.message);
            }
            console.log("Database connection was closed.");
        });
    } else {
        console.log("There is no database connection active.");
    }
}
    
export function getConnection() {
    if (db) {
        return db;
    } else {
        console.log("There is no database connection active.");
    }
}
