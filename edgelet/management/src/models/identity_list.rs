/*
 * IoT Edge Management API
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: 2018-06-28
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityList {
    #[serde(rename = "identities")]
    identities: Vec<::models::Identity>,
}

impl IdentityList {
    pub fn new(identities: Vec<::models::Identity>) -> IdentityList {
        IdentityList { identities }
    }

    pub fn set_identities(&mut self, identities: Vec<::models::Identity>) {
        self.identities = identities;
    }

    pub fn with_identities(mut self, identities: Vec<::models::Identity>) -> IdentityList {
        self.identities = identities;
        self
    }

    pub fn identities(&self) -> &Vec<::models::Identity> {
        &self.identities
    }
}
