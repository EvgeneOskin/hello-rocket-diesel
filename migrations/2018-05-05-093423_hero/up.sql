-- Your SQL goes here
CREATE TABLE heroes (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  `name` VARCHAR(60) NOT NULL,
  identity VARCHAR(60) NOT NULL,
  hometown VARCHAR(60) NOT NULL,
  age INT(11) NOT NULL
)