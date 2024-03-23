pub enum RecommendedGeneratorOption {
    Recommended2024,
}

pub struct CustomGeneratorOptions {
    pub iterations: u32,
    pub memory_size: u32,
    pub parallelism: u32,
}

pub enum GeneratorOptions {
    Recommended(RecommendedGeneratorOption),
    Custom(CustomGeneratorOptions),
}

impl RecommendedGeneratorOption {
    fn as_custom(&self) -> CustomGeneratorOptions {
        match self {
            RecommendedGeneratorOption::Recommended2024 => CustomGeneratorOptions {
                iterations: 16,
                memory_size: 16384,
                parallelism: 2,
            },
        }
    }
}

impl GeneratorOptions {
    fn into_custom(self) -> CustomGeneratorOptions {
        match self {
            GeneratorOptions::Recommended(recommended) => recommended.as_custom(),
            GeneratorOptions::Custom(custom) => custom,
        }
    }
}

pub struct FormatOptions {
    pub generation: usize,
    pub length: usize,
}

pub struct Account {
    pub url: String,
    pub username: String,
    pub options: FormatOptions,
    pub generator_options: GeneratorOptions,
}

#[derive(thiserror::Error, Debug)]
pub enum GeneratePasswordError {
    #[error("invalid argon2 parameters: {0}")]
    InvalidParameters(argon2::Error),
    #[error("argon2 hash failed: {0}")]
    GenerateError(argon2::Error),
}

pub fn generate(password: &str, account: Account) -> Result<String, GeneratePasswordError> {
    use argon2::{password_hash::Output, Algorithm, Argon2, Params, Version};
    use base64::{engine::general_purpose::STANDARD, Engine as _};

    let options = account.generator_options.into_custom();

    let argon2 = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(
            options.memory_size,
            options.iterations,
            options.parallelism,
            Some(account.options.length),
        )
        .map_err(GeneratePasswordError::InvalidParameters)?,
    );

    let salt = format!(
        "{}{}{}",
        account.username, account.options.generation, account.url
    );

    // pad right with spaces for minimum width of 8
    let salt = format!("{salt:<8}");

    let mut buffer = [0; Output::MAX_LENGTH];

    argon2
        .hash_password_into(
            password.as_bytes(),
            salt.as_bytes(),
            &mut buffer[..account.options.length],
        )
        .map_err(GeneratePasswordError::GenerateError)?;

    let encoded = STANDARD.encode(buffer);

    Ok(encoded.split_at(account.options.length).0.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_passwords_like_rehash_web() {
        let generated_password = generate(
            "hunter2",
            Account {
                url: "www.google.com".to_owned(),
                username: "jondoe@gmail.com".to_owned(),
                options: FormatOptions {
                    generation: 1,
                    length: 32,
                },
                generator_options: GeneratorOptions::Custom(CustomGeneratorOptions {
                    iterations: 15,
                    memory_size: 2048,
                    parallelism: 2,
                }),
            },
        )
        .unwrap();

        assert_eq!(generated_password, "h5cTlQyD0lyC42l2A6im6evdb4PAlTNS");
    }
}
