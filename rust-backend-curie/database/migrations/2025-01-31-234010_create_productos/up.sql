-- Your SQL goes here

CREATE TABLE `productos` (
    `idProducto` int NOT NULL AUTO_INCREMENT,
    `nombre` varchar(255) COLLATE utf8mb4_general_ci DEFAULT NULL,
    `idLocalizacion` int NOT NULL,
    `idUbicacion` int NOT NULL,
    `cantidad` int DEFAULT NULL,
    `stock_minimo` int DEFAULT NULL,
    PRIMARY KEY (`idProducto`),
    KEY `idLocalizacion` (`idLocalizacion`) USING BTREE,
    KEY `idUbicacion` (`idUbicacion`) USING BTREE,
    CONSTRAINT `productos_ibfk_1` FOREIGN KEY (`idLocalizacion`) REFERENCES `localizacion` (`idLocalizacion`),
    CONSTRAINT `productos_ibfk_2` FOREIGN KEY (`idUbicacion`) REFERENCES `ubicacion` (`idUbicacion`)
) ENGINE = InnoDB AUTO_INCREMENT = 550 DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci

