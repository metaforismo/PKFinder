# Bitcoin Private Key Finder 🔍

Questo progetto utilizza **Rust** per cercare una chiave privata Bitcoin in un determinato intervallo e verificare se genera un indirizzo Bitcoin specifico.

# Requisiti 📝

- **Rust** [toolchain ufficiale](https://www.rust-lang.org/it)
- Sistema operativo Linux, macOS o Windows
- Conoscenze di base di terminale

# Installazione 💽

### 1. Installa Rust
Rust può essere installato utilizzando il comando ufficiale `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Verifica l’installazione con:

```rustc --version```

### 2. Clona questo repository

Scarica i file del progetto nel tuo ambiente locale:

```bash
	git clone <repository_url> 
cd <nome_cartella_progetto>
```

### 3. Configura il progetto Rust

Assicurati che le dipendenze siano incluse. Apri il file Cargo.toml e aggiungi:

```bash
[dependencies]
bitcoin = "0.30.0"
rand = "0.8"
```

### 4. Compila il progetto

Compila il progetto in modalità ottimizzata:

```cargo build --release```

# Esecuzione 💻

### 1. Configura il programma

Nel file main.rs, modifica l’intervallo di ricerca e l’indirizzo Bitcoin desiderato:

```bash
let start: u128 = 0x80000000000000000;
let end: u128 = 0xfffffffffffffffff;
let target_address = "1MVDYgVaSN6iKKEsbzRUAYFrYJadLYZvvZ";
```

### 2. Avvia la ricerca

Esegui il programma in modalità release per ottenere le migliori prestazioni:

```cargo run --release```

### 3. Risultati

Se il programma trova una chiave privata valida, salverà i risultati in un file chiamato found_wallet.txt nella directory principale del progetto.

#### Note aggiuntive
	•	Velocità: La velocità del programma dipende dalla potenza della CPU. Per migliorare le prestazioni, considera l’uso di una GPU o un’implementazione parallela.
	•	Limitazioni: Questo programma è progettato solo per scopi educativi. L’uso per scopi illegali è severamente vietato.

# Troubleshooting 
### 1.	Problemi con la libreria Bitcoin?
Assicurati che la libreria bitcoin sia correttamente configurata. Controlla la versione corrente con:

```cargo update```


### 2.	Rust non funziona?
Verifica che rustc e cargo siano nel tuo PATH:

```echo $PATH```


### 3.	Errore di runtime?
Controlla che l’intervallo di ricerca e l’indirizzo Bitcoin siano correttamente formattati.

# Contribuire 🫂

Siamo sempre aperti a miglioramenti e nuove funzionalità. Sentiti libero di fare fork del progetto, implementare modifiche e inviare una pull request.

# Licenza 

Questo progetto è rilasciato sotto la licenza MIT.

