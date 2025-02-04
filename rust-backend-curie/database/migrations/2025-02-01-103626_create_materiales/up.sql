CREATE TABLE `materiales` (
    `idProducto` int NOT NULL,
    `subcategoria` varchar(255) COLLATE utf8mb4_general_ci DEFAULT NULL,
    `numero_serie` varchar(255) COLLATE utf8mb4_general_ci DEFAULT NULL,
    `descripcion` varchar(255) COLLATE utf8mb4_general_ci DEFAULT NULL,
    `fecha_compra` date DEFAULT NULL,
    `fechaCaducidad` date DEFAULT NULL,
    PRIMARY KEY (`idProducto`),
    CONSTRAINT `materiales_ibfk_1` FOREIGN KEY (`idProducto`) REFERENCES `productos` (`idProducto`) ON DELETE CASCADE
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci