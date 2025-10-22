pub mod hash_util {

    use base64::prelude::*;

    pub fn authenticate_and_hash(auth_token: Option<String>) -> Option<String> {
        auth_token.map(|auth_token| BASE64_STANDARD_NO_PAD.encode(auth_token))
    }
}