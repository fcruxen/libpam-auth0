#[macro_use]
extern crate pamsm;
extern crate reqwest;
extern crate slog;
extern crate slog_term;
extern crate slog_async;

use std::{fs::OpenOptions};

use std::process::Command;

use serde_json::json;
use pamsm::{PamServiceModule, Pam, PamFlags, PamError, PamLibExt};
use slog::{o, Logger, Drain, info, error};
 
const AUTH0_TOKEN_URL: &str = "";
const AUTH0_CLIENT_ID: &str = "";

struct PamAuth0;

impl PamAuth0 {
    fn logger() -> Logger {
        let log_path = "/var/log/libpam-auth0";
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(log_path)
            .unwrap();

        let decorator = slog_term::PlainDecorator::new(file);
        let drain = slog_term::FullFormat::new(decorator).build().fuse();
        let drain = slog_async::Async::new(drain).build().fuse();
        Logger::root(drain, o!("libpam" => "auth0"))
    }

    fn request(user: &str, password: &str) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let client = reqwest::blocking::Client::new();
        client
            .post(AUTH0_TOKEN_URL)
            .header("Content-Type", "application/json")
            .json(&json!({
                "grant_type": "password",
                "username": user,
                "password": password,
                "client_id": AUTH0_CLIENT_ID
        })).send()
    }

    fn create_local_user(username: &str, logger: Logger) -> PamError {
        let path = "/etc/passwd";
        let file =std::fs::read_to_string(path);
        match file {
            Ok(content ) => {
                info!(logger, "libpam-auth0 checking if auth0 user {username} is present locally");
                if !content.contains(username) {
                    info!(logger, "libpam-auth0 creating {username} locally");                    
                    let command =  Command::new("/usr/sbin/useradd")
                        .arg("-m")
                        .arg("--shell")
                        .arg("/bin/bash")
                        .arg(username)
                        .output(); 
                    match command {
                        Ok(out) => {
                            let stdout = String::from_utf8(out.stdout).unwrap();
                            let stderr = String::from_utf8(out.stderr).unwrap();
                            info!(logger, "libpam-auth0 stdout from creating new user: {stdout}");
                            info!(logger, "libpam-auth0 stderr from creating new user: {stderr}");
                            info!(logger, "libpam-auth0 created {username} locally")
                        },
                        Err(err) => {
                            error!(logger, "libpam-auth0 failed to create {username} locally: {err}");
                            return PamError::AUTH_ERR
                        }
                    }
                }
                info!(logger, "libpam-auth0 logged {username} sucessfully");
                PamError::SUCCESS
            },
            Err(err) => {
                error!(logger, "libpam-auth0 failed opening {path}: {err}");
                PamError::AUTH_ERR
            },
        }
        
    }
}

impl PamServiceModule for PamAuth0 {
    fn open_session(_: Pam, _: PamFlags, _: Vec<String>) -> PamError {
        PamError::SUCCESS
    }

    fn close_session(_: Pam, _: PamFlags, _: Vec<String>) -> PamError {
        PamError::SUCCESS
    }

    fn authenticate(pamh: Pam, _: PamFlags, _args: Vec<String>) -> PamError {
        let logger = PamAuth0::logger();
        
        let user: Result<&str, std::str::Utf8Error> = match pamh.get_user(None) {
            Ok(Some(u)) => u.to_str(),
            Ok(None) => return PamError::USER_UNKNOWN,
            Err(e) => return e,
        };
        
        info!(logger, "libpam-auth0 received username: {name}", name = user.clone().unwrap());
        
        let password = match pamh.get_authtok(None) {
            Ok(pass) => pass,
            Err(e) => return e
        };
        let token = password.clone().unwrap().to_str().unwrap();
        let masked_token = "*".repeat(token.len());
        
        info!(logger, "libpam-auth0 received auth token: {masked_token}");
        
        let response = PamAuth0::request(
            user.clone().unwrap(), 
            &password.clone().unwrap().to_string_lossy()
        );
        
        match response {
            Ok(res) if res.status().is_success() => {
                PamAuth0::create_local_user(user.unwrap(), logger)                
            },
            Ok(res) => {
                let status = res.status().clone();
                error!(logger, "libpam-auth0 failed to authenticate with status code: {status}");
                match res.text() {
                    Ok(body) => error!(logger, "libpam-auth0 failed to authenticate with response body: {body}"),
                    Err(err) => error!(logger, "libpam-auth0 failed to authenticate with error: {err}"),
                }
                PamError::AUTH_ERR
            }
            Err(e) => {
                error!(logger, "libpam-auth0 failed to authenticate: {e}");
                PamError::AUTH_ERR
            }
        }
        
                
    }
}

pam_module!(PamAuth0);