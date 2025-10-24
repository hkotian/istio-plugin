use base64::prelude::*;
use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Formatter};

// TODO use me later
// #[derive(PartialEq, Debug)]
// enum TokenKind {
//     Header,
//     Env,
// }

pub struct AuthToken<'a> {
    value: &'a str,
    // kind: TokenKind // TODO Will be used later
}

#[derive(Debug)]
pub struct InvalidArgumentError {
    desc: String,
}

impl InvalidArgumentError {
    fn new(desc: &str) -> InvalidArgumentError {
        InvalidArgumentError {
            desc: String::from(desc),
        }
    }
}

impl fmt::Display for InvalidArgumentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid argument error : {}", self.desc)
    }
}

impl Error for InvalidArgumentError {}

impl AuthToken<'_> {
    pub fn compute_token_hash<'a>(
        hdr_val: Option<&'a str>,
        url: Option<&'a str>,
    ) -> Result<String, InvalidArgumentError> {
        Self::parse_auth_token(hdr_val, url).map(|t| BASE64_STANDARD_NO_PAD.encode(t.value))
    }

    fn parse_auth_token<'a>(
        hdr_val: Option<&'a str>,
        url: Option<&'a str>,
    ) -> Result<AuthToken<'a>, InvalidArgumentError> {
        let env_id = url.map(AuthToken::get_env_from_path).flatten();

        match (env_id, hdr_val) {
            (Some(env), None) => {
                if AuthToken::is_valid_env_id(env) {
                    Ok(AuthToken { value: env })
                } else {
                    Err(InvalidArgumentError::new("Invalid envID provided"))
                }
            }
            (_, Some(auth_key)) => {
                if AuthToken::is_valid_auth_token(auth_key) {
                    Ok(AuthToken { value: auth_key })
                } else {
                    Err(InvalidArgumentError::new("Invalid auth token provided"))
                }
            }
            _ => Err(InvalidArgumentError::new("No token provided")),
        }
    }

    fn get_env_from_path(url: &str) -> Option<&str> {
        let parts: Vec<&str> = url.split('/').filter(|s| !s.is_empty()).collect();

        match parts.as_slice() {
            ["sdk", "evalx", env_id, ..] => Some(*env_id),
            ["sdk", "eval", env_id, ..] => Some(*env_id),
            ["sdk", "goals", env_id] => Some(*env_id),
            ["msdk", "evalx", env_id, ..] => Some(*env_id),
            ["msdk", "eval", env_id, ..] => Some(*env_id),
            ["edge", "eval", env_id] => Some(*env_id),
            ["goals", env_id] => Some(*env_id),
            _ => None,
        }
    }

    fn is_valid_env_id(r: &str) -> bool {
        bson::oid::ObjectId::parse_str(r).is_ok()
    }

    fn is_valid_auth_token(t: &str) -> bool {
        // TODO create a regex and try to get better matching
        t.starts_with("sdk-") || t.starts_with("mob-") || t.starts_with("api-")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::evalx_context("/sdk/evalx/my-env/contexts/context", Some("my-env"))]
    #[case::eval_context("/sdk/eval/my-env/contexts/context", Some("my-env"))]
    #[case::evalx_users("/sdk/evalx/my-env/users/user", Some("my-env"))]
    #[case::eval_users("/sdk/eval/my-env/users/user", Some("my-env"))]
    #[case::goals("/goals/my-env", Some("my-env"))]
    #[case::sdk_latest_all("/sdk/latest-all", None)]
    #[case::edge_eval_env("/edge/eval/envId", Some("envId"))]
    #[case::edge_eval_auth("/edge/eval", None)]
    fn test_get_env_from_path(#[case] url: &str, #[case] expected_val: Option<&str>) {
        assert_eq!(AuthToken::get_env_from_path(url), expected_val);
    }

    #[test]
    fn test_parse_token() {
        let url = Some("/sdk/evalx/my-env/contexts/context");
        let auth_token = None::<&str>;
        // let expected_kind = TokenKind::Env;
        let expected_value = "my-env";
        let expect_err = false;

        let result = AuthToken::parse_auth_token(auth_token, url);
        if expect_err {
            assert!(result.is_err());
        } else {
            let t = result.unwrap();
            // assert_eq!(t.kind, expected_kind);
            assert_eq!(t.value, expected_value);
        }
    }
}
