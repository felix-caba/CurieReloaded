-- Your SQL goes here
CREATE TABLE `usuarios` (
    `id` int NOT NULL AUTO_INCREMENT,
    `email` varchar(255) COLLATE utf8mb4_general_ci DEFAULT NULL,
    `password` varchar(255) COLLATE utf8mb4_general_ci NOT NULL,
    `username` varchar(255) COLLATE utf8mb4_general_ci NOT NULL,
    `admin` int DEFAULT NULL,
    `pfp` longblob,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB AUTO_INCREMENT = 15 DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci