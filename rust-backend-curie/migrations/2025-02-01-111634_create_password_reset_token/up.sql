-- Your SQL goes here
CREATE TABLE `password_reset_token` (
    `id` int NOT NULL AUTO_INCREMENT,
    `token` varchar(255) COLLATE utf8mb4_general_ci NOT NULL,
    `user_id` int NOT NULL,
    `expiry_date` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    `expiryDate` datetime(6) DEFAULT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `UK_token` (`token`),
    KEY `FK_user_id` (`user_id`),
    CONSTRAINT `FK_user_id` FOREIGN KEY (`user_id`) REFERENCES `usuarios` (`id`)
) ENGINE = InnoDB AUTO_INCREMENT = 11 DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci