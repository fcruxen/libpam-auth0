extern crate reqwest;
pub mod constants;
pub mod module;
pub mod macros;
pub mod items;

use items::AuthTok;
use module::PamHooks;
use module::PamHandle;
use constants::PamFlag;
use constants::PamResultCode;
use serde_json::json;
use std::ffi::CStr;

const AUTH0_TOKEN_URL: &str = "https://fcruxen-toptal.us.auth0.com/oauth/token";
const AUTH0_CLIENT_ID: &str = "...";
const AUTH0_CLIENT_SECRET: &str = "...";

pub struct Auth0PamModule;

pam_hooks!(Auth0PamModule);

impl PamHooks for Auth0PamModule {
    fn sm_authenticate(pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        // Get the user's username
        println!("pam lib starting...");
        let username = match pamh.get_user(None) {
            Ok(user) => user,
            Err(_) => return PamResultCode::PAM_USER_UNKNOWN,
        };
        println!("pam lib got username...");
        // Get the user's password
        let password: Option<AuthTok> = match pamh.get_item() {
            Ok(password) => password,
            Err(_) => return PamResultCode::PAM_AUTH_ERR,
        };
        println!("pam lib got passowrd...");
        // Make a POST request to the Auth0 token endpoint
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(AUTH0_TOKEN_URL)
            .header("Content-Type", "application/json")
            .json(&json!({
                "grant_type": "password",
                "username": username,
                "password": password.unwrap().to_string_lossy(),                
                "client_id": AUTH0_CLIENT_ID,
                "client_secret": AUTH0_CLIENT_SECRET,
            }))
            .send();

        match response {
            Ok(res) if res.status().is_success() => PamResultCode::PAM_SUCCESS,
            _ => PamResultCode::PAM_AUTH_ERR,
        }
    }

    
}


