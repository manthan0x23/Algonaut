use crate::utils::web::errors::AppError;
use validator::Validate;

pub fn validate_or_bad_request<T: Validate>(input: &T) -> Result<(), AppError> {
    if let Err(errors) = input.validate() {
        let msg: String = errors
            .field_errors()
            .into_iter()
            .flat_map(|(field, errs)| {
                errs.into_iter()
                    .filter_map(|e| e.message.to_owned())
                    .map(move |m| format!("{field}: {m}"))
            })
            .collect::<Vec<_>>()
            .join(" | ");

        Err(AppError::bad_request(&msg))
    } else {
        Ok(())
    }
}
