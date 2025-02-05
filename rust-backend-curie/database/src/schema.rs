// @generated automatically by Diesel CLI.

diesel::table! {
    localizacion (idLocalizacion) {
        idLocalizacion -> Integer,
        #[max_length = 255]
        nombre -> Nullable<Varchar>,
    }
}

diesel::table! {
    material (idProducto) {
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
    producto (idProducto) {
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
    auxiliar (idProducto) {
        idProducto -> Integer,
        #[max_length = 255]
        formato -> Nullable<Varchar>,
    }
}

diesel::table! {
    reactivo (idProducto) {
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
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        username -> Varchar,
        admin -> Nullable<Integer>,
        pfp -> Nullable<Longblob>,
    }
}

use diesel::define_sql_function;

diesel::joinable!(material -> producto (idProducto));
diesel::joinable!(password_reset_token -> usuarios (user_id));
diesel::joinable!(producto_riesgo -> producto (idProducto));
diesel::joinable!(producto_riesgo -> reactivo (idProducto));
diesel::joinable!(producto_riesgo -> riesgos (idRiesgo));
diesel::joinable!(producto -> localizacion (idLocalizacion));
diesel::joinable!(producto -> ubicacion (idUbicacion));
diesel::joinable!(auxiliar -> producto (idProducto));
diesel::joinable!(reactivo -> producto (idProducto));
diesel::joinable!(ubicacion -> localizacion (idLocalizacionFK));

define_sql_function!(fn last_insert_id() -> BigInt);



diesel::allow_tables_to_appear_in_same_query!(
    localizacion,
    material,
    password_reset_token,
    producto_riesgo,
    producto,
    auxiliar,
    reactivo,
    riesgos,
    ubicacion,
    usuarios,
);
