use std::{borrow::Cow, collections::HashMap};

use ruma::UserId;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Debug, Default, Clone, Serialize, Deserialize, Validate, PartialEq, Eq)]
pub struct LoginInfo {
    #[validate(length(min = 1), custom = "validate_matrix_user_id")]
    #[serde(rename = "userId")]
    pub user_id: String,

    #[validate(length(min = 6, max = 255))]
    pub password: String,
}

fn validate_matrix_user_id(user_id: &str) -> Result<(), ValidationError> {
    UserId::parse(user_id).map_err(|e| ValidationError {
        code: Cow::from("kallisti::validator::bad_user_id"),
        message: Some(Cow::from(e.to_string())),
        params: HashMap::new(),
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn login_info() {
        let login_info = LoginInfo {
            user_id: "@user:example.com".to_string(),
            password: "password".to_string(),
        };
        assert!(login_info.validate().is_ok());
    }
}
