// Importazione delle dipendenze necessarie
use rand::{Rng, SeedableRng};      // Per la generazione di numeri casuali
use rand_chacha::ChaCha20Rng;      // Implementazione specifica del generatore casuale
use regex::Regex;                   // Per le operazioni di regex
use sha2::{Digest, Sha256};        // Per l'hashing SHA-256, controllo dell'integrità

// Struttura principale dell'encryptor
pub struct ApeBananaEncryptor {
    seed: String,  // Chiave segreta usata per la crittografia
}

impl ApeBananaEncryptor {
    // Costruttore che inizializza l'encryptor con un seed
    pub fn new(seed: &str) -> Self {
        ApeBananaEncryptor {
            seed: seed.to_string(),
        }
    }

    // Funzione di crittografia
    pub fn encrypt(&self, message: &str) -> String {
        // Inizializza il generatore di numeri casuali con il seed
        let mut rng = ChaCha20Rng::from_seed(self.generate_seed());

        // Passo 1: Aggiunge un prefisso e suffisso al messaggio
        let mut transformed_message = format!("🦍APESSTRONK{}BANANA🍌", message);

        // Passo 2: Cripta ogni byte del messaggio | Encrypted è il nostro messaggio banana
        let mut encrypted = Vec::new();
        for (i, &byte) in transformed_message.as_bytes().iter().enumerate() {
            // Rotazione del byte basata sulla posizione
            let rotated = byte.rotate_left((i % 8) as u32);
            
            // Aggiunge un offset casuale
            let random_offset: u8 = rng.gen_range(1..=255);
            let transformed = rotated.wrapping_add(random_offset);
            
            encrypted.push(transformed);
        }

        // Passo 3: Aggiunge padding casuale ("banane finte")
        let fake_bananas_count: u8 = rng.gen_range(3..=10);
        for _ in 0..fake_bananas_count {
            encrypted.push(rng.gen());
        }

        // Passo 4: Memorizza il numero di banane finte aggiunte
        encrypted.push(fake_bananas_count);

        // Passo 5: Codifica in base64
        let encoded_message = base64::encode(encrypted);

        // Passo 6: Costruisce il messaggio finale con prefisso/suffisso
        let mut final_message = format!("🍌{}🦍", encoded_message);

        // Passo 7: Inserisce emoji e frasi ironiche casuali
        let monkey_phrases = [
            "Oo-oo-aa-aa!",
            "Bananaaaaaa!",
            "Ugha ugha!",
            "APE TOGETHER STRONG!",
        ];
        let emojis = ["🍌", "🦍", "🐒", "🐵", "👀"];

        // Inserisce da 1 a 3 frasi casuali
        let num_inserts = rng.gen_range(1..=3);
        for _ in 0..num_inserts {
            let phrase = monkey_phrases[rng.gen_range(0..monkey_phrases.len())];
            let emoji = emojis[rng.gen_range(0..emojis.len())];
            let monkey_chunk = format!("🐵{}_{}🐵", emoji, phrase);
            final_message.push_str(&monkey_chunk);
        }

        final_message
    }

    // Funzione di decrittografia
    pub fn decrypt(&self, encrypted_message: &str) -> String {
        // Passo 1: Rimuove le frasi monkey aggiunte
        let re = Regex::new(r"🐵[^🐵]*🐵").unwrap();
        let cleaned_message = re.replace_all(encrypted_message, "");

        // Passo 2: Rimuove prefisso/suffisso
        let stripped_message = cleaned_message
            .strip_prefix("🍌")
            .and_then(|s| s.strip_suffix("🦍"))
            .unwrap_or(&cleaned_message);

        // Passo 3: Decodifica base64
        let encrypted = match base64::decode(stripped_message) {
            Ok(data) => data,
            Err(_) => {
                return String::from("ERROR: Failed to decode base64. Possibly corrupted data?");
            }
        };

        // Passo 4: Verifica la lunghezza e recupera il conteggio delle banane finte
        if encrypted.is_empty() {
            return String::from("ERROR: Encrypted data is empty.");
        }

        let fake_bananas_count = *encrypted.last().unwrap() as usize;
        if encrypted.len() < 1 + fake_bananas_count {
            return String::from("ERROR: Malformed or corrupted ciphertext.");
        }

        // Rimuove le banane finte e il byte del conteggio
        let payload_end = encrypted.len() - 1 - fake_bananas_count;
        let payload = &encrypted[..payload_end];

        // Inizializza RNG con lo stesso seed
        let mut rng = ChaCha20Rng::from_seed(self.generate_seed());
        let mut decrypted = Vec::with_capacity(payload.len());

        // Passo 5: Inverte la crittografia
        for (i, &byte) in payload.iter().enumerate() {
            // Rimuove l'offset casuale
            let random_offset: u8 = rng.gen_range(1..=255);
            let untransformed = byte.wrapping_sub(random_offset);

            // Inverte la rotazione
            let original = untransformed.rotate_right((i % 8) as u32);
            decrypted.push(original);
        }

        // Passo 6: Converte in stringa
        let decrypted_message = match String::from_utf8(decrypted) {
            Ok(s) => s,
            Err(_) => return String::from("ERROR: Decrypted bytes are not valid UTF-8."),
        };

        // Passo 7: Rimuove il prefisso/suffisso dal messaggio originale
        decrypted_message
            .strip_prefix("🦍APESSTRONK")
            .and_then(|s| s.strip_suffix("BANANA🍌"))
            .unwrap_or(&decrypted_message)
            .to_string()
    }

    // Genera un seed di 32 byte usando SHA-256
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

    let message = "TEst Messaggio ODIO CISCO";
    println!("Original Message: {}", message);

    let encrypted_message = encryptor.encrypt(message);
    println!("Encrypted Message: {}", encrypted_message);

    let decrypted_message = encryptor.decrypt(&encrypted_message);
    println!("Decrypted Message: {}", decrypted_message);
}