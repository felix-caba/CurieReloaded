-- Your SQL goes here
CREATE TABLE `producto_riesgo` (
    `idProducto` int NOT NULL,
    `idRiesgo` int NOT NULL,
    PRIMARY KEY (`idProducto`, `idRiesgo`),
    KEY `riesgo_id` (`idRiesgo`),
    CONSTRAINT `FKw8phsg5voraao49xgxviit2w` FOREIGN KEY (`idProducto`) REFERENCES `reactivos` (`idProducto`),
    CONSTRAINT `producto_riesgo_ibfk_1` FOREIGN KEY (`idProducto`) REFERENCES `productos` (`idProducto`),
    CONSTRAINT `producto_riesgo_ibfk_2` FOREIGN KEY (`idRiesgo`) REFERENCES `riesgos` (`idRiesgo`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci