//! `PointSharp HOTP`
//!
//! Usage: From the `PointSharp` portal retrieve the 6 digit activation code
//! then:
//!
//! ```shell
//! pointsharp <ACTIVATIONCODE> [COUNT]
//! ```
//! and use the resulting OTP value for logging in.
//!
//! Run with `--help` for a full list of options.
//!
//! The base32 encoded HOTP SECRET from `--oathmode` can be used by not only `oathtool`,
//! but all standard HOTP capable tools such as Aegis, Google Authenticator, Microsoft Authenticator etc.
//!
//! The default counter value is 123456789, and should be incremented by 1
//! for each authentication attempt, see general [HOTP](https://datatracker.ietf.org/doc/html/rfc4226)
//! functionality.
//!
//! Example HOTP:
//!
//! ```text
//! activation_code = "539787";
//! Count = 0, OTP = 750 277
//! Count = 1, OTP = 962 814
//! Count = 2, OTP = 331 876
//! Count = 3, OTP = 950 412
//! ```
//!
use ootp::hotp::{Hotp, MakeOption};

use clap::{CommandFactory, ErrorKind, Parser};

/// Simple program to calculate pointsharp HOTP codes
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// If set activate oathtool mode,
    /// useful with command substitution: oathtool $(pointsharp -o 123456)
    #[clap(short, long)]
    pub oathmode: bool,

    /// HOTP window, how many values to print
    #[clap(short, long)]
    pub window: Option<u32>,

    /// HOTP Secret, 6 digits
    activation_code: String,

    /// HOTP counter
    pub count: Option<u32>,
}

fn main() {
    let cli = Cli::parse();

    // From the Java application, converted from i8 to u8
    let mut seed: [u8; 20] = [
        0xD9, 0x9D, 0xBB, 0xF4, 0xF9, 0x97, 0x18, 0xEF, 0xFE, 0x25, 0xFD, 0x00, 0x7D, 0xFC, 0xE0,
        0x6B, 0xFB, 0x50, 0x30, 0xA5,
    ];

    // Default counter start value used by PointSharp
    let mut counter: u32 = 123_456_789;

    let activation_code = cli.activation_code;

    if let 1..=6 = activation_code.len() {
    } else {
        let mut cmd = Cli::command();
        cmd.error(
            ErrorKind::InvalidValue,
            "Must provide the 6 digit activation code",
        )
        .exit();
    }

    // Modify default seed with the activation code
    seed_add_activation_code(&mut seed, &activation_code);
    // Default minimum PointSharp counter added to user requested HOTP count
    counter += cli.count.unwrap_or(0);

    if cli.oathmode {
        // Activate oathtool base32 mode
        print!("--base32 {}", base32_encode(&seed));
        // Emit oathtool counter flag
        print!(" --counter {}", counter);
    } else {
        // Provide OTP directly via OOTP crate
        let seed_str = unsafe { std::str::from_utf8_unchecked(&seed) };
        let hotp = Hotp::new(seed_str);
        if let Some(w) = cli.window {
            // Print multiple consecutive OTP values
            for i in 0..w {
                let code = hotp.make(MakeOption::Counter(u64::from(counter + i)));
                println!("{}", code);
            }
        } else {
            let code = hotp.make(MakeOption::Counter(u64::from(counter)));
            println!("{}", code);
        }
    }
}

fn seed_add_activation_code(seed: &mut [u8], activation_code: &str) {
    // Copy the provided characters into the seed,
    // overwriting existing bytes
    for (i, c) in activation_code.chars().enumerate() {
        seed[i] = c as char as u8;
    }
}

fn base32_encode(secret: &[u8]) -> String {
    base32::encode(base32::Alphabet::RFC4648 { padding: false }, secret)
}
