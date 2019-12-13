pub struct Client {
    pub(crate) username: String,
    pub(crate) password: String,
}

impl Client {
    pub fn new(username: &str, password: &str) -> Client {
        Client {
            username: username.to_owned(),
            password: password.to_owned(),
        }
    }
}
