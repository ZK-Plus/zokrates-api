use rocket::http::Status;
use rocket::request::Request;
use rocket::serde::{json::Json, Serialize};
use rocket::{response, response::Responder};
use rocket_okapi::{
    gen::OpenApiGenerator,
    okapi::openapi3::{RefOr, Response as OpenApiReponse, Responses},
    response::OpenApiResponderInner,
    JsonSchema,
};
use schemars::Map;

pub type ApiResult<T> = Result<Json<T>, ApiError>;

#[derive(Debug, PartialEq, Eq, JsonSchema)]
pub enum ApiError {
    ResourceAlreadyExists(String),
    ResourceNotFound(String),
    CompilationError(String),
    InternalError(String),
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct ErrorResponse {
    pub error_message: String,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ApiError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        // log `self` to your favored error tracker, e.g.
        // sentry::capture_error(&self);
        let mut res = Json(ErrorResponse {
            error_message: format!("{:?}", self),
        })
        .respond_to(req)?;

        match self {
            ApiError::CompilationError(_) => res.set_status(Status::BadRequest),
            ApiError::ResourceNotFound(_) => res.set_status(Status::NotFound),
            ApiError::ResourceAlreadyExists(_) => res.set_status(Status::Conflict),
            _ => res.set_status(Status::InternalServerError),
        };
        Ok(res)
    }
}

impl OpenApiResponderInner for ApiError {
    fn responses(
        _generator: &mut OpenApiGenerator,
    ) -> Result<Responses, rocket_okapi::OpenApiError> {
        let mut responses = Map::new();
        responses.insert(
            "400".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "\
                [400 Bad Request](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/400)\n\
                The server cannot or will not process the request due to something that is perceived to be a client \
                error (e.g., malformed request syntax, invalid request message framing, or deceptive request routing).\
                "
                .to_string(),
                ..Default::default()
            }),
        );
        responses.insert(
            "403".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "\
                [403 Forbidden](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/403)\n\
                The client does not have access rights to the content; that is, it is unauthorized, \
                so the server is refusing to give the requested resource. Unlike `401` Unauthorized, \
                the client's identity is known to the server.\
                "
                .to_string(),
                ..Default::default()
            }),
        );
        responses.insert(
            "404".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "\
                [404 Not Found](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/404)\n\
                The server can not find the requested resource. In the browser, this means the URL is not recognized. \
                In an API, this can also mean that the endpoint is valid but the resource itself does not exist. \
                Servers may also send this response instead of `403` Forbidden to \
                hide the existence of a resource from an unauthorized client. \
                This response code is probably the most well known due to its frequent occurrence on the web.\
                "
                .to_string(),
                ..Default::default()
            }),
        );
        responses.insert(
            "422".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "\
                [422 Unprocessable Entity](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/422)\n\
                The request was well-formed but was unable to be followed due to semantic errors.\
                ".to_string(),
                ..Default::default()
            }),
        );
        responses.insert(
            "500".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "\
                [500 Internal Server Error](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/500)\n\
                This response is given when something went wrong on the server.\
                ".to_string(),
                ..Default::default()
            }),
        );
        Ok(Responses {
            responses,
            ..Default::default()
        })
    }
}
