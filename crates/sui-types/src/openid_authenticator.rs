// Copyright (c) 2021, Facebook, Inc. and its affiliates
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
use crate::{
    base_types::SuiAddress,
    committee::EpochId,
    crypto::{Signature, SuiSignature},
    error::SuiError,
    signature::AuthenticatorTrait,
};
use fastcrypto::rsa::RSAPublicKey;
use fastcrypto::rsa::RSASignature;
use fastcrypto_zkp::bn254::api::{Bn254Fr, ToConstraintField};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use shared_crypto::intent::Intent;
use shared_crypto::intent::{IntentMessage, IntentScope};
use std::{hash::Hash, str::FromStr};

#[cfg(test)]
#[path = "unit_tests/openid_authenticator_tests.rs"]
mod openid_authenticator_tests;

/// An open id authenticator with all the necessary field.
#[derive(Debug, Clone, PartialEq, Eq, JsonSchema, Hash, Serialize, Deserialize)]
pub struct OpenIdAuthenticator {
    pub vk_gamma_abc_g1: Vec<u8>,  // vk field
    pub alpha_g1_beta_g2: Vec<u8>, // vk field
    pub gamma_g2_neg_pc: Vec<u8>,  // vk field
    pub delta_g2_neg_pc: Vec<u8>,  // vk field
    proof_points: Vec<u8>,         // 3 elements: g1, g2, g1
    hash: Vec<u8>, // hash = sha2(padded_content) where padded_content = content + sha2_pad(content)
    masked_content: MaskedContent, // padded_content && bitmask
    max_epoch: EpochId,
    jwt_signature: Vec<u8>,
    user_signature: Signature,
    bulletin_signature: Signature,
    bulletin: Vec<OAuthProviderContent>,
}

impl OpenIdAuthenticator {
    fn get_ephemeral_pubkey(&self) -> &[u8] {
        self.user_signature.public_key_bytes()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, JsonSchema, Hash, Serialize, Deserialize)]
struct MaskedContent {
    content: Vec<u8>,
}

impl MaskedContent {
    fn header(&self) -> JWTHeader {
        JWTHeader {
            alg: "RS256".to_string(),
            kid: "986ee9a3b7520b494df54fe32e3e5c4ca685c89d".to_string(),
            typ: "JWT".to_string(),
        }
    }

    fn iss(&self) -> String {
        "https://accounts.google.com".to_string()
    }

    pub fn validate(&self) -> Result<(), SuiError> {
        if self.header().alg != "RS256" || self.header().typ != "JWT" {
            return Err(SuiError::InvalidAuthenticator);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, JsonSchema, Hash, Serialize, Deserialize)]
struct OAuthProviderContent {
    iss: String,
    kty: String,
    kid: String,
    e: String,
    n: String,
    alg: String,
}

struct JWTHeader {
    alg: String,
    kid: String,
    typ: String,
}

impl AuthenticatorTrait for OpenIdAuthenticator {
    /// Verify a proof for an intent message with its sender.
    fn verify_secure_generic<T>(
        &self,
        intent_msg: &IntentMessage<T>,
        author: SuiAddress,
        epoch: Option<EpochId>,
    ) -> Result<(), SuiError>
    where
        T: Serialize,
    {
        // Verify the author of the transaction is indeed the hash of the verifying key. 
        if author != self.into() {
            return Err(SuiError::InvalidAuthenticator);
        }

        if self.masked_content.validate().is_err() {
            return Err(SuiError::InvalidAuthenticator);
        }

        if self.max_epoch < epoch.unwrap_or(0) {
            return Err(SuiError::InvalidAuthenticator);
        }

        // Verify the foundation signature indeed commits to the OAuth provider content,
        // that is, a list of valid pubkeys available at https://www.googleapis.com/oauth2/v3/certs.
        if self
            .bulletin_signature
            .verify_secure(
                &IntentMessage::new(
                    Intent::default().with_scope(IntentScope::PersonalMessage),
                    self.bulletin.clone(),
                ),
                // foundation address
                SuiAddress::from_str(
                    "0x73a6b3c33e2d63383de5c6786cbaca231ff789f4c853af6d54cb883d8780adc0",
                )
                .unwrap(),
            )
            .is_err()
        {
            return Err(SuiError::InvalidSignature {
                error: "Bulletin signature verify failed".to_string(),
            });
        }
        println!("Bulletin signature verify");

        // Verify the JWT signature against the OAuth provider public key.
        let sig = RSASignature::from_bytes(&self.jwt_signature)?;
        let mut verified = false;
        for info in self.bulletin.iter() {
            if info.kid == self.masked_content.header().kid && info.iss == self.masked_content.iss()
            {
                let pk = RSAPublicKey::from_raw_components(
                    &base64_url::decode(&info.n).unwrap(),
                    &base64_url::decode(&info.e).unwrap(),
                )?;
                if pk.verify_prehash(&self.hash, &sig).is_ok() {
                    verified = true;
                }
            }
        }
        println!("verify jwt signature {:?}", verified);

        // if !verified {
        //     return Err(SuiError::InvalidSignature {
        //         error: "JWT signature verify failed".to_string(),
        //     });
        // }

        // Verify the user signature over the transaction data
        if self
            .user_signature
            .verify_secure(intent_msg, author)
            .is_err()
        {
            return Err(SuiError::InvalidSignature {
                error: "User signature verify failed".to_string(),
            });
        }
        println!("user sig verified");

        let public_inputs: Vec<Bn254Fr> = [
            &self.hash,
            &self.masked_content.content,
            self.get_ephemeral_pubkey(),
            &self.max_epoch.to_le_bytes(),
        ]
        .iter()
        .flat_map(|x| x.to_field_elements().unwrap())
        .collect();
        match fastcrypto_zkp::bn254::api::verify_groth16(
            &self.vk_gamma_abc_g1,
            &self.alpha_g1_beta_g2,
            &self.gamma_g2_neg_pc,
            &self.delta_g2_neg_pc,
            &public_inputs,
            &self.proof_points,
        ) {
            Ok(true) => Ok(()),
            Ok(false) | Err(_) => Err(SuiError::InvalidSignature {
                error: "Groth16 proof verification failed".to_string(),
            }),
        }
    }
}

impl AsRef<[u8]> for OpenIdAuthenticator {
    fn as_ref(&self) -> &[u8] {
        todo!()
    }
}
