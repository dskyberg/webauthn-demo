/// The first interaction from the UserAgent to the Server for the Attestation Registration Challenge
/// flow.  The UserAgent will collect attributes and send to the Server. The
/// Server will use these attributes to persist the User and to initialize  a
/// [PublicKeyCredentialCreationOptions], which is returned the UserAgent.  
///
/// The attributes in this object are not presented direct to CTAP Client. THis is
/// aninteraction strictly between the UserAgent and the Verifier.
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationChallengeRequest {
    pub name: String,
    pub display_name: String,
}
