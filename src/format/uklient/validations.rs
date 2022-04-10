use regex::Regex;
use validator::ValidationError;

pub fn validate_uklient_fallback_mc_version(version: &str) -> Result<(), ValidationError> {
    let re = Regex::new(r"^1(\\.\\d{1,2}){1,2}$").unwrap();

    if re.is_match(version) {
        return Err(ValidationError::new(
            "uklient_fallback_mc_version_does_not_pass_regex",
        ));
    }

    Ok(())
}
