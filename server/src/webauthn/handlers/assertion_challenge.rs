/// Verifier Assertion Challenge
///
/// To begin the assertion flow, the WebAuthn Client (aka the browser app) sends the WebAuthn Verifier
/// a user name. The Verifier can look up the username to determine appropriate options
/// to send to the WebAuthn Authenticator (via the WebAuthn Client)
///
use crate::webauthn::model::{PublicKeyCredentialRequestOptions, UserEntity};
use crate::{errors::Error, services::Session, DataServices};
use actix_web::{web, HttpRequest, HttpResponse};

pub async fn assertion_challenge(
    service: web::Data<DataServices>,
    request: web::Json<UserEntity>,
    _req: HttpRequest,
) -> Result<HttpResponse, Error> {
    log::info!("Registration Request: {:?}", &request);

    // The AppConfig will drive behavior
    let config = service.get_config().await?;

    // Get the user by name, or return 403
    let user = match service.get_user(&request.name).await? {
        Some(user) => user,
        None => {
            log::info!("User not found: {}", request.name);
            return Ok(HttpResponse::Forbidden().json(format!(
                r#"{{"message": "User not found: {}"}}"#,
                request.name
            )));
        }
    };

    // Get the credential id's for this user, or return 403
    let credential = match service.get_user_credential(&user.name).await? {
        Some(c) => c,
        None => {
            log::info!("Credential not found for user: {}", request.name);
            return Ok(HttpResponse::Forbidden().body(format!(
                r#"{{"message": "Credential not found: {}"}}"#,
                request.name
            )));
        }
    };

    // Create a challenge, and save it.
    let challenge = service.create_new_challenge().await?;

    // Create the PublicKey Creation Options
    let pk_options = PublicKeyCredentialRequestOptions::try_from((
        &config.webauthn,
        &credential,
        &challenge.value,
    ))?;

    // Create a session for the next step (response).
    let session = Session::default()
        .with("name", &user.name)
        .with("challenge", &pk_options.challenge.to_string());
    session.put_session(&service).await?;

    // Return the PK Options
    Ok(HttpResponse::Ok()
        .insert_header(session.to_header())
        .json(pk_options))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[test]
    fn test_it() {
        let msg = "error messasge";
        println!("{}", json! {{"message": msg}});
    }
}
