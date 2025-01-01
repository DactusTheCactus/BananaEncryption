use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use regex::Regex;
use sha2::{Digest, Sha256};

pub struct ApeBananaEncryptor {
    seed: String,
}

impl ApeBananaEncryptor {
    pub fn new(seed: &str) -> Self {
        ApeBananaEncryptor {
            seed: seed.to_string(),
        }
    }

    pub fn encrypt(&self, message: &str) -> String {
        let mut rng = ChaCha20Rng::from_seed(self.generate_seed());

        // Step 1: Transform message
        let mut transformed_message = format!("🦍APESSTRONK{}BANANA🍌", message);

        // Step 2: Encrypt each byte
        let mut encrypted = Vec::new();
        for (i, &byte) in transformed_message.as_bytes().iter().enumerate() {
            // Rotate byte based on position
            let rotated = byte.rotate_left((i % 8) as u32);

            // Add random offset
            let random_offset: u8 = rng.gen_range(1..=255);
            let transformed = rotated.wrapping_add(random_offset);

            encrypted.push(transformed);
        }

        // Step 3: Add random “fake banana” padding
        let fake_bananas_count: u8 = rng.gen_range(3..=10);
        for _ in 0..fake_bananas_count {
            // Insert random bytes
            encrypted.push(rng.gen());
        }

        // Step 4: Store how many fake bananas we added (1 byte)
        encrypted.push(fake_bananas_count);

        // Step 5: Base64-encode
        let encoded_message = base64::encode(encrypted);

        // Step 6: Build final string with prefix/suffix
        let mut final_message = format!("🍌{}🦍", encoded_message);

        // STEP 7: IRONIC EMOJI + PHRASE INSERTION
        // We'll randomly sprinkle some monkey emojis/phrases around.
        let monkey_phrases = [
            "Oo-oo-aa-aa!",
            "Bananaaaaaa!",
            "Ugha ugha!",
            "APE TOGETHER STRONG!",
        ];
        let emojis = ["🍌", "🦍", "🐒", "🐵", "👀"];

        // Decide how many monkey inserts
        let num_inserts = rng.gen_range(1..=3);
        for _ in 0..num_inserts {
            let phrase = monkey_phrases[rng.gen_range(0..monkey_phrases.len())];
            let emoji = emojis[rng.gen_range(0..emojis.len())];
            // Insert chunk like: 🐵🍌_APE TOGETHER STRONG!🐵
            // The `🐵` markers are how we'll find & remove them in decrypt.
            let monkey_chunk = format!("🐵{}_{}🐵", emoji, phrase);
            final_message.push_str(&monkey_chunk);
        }

        final_message
    }

    pub fn decrypt(&self, encrypted_message: &str) -> String {
        // 1) Strip the extra monkey-chunks (they're not part of the real data!)
        let re = Regex::new(r"🐵[^🐵]*🐵").unwrap();
        let cleaned_message = re.replace_all(encrypted_message, "");

        // 2) Strip our custom prefix/suffix
        let stripped_message = cleaned_message
            .strip_prefix("🍌")
            .and_then(|s| s.strip_suffix("🦍"))
            .unwrap_or(&cleaned_message);

        // 3) Base64-decode
        let encrypted = match base64::decode(stripped_message) {
            Ok(data) => data,
            Err(_) => {
                return String::from("ERROR: Failed to decode base64. Possibly corrupted data?");
            }
        };

        // 4) Check length
        if encrypted.is_empty() {
            return String::from("ERROR: Encrypted data is empty.");
        }

        // The last byte is the number of fake bananas
        let fake_bananas_count = *encrypted.last().unwrap() as usize;
        if encrypted.len() < 1 + fake_bananas_count {
            return String::from("ERROR: Malformed or corrupted ciphertext.");
        }

        // The true payload ends right before last byte minus the fake bananas
        let payload_end = encrypted.len() - 1 - fake_bananas_count;
        let payload = &encrypted[..payload_end];

        let mut rng = ChaCha20Rng::from_seed(self.generate_seed());
        let mut decrypted = Vec::with_capacity(payload.len());

        // 5) Reverse the encryption
        for (i, &byte) in payload.iter().enumerate() {
            // Remove the random offset
            let random_offset: u8 = rng.gen_range(1..=255);
            let untransformed = byte.wrapping_sub(random_offset);

            // Reverse rotate
            let original = untransformed.rotate_right((i % 8) as u32);
            decrypted.push(original);
        }

        // 6) Convert back to string
        let decrypted_message = match String::from_utf8(decrypted) {
            Ok(s) => s,
            Err(_) => return String::from("ERROR: Decrypted bytes are not valid UTF-8."),
        };

        // 7) Strip known prefix/suffix from the real message
        decrypted_message
            .strip_prefix("🦍APESSTRONK")
            .and_then(|s| s.strip_suffix("BANANA🍌"))
            .unwrap_or(&decrypted_message)
            .to_string()
    }

    // Uses a SHA-256 hash of the user’s seed string to generate the RNG seed.
    fn generate_seed(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(self.seed.as_bytes());
        let result = hasher.finalize();

        let mut seed = [0u8; 32];
        seed.copy_from_slice(&result[..32]);
        seed
    }
}

fn main() {
    let king_seed = "MIGHTY_RUST_APE_SEED";
    let encryptor = ApeBananaEncryptor::new(king_seed);

    let message = "ProtectTheShinyBananas!";
    println!("Original Message: {}", message);

    let encrypted_message = encryptor.encrypt(message);
    println!("Encrypted Message: {}", encrypted_message);

    let decrypted_message = encryptor.decrypt(&encrypted_message);
    println!("Decrypted Message: {}", decrypted_message);
}
