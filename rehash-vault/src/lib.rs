use std::collections::HashMap;

pub struct VaultSettings {
    pub default_generator_options: rehash_generator::GeneratorOptions,
    pub encrypt: bool,
}

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

pub struct Vault {
    pub settings: VaultSettings,
    pub entries: HashMap<uuid::Uuid, VaultAccount>,
}

pub struct EncryptedVault {
    pub iv: Box<[u8]>,
    pub store: Box<[u8]>,
}

impl Vault {
    pub fn encrypt(self, _password: &str) -> EncryptedVault {
        todo!("encryption")
    }
}

impl EncryptedVault {
    pub fn decrypt(self, _password: &str) -> Vault {
        todo!("decryption")
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {}
}
