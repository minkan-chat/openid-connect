use jose::{self, JsonWebKey};

pub struct OpenIDClient {
    jwks: Vec<JsonWebKey>,
}

impl OpenIDClient {
    pub fn keys(&self) -> impl Iterator<Item = &JsonWebKey> {
        self.jwks.iter()
    }
}
