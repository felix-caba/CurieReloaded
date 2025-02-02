// @generated automatically by Diesel CLI.

diesel::table! {
    localizacion (idLocalizacion) {
        idLocalizacion -> Integer,
        #[max_length = 255]
        nombre -> Nullable<Varchar>,
    }
}

diesel::table! {
    materiales (idProducto) {
        idProducto -> Integer,
        #[max_length = 255]
        subcategoria -> Nullable<Varchar>,
        #[max_length = 255]
        numero_serie -> Nullable<Varchar>,
        #[max_length = 255]
        descripcion -> Nullable<Varchar>,
        fecha_compra -> Nullable<Date>,
        fechaCaducidad -> Nullable<Date>,
    }
}

diesel::table! {
    password_reset_token (id) {
        id -> Integer,
        #[max_length = 255]
        token -> Varchar,
        user_id -> Integer,
        expiry_date -> Timestamp,
        expiryDate -> Nullable<Datetime>,
    }
}

diesel::table! {
    producto_riesgo (idProducto, idRiesgo) {
        idProducto -> Integer,
        idRiesgo -> Integer,
    }
}

diesel::table! {
    productos (idProducto) {
        idProducto -> Integer,
        #[max_length = 255]
        nombre -> Nullable<Varchar>,
        idLocalizacion -> Integer,
        idUbicacion -> Integer,
        cantidad -> Nullable<Integer>,
        stock_minimo -> Nullable<Integer>,
    }
}

diesel::table! {
    productos_auxiliares (idProducto) {
        idProducto -> Integer,
        #[max_length = 255]
        formato -> Nullable<Varchar>,
    }
}

diesel::table! {
    reactivos (idProducto) {
        idProducto -> Integer,
        #[max_length = 255]
        formato -> Nullable<Varchar>,
        #[max_length = 255]
        gradoPureza -> Nullable<Varchar>,
        fechaCaducidad -> Nullable<Date>,
    }
}

diesel::table! {
    riesgos (idRiesgo) {
        idRiesgo -> Integer,
        #[max_length = 255]
        descripcion -> Varchar,
        imagen -> Nullable<Longblob>,
    }
}

diesel::table! {
    ubicacion (idUbicacion) {
        idUbicacion -> Integer,
        #[max_length = 255]
        nombre -> Nullable<Varchar>,
        idLocalizacionFK -> Integer,
    }
}

diesel::table! {
    usuarios (id) {
        id -> Integer,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        #[max_length = 255]
        password -> Nullable<Varchar>,
        #[max_length = 255]
        username -> Nullable<Varchar>,
        admin -> Nullable<Integer>,
        pfp -> Nullable<Longblob>,
    }
}

diesel::joinable!(materiales -> productos (idProducto));
diesel::joinable!(password_reset_token -> usuarios (user_id));
diesel::joinable!(producto_riesgo -> productos (idProducto));
diesel::joinable!(producto_riesgo -> reactivos (idProducto));
diesel::joinable!(producto_riesgo -> riesgos (idRiesgo));
diesel::joinable!(productos -> localizacion (idLocalizacion));
diesel::joinable!(productos -> ubicacion (idUbicacion));
diesel::joinable!(productos_auxiliares -> productos (idProducto));
diesel::joinable!(reactivos -> productos (idProducto));
diesel::joinable!(ubicacion -> localizacion (idLocalizacionFK));

diesel::allow_tables_to_appear_in_same_query!(
    localizacion,
    materiales,
    password_reset_token,
    producto_riesgo,
    productos,
    productos_auxiliares,
    reactivos,
    riesgos,
    ubicacion,
    usuarios,
);
