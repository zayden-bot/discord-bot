const sqlite3 = require("sqlite3").verbose(); // npm install sqlite3
const config = require("./configs/bot_config.json")
let db = false;

module.exports = {
    init: function () { // Create connection
        if (db) {
            console.log("There is already a database connection active!");
        } else {
            db = new sqlite3.Database(config.dbFile, sqlite3.OPEN_READWRITE, (err) => {
                if (err) {
                  console.error(err.message);
                }
                console.log("Database connection was established.");
            });
        }
    },

    run: function(data) { // Run a query
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
    },

    each: function(data, callback) { // Used for SELECT query
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
    },

    end: function () { // Destroy connection
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
    },
    
    getConnection: function() {
        if (db) {
            return db;
        } else {
            console.log("There is no database connection active.");
        }
    }
}
