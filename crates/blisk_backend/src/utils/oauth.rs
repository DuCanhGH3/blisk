use std::str::FromStr;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("this user either doesn't exist or has already been verified")]
    AlreadyVerified,
    #[error("this user is not valid")]
    Invalid,
    #[error("this error is not expected")]
    Unexpected,
}

pub struct AuthResponseType {
    code: bool,
    id_token: bool,
    token: bool,
}

impl AuthResponseType {
    pub fn new() -> Self {
        Self {
            code: false,
            id_token: false,
            token: false,
        }
    }
}

impl FromStr for AuthResponseType {
    type Err = AuthError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ret = AuthResponseType::new();

        for response_type in s.split(' ') {
            match response_type {
                "code" => ret.code = true,
                "id_token" => ret.id_token = true,
                "token" => ret.token = true,
                &_ => {}
            }
        }

        Ok(ret)
    }
}

pub struct AuthScope {
    pub openid: bool,
    pub profile: bool,
    pub email: bool,
}

impl AuthScope {
    fn new() -> Self {
        AuthScope {
            openid: false,
            profile: false,
            email: false,
        }
    }
}

impl FromStr for AuthScope {
    type Err = AuthError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ret = AuthScope::new();
        for scope in s.split(' ') {
            match scope {
                "openid" => ret.openid = true,
                "profile" => ret.profile = true,
                "email" => ret.email = true,
                &_ => {}
            }
        }
        Ok(ret)
    }
}

pub struct AuthPrompt {
    pub none: bool,
    pub login: bool,
    pub consent: bool,
}

impl AuthPrompt {
    fn new() -> Self {
        Self {
            none: false,
            login: false,
            consent: false,
        }
    }
}

impl FromStr for AuthPrompt {
    type Err = AuthError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ret = AuthPrompt::new();

        for prompt in s.split(' ') {
            match prompt {
                "none" => ret.none = true,
                "login" => ret.login = true,
                "consent" => ret.consent = true,
                &_ => {}
            }
        }

        Ok(ret)
    }
}
