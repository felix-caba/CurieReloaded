-- Your SQL goes here
CREATE TABLE `riesgos` (
    `idRiesgo` int NOT NULL AUTO_INCREMENT,
    `descripcion` varchar(255) COLLATE utf8mb4_general_ci NOT NULL,
    `imagen` longblob,
    PRIMARY KEY (`idRiesgo`)
) ENGINE = InnoDB AUTO_INCREMENT = 11 DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci