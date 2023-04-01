use base64::{Engine as _, engine::{self, general_purpose}, alphabet};

const CUSTOM_ENGINE: engine::GeneralPurpose = 
    engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

#[derive(Debug, PartialEq)]
pub struct MayaClient {}

#[derive(Debug, Clone, PartialEq)]
pub struct MayaAuthToken {
    pub token: String,
}

impl MayaClient {

    pub fn new(client_token: &str, client_secret: &str) -> MayaAuthToken {
        
        let generated_token = Self::generate_token(client_token, client_secret);

        let client_token: String = format!("Basic {}", generated_token);

        MayaAuthToken {
            token: client_token,
        }
    }

    fn generate_token(client_secret: &str, client_token: &str) -> String {
        let mut token = String::new();
        let token_format = format!("{}:{}", client_token, client_secret);
        
        CUSTOM_ENGINE.encode_string(&token_format.as_bytes(), &mut token);
        
        return token;
    }
}

impl MayaAuthToken {
    pub fn new(token: &str) -> MayaAuthToken {
        MayaAuthToken {
            token: token.to_string(),
        }
    }

    pub fn token(&self) -> &str {
        &self.token
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_token() {
        let result = 
            MayaClient::new("client_token", "client_secret");
        assert_eq!(result.token.is_empty(), false);
    }
}
