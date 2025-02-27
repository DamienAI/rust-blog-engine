use actix_web::{error, HttpRequest, HttpResponse};

/// handle json deconding errors and return the details about the error in the http response.
pub fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
  let detail = err.to_string();
  let resp = match &err {
    error::JsonPayloadError::ContentType => {
          HttpResponse::UnsupportedMediaType().body(detail)
      }
      error::JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
          HttpResponse::UnprocessableEntity().body(detail)
      }
      _ => HttpResponse::BadRequest().body(detail),
  };
  error::InternalError::from_response(err, resp).into()
}

/// Proccess form decoding errors and return the details in the http response.
pub fn form_error_handler(err: error::UrlencodedError, _req: &HttpRequest) -> error::Error {
  let detail = err.to_string();
  let resp = match &err {
    error::UrlencodedError::ContentType => {
          HttpResponse::UnsupportedMediaType().body(detail)
      }
      error::UrlencodedError::Parse => {
          HttpResponse::UnprocessableEntity().body(detail)
      }
      _ => HttpResponse::BadRequest().body(detail),
  };
  error::InternalError::from_response(err, resp).into()
}
