use std::collections::HashMap;

pub struct VaultSettings {
    default_generator_options: rehash_generator::GeneratorOptions,
    encrypt: bool,
}

pub struct VaultAccount {
    url: String,
    username: String,
    options: rehash_generator::FormatOptions,
    generator_options: rehash_generator::GeneratorOptions,
    display_name: Option<String>,
    notes: Option<String>,
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
    settings: VaultSettings,
    entries: HashMap<uuid::Uuid, VaultAccount>,
}

pub struct EncryptedVault {
    iv: Box<[u8]>,
    store: Box<[u8]>,
}

impl Vault {
    pub fn encrypt(self, password: &str) -> EncryptedVault {
        todo!("encryption")
    }
}

impl EncryptedVault {
    pub fn decrypt(self, password: &str) -> Vault {
        todo!("decryption")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
