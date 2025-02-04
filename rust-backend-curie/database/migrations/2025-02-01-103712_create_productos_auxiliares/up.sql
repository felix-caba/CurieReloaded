-- Your SQL goes here
CREATE TABLE `productos_auxiliares` (
    `idProducto` int NOT NULL,
    `formato` varchar(255) COLLATE utf8mb4_general_ci DEFAULT NULL,
    PRIMARY KEY (`idProducto`) USING BTREE,
    CONSTRAINT `productos_auxiliares_ibfk_1` FOREIGN KEY (`idProducto`) REFERENCES `productos` (`idProducto`) ON DELETE CASCADE
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci