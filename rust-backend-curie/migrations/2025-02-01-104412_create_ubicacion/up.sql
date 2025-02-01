-- Your SQL goes here
CREATE TABLE `ubicacion` (
    `idUbicacion` int NOT NULL AUTO_INCREMENT,
    `nombre` varchar(255) COLLATE utf8mb4_general_ci DEFAULT NULL,
    `idLocalizacionFK` int NOT NULL,
    PRIMARY KEY (`idUbicacion`),
    KEY `fkLocalizacion` (`idLocalizacionFK`) USING BTREE,
    CONSTRAINT `fk_localizacion` FOREIGN KEY (`idLocalizacionFK`) REFERENCES `localizacion` (`idLocalizacion`)
) ENGINE = InnoDB AUTO_INCREMENT = 82 DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci