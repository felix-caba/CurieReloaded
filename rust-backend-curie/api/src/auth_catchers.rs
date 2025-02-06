use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: ErrorDetails,
}

#[derive(Serialize)]
pub struct ErrorDetails {
    pub code: u16,
    pub reason: String,
    pub description: String,
}

#[catch(401)]
pub fn unauthorized_user() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        error: ErrorDetails {
            code: 401,
            reason: "Unauthorized".to_string(),
            description: "Usuario o contraseña incorrectos.".to_string(),
        }
    })
}

#[catch(404)]
pub fn not_found_user() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        error: ErrorDetails {
            code: 404,
            reason: "Not Found".to_string(),
            description: "Usuario no encontrado.".to_string(),
        }
    })
}

#[catch(409)]
pub fn conflict() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        error: ErrorDetails {
            code: 409,
            reason: "Conflict".to_string(),
            description: "The request could not be completed due to a conflict with the current state of the resource.".to_string(),
        }
    })
}

#[catch(422)]
pub fn unprocessable_entity() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        error: ErrorDetails {
            code: 422,
            reason: "Unprocessable Entity".to_string(),
            description: "The request was well-formed but was unable to be followed due to semantic errors.".to_string(),
        }
    })
}

#[catch(500)]
pub fn internal_server_error() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        error: ErrorDetails {
            code: 500,
            reason: "Internal Server Error".to_string(),
            description: "The server encountered an unexpected condition that prevented it from fulfilling the request.".to_string(),
        }
    })
}











