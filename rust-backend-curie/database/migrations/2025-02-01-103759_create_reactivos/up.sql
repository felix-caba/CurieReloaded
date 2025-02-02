-- Your SQL goes here
CREATE TABLE `reactivos` (
    `idProducto` int NOT NULL,
    `formato` varchar(255) COLLATE utf8mb4_general_ci DEFAULT NULL,
    `gradoPureza` varchar(255) COLLATE utf8mb4_general_ci DEFAULT NULL,
    `fechaCaducidad` date DEFAULT NULL,
    PRIMARY KEY (`idProducto`),
    KEY `idProducto` (`idProducto`),
    CONSTRAINT `reactivos_ibfk_1` FOREIGN KEY (`idProducto`) REFERENCES `productos` (`idProducto`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci