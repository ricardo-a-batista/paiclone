CREATE TABLE IF NOT EXISTS users (
   id INTEGER PRIMARY KEY AUTOINCREMENT,
   username VARCHAR(20),
   password VARCHAR(20),
   ADMIN BOOL
);
