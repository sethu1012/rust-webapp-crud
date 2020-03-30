-- Your SQL goes here

CREATE TABLE `users` (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    `name` VARCHAR(85) NOT NULL,
    email VARCHAR(85) NOT NULL UNIQUE,
    `password` TEXT NOT NULL
);