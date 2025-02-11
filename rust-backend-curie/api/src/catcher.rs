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
pub fn unauthorized() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        error: ErrorDetails {
            code: 401,
            reason: "Unauthorized".to_string(),
            description: "Authentication required".to_string(),
        },
    })
}

#[catch(403)]
pub fn forbidden() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        error: ErrorDetails {
            code: 403,
            reason: "Forbidden".to_string(),
            description: "Access denied".to_string(),
        },
    })
}

#[catch(404)]
pub fn not_found() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        error: ErrorDetails {
            code: 404,
            reason: "Not Found".to_string(),
            description: "Resource not found".to_string(),
        },
    })
}

#[catch(500)]
pub fn internal_server_error() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        error: ErrorDetails {
            code: 500,
            reason: "Internal Server Error".to_string(),
            description: "An internal server error occurred".to_string(),
        },
    })
}
