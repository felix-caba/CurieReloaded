# Proyecto Final de Desarrollo Frontend - IES ZAIDÍN VERGELES

Este proyecto representa el desarrollo final de la asignatura de Desarrollo Frontend en el IES ZAIDÍN VERGELES. 
La aplicación consta de un backend en Rust y un frontend en React Native, integrando diversas funcionalidades avanzadas para una experiencia fluida y segura.
He hecho este Readme con ayuda de ChatGPT.

## Tecnologías Utilizadas

### Backend (Rust)
- Implementación del servicio en Rust con soporte para modelos personalizados con genéricos.
- Flujo de autenticación basado en JWT.
- Middleware para el manejo de errores.
- API REST optimizada para la gestión de datos.
- Sistema robusto de manejo de errores.


### Frontend (React Native)
- Conexión con el servicio en Rust para autenticación y gestión de datos.
- Sistema de login y registro.
- Implementación de RTK Query con gestión de caché y invalidación de tags.
- Navegación estructurada y adaptativa.
- Onboarding personalizado para la selección de productos en la pantalla inicial.
- Sistema de notificaciones tipo "toast" para una mejor experiencia de usuario.
- Vista de productos en lista enorme con implementacion de búsqueda y selección de detalles.
- Uso de Redux para manejar el contexto y API (RTK Query)
- Middleware de errores en RTK Query


## Seguridad y Almacenamiento
- Uso de **Keychain** para almacenar el token JWT de forma segura.
- Almacenamiento de datos locales mediante **MMKV**, una solución rápida y eficiente.

## Versión en Kotlin
Adicionalmente, se ha desarrollado una versión en **Kotlin** que implementa:
- Sistema de login.
- Integración con WebSockets.
- Visualización de gráficos interactivos.

Esta versión está disponible en el enlace de **Google Drive** adjunto al archivo o tarea en **Moodle**.

Implemento lo siguiente:

## Opciones en Kotlin

Marcaré con Sí? las opciones que se hayan visto en clase pero no las tenga la actual app de Kotlin.

- Definición de componentes: Sí
- Navegación entre pantallas: Sí?
- Uso de Retrofit: Sí.
- Biblioteca Externa: Sí?
- Representación de Gráficas sin B.Ext: Sí.
- Comunicación con WebSocket: A medias, clase creada pero sin implementación.
- Uso de Vertical Pager: No.

Si no estás de acuerdo con algo, házmelo saber y aportaré el código faltante, ya que la App de Kotlin estaba medio borrada, 
resultado de ir cambiando de Biblioteca Ext a Impl sin Ext, y hacerlo todo en la misma App por no llenar de basurilla el disco.

---

### Instalación y Uso
#### 1. Clonar el repositorio
```sh
  git clone <URL_DEL_REPOSITORIO>
  cd <NOMBRE_DEL_PROYECTO>
```

#### 2. Instalar dependencias
Para el backend en Rust:
```sh
  cargo build
```
Para el frontend en React Native:
```sh
  npm install
```

#### 3. Ejecutar la aplicación
Backend:
```sh
  cargo run
```
Frontend:
```sh
  npm start
```

---

### Autor
Proyecto desarrollado como parte del **Desarrollo Frontend en IES ZAIDÍN VERGELES**.

