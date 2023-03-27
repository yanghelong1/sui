// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::{
    base_types::SuiAddress,
    crypto::{get_key_pair_from_rng, DefaultHash, Signature, SuiKeyPair},
    openid_authenticator::{MaskedContent, OAuthProviderContent, OpenIdAuthenticator},
    signature::{AuthenticatorTrait, GenericSignature},
    utils::make_transaction,
};
use fastcrypto::encoding::{Encoding, Hex};
use fastcrypto::hash::HashFunction;
use rand::{rngs::StdRng, SeedableRng};
use shared_crypto::intent::{Intent, IntentMessage, IntentScope};

pub fn keys() -> Vec<SuiKeyPair> {
    let mut seed = StdRng::from_seed([0; 32]);
    let kp1: SuiKeyPair = SuiKeyPair::Ed25519(get_key_pair_from_rng(&mut seed).1);
    let kp2: SuiKeyPair = SuiKeyPair::Secp256k1(get_key_pair_from_rng(&mut seed).1);
    let kp3: SuiKeyPair = SuiKeyPair::Secp256r1(get_key_pair_from_rng(&mut seed).1);
    vec![kp1, kp2, kp3]
}

#[test]
fn openid_authenticator_scenarios() {
    let keys = keys();
    let foundation_key = &keys[0];
    let user_key = &keys[1];

    // Make the user address out of the verifying key.
    let vk_gamma_abc_g1 = Hex::decode("81aabea18713222ac45a6ef3208a09f55ce2dde8a11cc4b12788be2ae77ae318176d631d36d80942df576af651b57a31a95f2e9bcaebbb53a588251634715599f7a7e9d51fe872fe312edf0b39d98f0d7f8b5554f96f759c041ea38b4b1e5e19").unwrap();
    let alpha_g1_beta_g2 = Hex::decode("097ca8074c7f1d661e25d70fc2e6f14aa874dabe3d8a5d7751a012a737d30b59fc0f5f6d4ce0ea6f6c4562912dfb2a1442df06f9f0b8fc2d834ca007c8620823926b2fc09367d0dfa9b205a216921715e13deedd93580c77cae413cbb83134051cb724633c58759c77e4eda4147a54b03b1f443b68c65247166465105ab5065847ae61ba9d8bdfec536212b0dadedc042dab119d0eeea16349493a4118d481761b1e75f559fbad57c926d599e81d98dde586a2cfcc37b49972e2f9db554e5a0ba56bec2d57a8bfed629ae29c95002e3e943311b7b0d1690d2329e874b179ce5d720bd7c5fb5a2f756b37e3510582cb0c0f8fc8047305fc222c309a5a8234c5ff31a7b311aabdcebf4a43d98b69071a9e5796372146f7199ba05f9ca0a3d14b0c421e7f1bd02ac87b365fd8ce992c0f87994d0ca66f75c72fed0ce94ca174fcb9e5092f0474e07e71e9fd687b3daa441193f264ca2059760faa9c5ca5ef38f6ecefef2ac7d8c47df67b99c36efa64f625fe3f55f40ad1865abbdf2ff4c3fc3a162e28b953f6faec70a6a61c76f4dca1eecc86544b88352994495ae7fc7a77d387880e59b2357d9dd1277ae7f7ee9ba00b440e0e6923dc3971de9050a977db59d767195622f200f2bf0d00e4a986e94a6932627954dd2b7da39b4fcb32c991a0190bdc44562ad83d34e0af7656b51d6cde03530b5d523380653130b87346720ad6dd425d8133ffb02f39a95fc70e9707181ecb168bd8d2d0e9e85e262255fecab15f1ada809ecbefa42a7082fa7326a1d494261a8954fe5b215c5b761fb10b7f18").unwrap();
    let gamma_g2_neg_pc = Hex::decode("8398b153643614fc1071a54e288edb6402f1d9e00d3408c76d95c16885cc992dff5c6ebee3b739cb22359ab2d126026a1626c43ea7b898a7c1d2904c1bd4bbce5d0b1b16fab8535a52d1b08a5217df2e912ee1b0f4140892afa31d479f78dfbc").unwrap();
    let delta_g2_neg_pc = Hex::decode("a2ab58a209ad00df6c86ab14841e8daa7a380a6853f28bacf38aad9903b6149fff4b119dea16de8aa3e5050b9d563a01009e061a950c233f66511c8fae2a8c58503059821df7f6defbba8f93d26e412cc07b66a9f3cdd740cce5c8488ce94fc8").unwrap();

    let mut hasher = DefaultHash::default();
    hasher.update(&vk_gamma_abc_g1);
    hasher.update(&alpha_g1_beta_g2);
    hasher.update(&gamma_g2_neg_pc);
    hasher.update(&delta_g2_neg_pc);
    let user_address = SuiAddress::from_bytes(hasher.finalize().digest).unwrap();

    // Create an example bulletin with 2 keys from Google.
    let example_bulletin = vec![
        OAuthProviderContent {
            iss: "https://accounts.google.com".to_string(),
            kty: "RSA".to_string(),
            kid: "986ee9a3b7520b494df54fe32e3e5c4ca685c89d".to_string(),
            e: "AQAB".to_string(),
            n: "onb-s1Mvbpti06Sp-ZsHH5eeJxdvMhRgfmx5zK7cVlcAajI_0rKu8ylU2CkfgPlMe9-8W5ayozm1h2yx2ToS7P7qoR1sMINXbKxobu8xy9zOBuFAr3WvEoor6lo0Qp747_4bN1sVU6GBEBEXLjb8vHN-o_yoBv8NSB_yP7XbEaS3U5MJ4V2s5o7LziIIRP9PtzF0m3kWm7DuyEzGvCaW8s9bOiMd3eZyXXyfKjlBB727eBXgwqcV-PttECRw6JCLO-11__lmqfKIj5CBw18Pb4ZrNwBa-XrGXfHSSAJXFkR4LR7Bj24sWzlOcKXN2Ew4h3WDJfxtN_StNSYoagyaFQ".to_string(),
            alg: "RS256".to_string(),
        },
        OAuthProviderContent {
            iss: "https://accounts.google.com".to_string(),
            kty: "RSA".to_string(),
            kid: "1aae8d7c92058b5eea456895bf89084571e306f3".to_string(),
            e: "AQAB".to_string(),
            n: "sguIKIvlEVBsEGk77iV2yNQxpY_Qkiy3yuMfY4wpmnPlevlDKASu6uP_CGubzThiBHlChYDDNvYfOitWXDwpxbJ_MqmajA-dDbrI5LdggyJpSoWPKThPJ1CKRhRiRXJjXGi6Gg6TfbYRwu0ziyDgZZ125NszuNOUO1pc1qGun4SPifzY7OY6BtADZDqTWHFTfm_yhgBgyElE-r4d-ZqPe9tYYqCnAvILBuZbPYt3UC3fAr9JltdUO54vxKblo2z2fd-H9zBn9jRZOBkuVVB8QSV5sre-H23CTBABSpZpe0SrJpXgG9AuV4Da7FRHBC9A-oLYLe-UF5_5c6_cd7c_KQ".to_string(),
            alg: "RS256".to_string(),
        },
    ];

    // Sign the bulletin content with the sui foundation key as a personal message.
    let bulletin_sig = Signature::new_secure(
        &IntentMessage::new(
            Intent::default().with_scope(IntentScope::PersonalMessage),
            example_bulletin.clone(),
        ),
        foundation_key,
    );

    // Sign the user transaction with the user's ephemeral key.
    let tx = make_transaction(
        user_address,
        user_key,
        Intent::default().with_scope(IntentScope::PersonalMessage),
    );
    let s = match tx.inner().tx_signatures.first().unwrap() {
        GenericSignature::Signature(s) => s,
        _ => panic!("Expected a signature"),
    };

    let authenticator = OpenIdAuthenticator {
        vk_gamma_abc_g1,
        alpha_g1_beta_g2,
        gamma_g2_neg_pc,
        delta_g2_neg_pc,
        proof_points: Hex::decode("a29981304df8e0f50750b558d4de59dbc8329634b81c986e28e9fff2b0faa52333b14a1f7b275b029e13499d1f5dd8ab955cf5fa3000a097920180381a238ce12df52207597eade4a365a6872c0a19a39c08a9bfb98b69a15615f90cc32660180ca32e565c01a49b505dd277713b1eae834df49643291a3601b11f56957bde02d5446406d0e4745d1bd32c8ccb8d8e80b877712f5f373016d2ecdeebb58caebc7a425b8137ebb1bd0c5b81c1d48151b25f0f24fe9602ba4e403811fb17db6f14").unwrap(),
        hash: vec![],
        masked_content: MaskedContent {content: vec![]},
        max_epoch: 1,
        jwt_signature: base64_url::decode("iolDOQ326ix-qttDNrjVbq9Kvh5M1WOWMUCN-M7lqOKv-tcCu8tyhgH5wScBwzWBLzHg0YsdML6BGT6SmC1DESVUU9UhCL35oY4s2mL9iiz3CNVIkupmvdmQnDYvTXyXBsX94G3abAqE2Up_2ci7d21o29lgjKC6p8AoWAHWLpaTDabWQ54yYQAjeOgjYEKlBt54w6ATVHNKNeYDBEnd4hWSPWVbGyor4JQPum2DuxhHaWWFLkhb7l9nS6QITfNYRlPbQUAsE9filBlqsxoCeul2pbX6z1kVvWnOv885VUqnCGJpkJBVGx9_XORoNIOHj4Hs_VRON2rqVnfxKj1aCA").unwrap(),
        user_signature: s.clone(),
        bulletin_signature: bulletin_sig,
        bulletin: example_bulletin
    };
    assert!(authenticator
        .verify_secure_generic(
            &IntentMessage::new(Intent::default(), tx),
            user_address,
            Some(0)
        )
        .is_ok());
}

#[test]
fn test_authenticator_failure() {}

#[test]
fn test_serde_roundtrip() {}

#[test]
fn test_open_id_authenticator_address() {}
