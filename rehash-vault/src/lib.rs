use std::collections::HashMap;

use aes_gcm::{
    aead::{Aead, Nonce},
    Aes256Gcm, Key, KeyInit,
};
use base64::{engine::general_purpose::STANDARD, Engine};

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VaultSettings {
    pub default_generator_options: rehash_generator::GeneratorOptions,
    pub encrypt: bool,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VaultAccount {
    pub url: String,
    pub username: String,
    pub options: rehash_generator::FormatOptions,
    pub generator_options: rehash_generator::GeneratorOptions,
    pub display_name: Option<String>,
    pub notes: Option<String>,
}

impl From<VaultAccount> for rehash_generator::Account {
    fn from(account: VaultAccount) -> Self {
        rehash_generator::Account {
            url: account.url,
            username: account.username,
            options: account.options,
            generator_options: account.generator_options,
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum VaultFromStringError {
    #[error("could not decrypt encrypted vault: {0}")]
    DecryptError(DecryptError),
    #[error("unknown format: unencrypted vault failed because {0} \n encrypted vault failed because {1}")]
    UnknownFormatError(serde_json::Error, serde_json::Error),
}

pub fn load_from_json_string(string: &str, password: &str) -> Result<Vault, VaultFromStringError> {
    match serde_json::from_str::<Vault>(string) {
        Ok(unencrypted_vault) => Ok(unencrypted_vault),
        Err(err) => match serde_json::from_str::<EncryptedVault>(string) {
            Ok(encrypted_vault) => encrypted_vault
                .decrypt(password)
                .map_err(VaultFromStringError::DecryptError),
            Err(err2) => Err(VaultFromStringError::UnknownFormatError(err, err2)),
        },
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct Vault {
    pub settings: VaultSettings,
    pub entries: HashMap<uuid::Uuid, VaultAccount>,
}

#[derive(serde::Deserialize, Debug)]
pub struct EncryptedVault {
    pub iv: String,
    pub store: String,
}

#[derive(thiserror::Error, Debug)]
pub enum DecryptError {
    #[error("invalid encoding")]
    InvalidEncoding(base64::DecodeError),
    #[error("wrong password")]
    WrongPassword,
    #[error("mate")]
    AesError,
}

impl EncryptedVault {
    pub fn decrypt(self, password: &str) -> Result<Vault, DecryptError> {
        todo!("decryption");
    }
}
