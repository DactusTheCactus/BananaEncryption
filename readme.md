### **ApeBananaEncryptor: Guida Pratica**

---

#### **Introduzione**
L'**ApeBananaEncryptor** è una semplice libreria Rust progettata per crittografare e decrittografare messaggi. Nonostante il tema ludico e ironico, fornisce un esempio di come implementare algoritmi di trasformazione di dati usando Rust. **Nota importante:** Questa libreria non è destinata a garantire sicurezza reale, ma solo a scopi educativi e ludici.

---

### **Funzionalità**

1. **Crittografia dei messaggi:**
   - Trasforma i messaggi leggibili in stringhe crittografate non comprensibili.
   - Aggiunge "rumore" al messaggio per renderlo apparentemente casuale.

2. **Decrittografia dei messaggi:**
   - Ripristina i messaggi originali se si dispone della chiave corretta.

---

### **Utilizzo**

#### **Costruzione dell'oggetto Encryptor**
```rust
let seed = "chiave_segreta";
let encryptor = ApeBananaEncryptor::new(seed);
```

#### **Crittografia**
```rust
let messaggio = "ProteggiIlMessaggio";
let messaggio_crittografato = encryptor.encrypt(messaggio);
println!("Messaggio crittografato: {}", messaggio_crittografato);
```

#### **Decrittografia**
```rust
let messaggio_originale = encryptor.decrypt(&messaggio_crittografato);
println!("Messaggio decrittografato: {}", messaggio_originale);
```

---

### **Dettagli Tecnici**

#### **Struttura Principale**
```rust
pub struct ApeBananaEncryptor {
    seed: String, // Chiave segreta per la crittografia
}
```

#### **Algoritmo**
1. Aggiunta di prefissi e suffissi per delimitare il messaggio.
2. Manipolazione di byte attraverso:
   - Rotazione basata sull'indice del byte.
   - Offset casuale generato da un RNG basato su un seed.
3. Aggiunta di byte casuali per mascherare il messaggio.
4. Codifica finale in base64 per la rappresentazione leggibile.

#### **Decodifica**
- Rimozione di byte casuali.
- Decodifica base64.
- Ripristino del messaggio originale invertendo le operazioni di manipolazione.

---

### **Sicurezza**
**Nota importante:** Questo schema **non è sicuro** per l'uso in produzione. È un esempio didattico di manipolazione dei dati. 😊

---

### **Esempio Completo**
```rust
fn main() {
    let seed = "chiave_segreta";
    let encryptor = ApeBananaEncryptor::new(seed);

    let messaggio = "ProteggiIlMessaggio";
    println!("Messaggio originale: {}", messaggio);

    let messaggio_crittografato = encryptor.encrypt(messaggio);
    println!("Messaggio crittografato: {}", messaggio_crittografato);

    let messaggio_decrittografato = encryptor.decrypt(&messaggio_crittografato);
    println!("Messaggio decrittografato: {}", messaggio_decrittografato);
}
```
### 🦍 **Guida della Tribù delle Scimmie al Leggendario ApeBananaEncryptor** 🍌

---

**OOOH OOOOH! CRITTOGRAFIA BANANA SOLO PER SCIMMIE DAL CERVELLONE!** 🧠🦍

Benvenuti, possenti scimmie della giungla di internet! Oggi, King Kong in persona ci ha benedetti con i segreti del **ApeBananaEncryptor**, l'arma definitiva per proteggere le vostre splendenti banane dai scimmioni curiosi! 🦍🍌 Questo non è solo codice—è una dichiarazione di superiorità scimmiesca! Sbucciamo questa banana e vediamo cosa nasconde! 🍌🔧

---

### 🛠️ **Strumenti Scimmieschi Presi in Prestito**

- **`rand::{Rng, SeedableRng}`** 🌀  
  - Fa accadere la magia delle banane casuali.  
  - `Rng`: Tira i dadi per le scimmie!  
  - `SeedableRng`: Aiuta le scimmie a ricordare come hanno tirato i dadi. Cose da scimmie intelligenti.

- **`rand_chacha::ChaCha20Rng`** 🎲  
  - Uno strumento di casualità avanzato da scimmia. Non sempre usato qui, ma incluso per vantarsi. 🦍💪

- **`base64::{engine::general_purpose, Engine}`** 📜  
  - Trasforma i noiosi byte in una purea di banana leggibile e gustosa. Sicura per occhi scimmieschi. 👀🍌

---

### 🍌 **Che Cos’è ApeBananaEncryptor?**

È uno strumento super-scimmia per:  
1. Sminuzzare i vostri preziosi messaggi-banana. 🍌➡️🦍➡️🤔  
2. Restituire un miscuglio di emoji e frasi da scimmia.  
3. Revertire la magia (se avete il **seme scimmiesco**) per riavere le vostre banane. 🦍🍌➡️🔑➡️📜

---

### 🏗️ **Il Progetto: ApeBananaEncryptor**

```rust
pub struct ApeBananaEncryptor {
    seed: String,
}
```

- **`seed`**: La sacra password banana. Proteggila come un tesoro di banane splendenti. NON CONDIVIDERLA, NEMMENO CON ALTRE SCIMMIE! 🛡️🍌

---

### 🦍 **La Magia Spiegata alle Scimmie**

#### **1. `new()` - Costruisci il Protettore delle Banane**

**Traduzione per Scimmie:**  
- Il re delle scimmie chiama questa funzione per costruire una macchina protettrice di banane.  
- Dagli il tuo speciale **seme scimmiesco**, ed è pronta per difendere i tuoi tesori fruttati.

#### **2. `encrypt()` - Trasforma le Banane in Confusione**

**Cosa Succede nella Giungla:**  
1. Avvolge il messaggio-banana con **`APESSTRONK`** e **`BANANA`**. 🍌💪  
2. Mescola le lettere come una scimmia confusa usando rotazioni di byte. 🙉  
3. Aggiunge spruzzi di **banana casuale** per ulteriore confusione. 🍌✨  
4. Confeziona tutto insieme (IV, miscuglio crittografato e offset).  
5. Aggiunge un tocco di emoji e frasi scimmiesche come:  
   - **“OOOH OOOH!”**  
   - **“È ORA DI BANANE!”**  
   - **“SCIMMIE FORTI!”**

**Risultato:** Una stringa ridicola che sembra creata da una scimmia che ha tirato emoji ovunque. 🙈

---

#### **3. `decrypt()` - Riporta Indietro le Banane**

**Cosa Succede nella Giungla:**  
1. Rimuove tutto il **nonsense scimmiesco** (emoji e frasi).  
2. Inverte le trasformazioni (de-mescola i byte).  
3. Toglie l’avvolgimento di **`APESSTRONK`** e **`BANANA`**.  
4. Voilà! Le tue banane tornano al loro posto. 🍌🎉

---

### 🔒 **Quanto È Sicura Questa Magia Scimmiesca?**

**Risposta Breve:** Non molto. 🙊

**Perché?**  
- È come usare bucce di banana come serrature—divertente ma facile da scassinare.  
- Rotazione di byte e offset casuali? Una scimmia curiosa potrebbe scoprirlo.  
- Seme hardcoded? Grande errore scimmiesco. Le vere scimmie usano le casseforti! 🔐

---

### 🍌 **Regole della Giungla di King Kong**

- **Crittografa:** Proteggi le tue banane!
  ```rust
  let encrypted_message = encryptor.encrypt("IlTuoMessaggioSegretoBanana!");
  ```

- **Decrittografa:** Riporta indietro le banane!
  ```rust
  let decrypted_message = encryptor.decrypt(&encrypted_message);
  ```

---

### 👑 **Parole Finali dal Re delle Scimmie**

"OOOH OOOH! Scimmie unite forti! Proteggete le vostre banane e diffondete la saggezza dell'ApeBananaEncryptor in tutta la giungla! SCIMMIE POTENTI!!!" 🦍🍌

--- 
