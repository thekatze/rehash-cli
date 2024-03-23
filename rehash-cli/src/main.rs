use clap::Parser as _;
use color_eyre::eyre::Context as _;

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(
        long,
        help = "The url of the service to generate the password for, required to derive the password"
    )]
    url: String,
    #[arg(
        long,
        help = "The username for the service to generate the password for, required to derive the password"
    )]
    username: String,

    #[arg(
        long,
        default_value_t = 1,
        help = "An arbitrary number to increment if the password needs to be changed"
    )]
    generation: usize,
    #[arg(
        short,
        long,
        default_value_t = 32,
        help = "Sets the length of generated password"
    )]
    length: usize,

    #[arg(
        long,
        default_value_t = false,
        help = "Prints password to console instead of copying it to the clipboard"
    )]
    print: bool,

    #[arg(long, help = "Sets password to skip the interactive password prompt")]
    password: Option<String>,

    #[command(flatten)]
    custom: CustomGeneratorOptions,
}

#[derive(clap::Args, Clone)]
struct CustomGeneratorOptions {
    #[arg(short, help = "Overrides argon2 parameter iterations (t_cost)")]
    iterations: Option<u32>,
    #[arg(short, help = "Overrides argon2 parameter memory size (m_cost)")]
    memory_size: Option<u32>,
    #[arg(short, help = "Overrides argon2 parameter parallelism (p_cost)")]
    parallelism: Option<u32>,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let cli = Args::parse();

    let generator_options = match (
        cli.custom.iterations,
        cli.custom.memory_size,
        cli.custom.parallelism,
    ) {
        (None, None, None) => rehash_generator::GeneratorOptions::Recommended(
            rehash_generator::RecommendedGeneratorOption::Recommended2024,
        ),
        (Some(iterations), Some(memory_size), Some(parallelism)) => {
            rehash_generator::GeneratorOptions::Custom(rehash_generator::CustomGeneratorOptions {
                iterations,
                memory_size,
                parallelism,
            })
        }
        _ => color_eyre::eyre::bail!(
            "not all custom generator options passed, -i <ITERATIONS> -m <MEMORY_SIZE> and -p <PARALLELISM> must all be passed together"
        ),
    };

    let vault_password = cli.password.unwrap_or(
        rpassword::prompt_password("Enter password: ")
            .context("could not prompt password, expected interactive shell")?,
    );

    let password = rehash_generator::generate(
        &vault_password,
        rehash_generator::Account {
            url: cli.url,
            username: cli.username,
            options: rehash_generator::FormatOptions {
                generation: cli.generation,
                length: cli.length,
            },
            generator_options,
        },
    )
    .context("generating password failed")?;

    if cli.print {
        println!("{}", password);
    } else {
        arboard::Clipboard::new()
            .context("could not get clipboard access")?
            .set_text(password)
            .context("could not write to clipboard")?;

        println!("Generated password copied to clipboard");
    }

    Ok(())
}
