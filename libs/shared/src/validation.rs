use validator::{Validate, ValidationErrors};

/// Validates the request body and returns an `Err` with an appropriate `HttpResponse` if validation fails.
pub fn validate_request_body<T: Validate>(body: &T) -> Result<(), ValidationErrors> {
    body.validate()
}
