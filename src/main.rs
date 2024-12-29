use bitcoin::{Network, PrivateKey, PublicKey, Address};
use bitcoin::secp256k1::{Secp256k1, SecretKey};
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{Instant, Duration};
use std::thread;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

fn save_private_key(target_address: &str, private_key: &PrivateKey) -> Result<(), std::io::Error> {
    let file_path = "/Users/francescogiannicola/Scrivania/private_key_found.txt";
    let fallback_path = "/tmp/private_key_found.txt"; // Percorso alternativo

    // Prova a scrivere nel percorso principale
    let result = OpenOptions::new()
        .create(true)
        .write(true)
        .open(file_path)
        .and_then(|mut file| {
            writeln!(file, "CHIAVE VERIFICATA:\nIndirizzo: {}\nPrivate Key: {}\nData e ora: {}", 
                target_address, 
                private_key,
                chrono::Local::now()
            )
        });

    // Se fallisce, prova a scrivere nel percorso alternativo
    if result.is_err() {
        println!("Impossibile scrivere nel file {}. Provo a scrivere in {}", file_path, fallback_path);
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(fallback_path)?;
        writeln!(file, "CHIAVE VERIFICATA:\nIndirizzo: {}\nPrivate Key: {}\nData e ora: {}", 
            target_address, 
            private_key,
            chrono::Local::now()
        )?;
    }

    Ok(())
}

fn main() {
    let secp = Secp256k1::new();
    let target_address = "1MVDYgVaSN6iKKEsbzRUAYFrYJadLYZvvZ";
    
    let start = u128::from_str_radix("80000000000000000", 16).unwrap();
    let end = u128::from_str_radix("fffffffffffffffff", 16).unwrap();
    let total_keys = end - start;

    // Numero di thread da utilizzare (usa il numero di core logici disponibili)
    let num_threads = num_cpus::get();
    
    println!("Iniziando la ricerca nel range:");
    println!("Da:   {:x}", start);
    println!("A:    {:x}", end);
    println!("Utilizzando {} threads", num_threads);

    // Contatori condivisi tra i thread
    let keys_checked = Arc::new(AtomicU64::new(0));
    let found = Arc::new(AtomicU64::new(0));
    
    let start_time = Instant::now();
    let mut handles = vec![];

    // Dividi il range tra i thread
    let chunk_size = (end - start) / num_threads as u128;
    
    for i in 0..num_threads {
        let thread_start = start + (chunk_size * i as u128);
        let thread_end = if i == num_threads - 1 { end } else { thread_start + chunk_size };
        
        let keys_checked = Arc::clone(&keys_checked);
        let found = Arc::clone(&found);
        let target_address = target_address.to_string();

        let handle = thread::spawn(move || {
            let secp = Secp256k1::new();
            let mut local_count = 0;

            for num in thread_start..=thread_end {
                local_count += 1;
                if local_count >= 10000 {
                    keys_checked.fetch_add(local_count, Ordering::Relaxed);
                    local_count = 0;
                }

                let private_key_bytes = num.to_be_bytes();
                
                if let Ok(secret_key) = SecretKey::from_slice(&private_key_bytes) {
                    let private_key = PrivateKey::new(secret_key, Network::Bitcoin);
                    let public_key = PublicKey::from_private_key(&secp, &private_key);
                    let address = Address::p2pkh(&public_key, Network::Bitcoin);
                    
                    if address.to_string() == target_address {
                        // Doppia verifica
                        let verification_public_key = PublicKey::from_private_key(&secp, &private_key);
                        let verification_address = Address::p2pkh(&verification_public_key, Network::Bitcoin);
                        
                        if verification_address.to_string() == target_address {
                            found.store(1, Ordering::Relaxed);
                            
                            // Salva su file
                            if let Err(e) = save_private_key(&target_address, &private_key) {
                                eprintln!("Errore durante il salvataggio della chiave: {}", e);
                            }
                            
                            println!("\nTROVATO! Private Key: {}", private_key);
                            return; // Esci dal thread
                        }
                    }
                }

                if found.load(Ordering::Relaxed) == 1 {
                    return; // Esci dal thread se trovato
                }
            }
        });

        handles.push(handle);
    }

    // Thread separato per mostrare il progresso
    let keys_checked_clone = Arc::clone(&keys_checked);
    let progress_handle = thread::spawn(move || {
        while found.load(Ordering::Relaxed) == 0 {
            let keys = keys_checked_clone.load(Ordering::Relaxed);
            let elapsed = start_time.elapsed();
            let keys_per_second = keys as f64 / elapsed.as_secs_f64();
            let progress = keys as f64 / total_keys as f64 * 100.0;

            print!("\rProgresso: {:.6}% | Chiavi verificate: {} | Velocità: {:.0} k/s | Tempo: {:02}:{:02}:{:02}", 
                progress,
                keys,
                keys_per_second,
                elapsed.as_secs() / 3600,
                (elapsed.as_secs() / 60) % 60,
                elapsed.as_secs() % 60
            );
            std::io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Attendi che tutti i thread finiscano
    for handle in handles {
        handle.join().unwrap();
    }
    progress_handle.join().unwrap();
}
