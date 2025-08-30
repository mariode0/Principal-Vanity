use rand::RngCore;
use bip39::{Language, Mnemonic};
use bip32::{Seed, XPrv, DerivationPath};
use k256::SecretKey;
use ic_agent::identity::Secp256k1Identity;
use ic_agent::Identity;
use std::str::FromStr;
use std::time::Instant;
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use std::sync::Arc;
use rayon::prelude::*;

fn generate_identity() -> Result<(String, Mnemonic, SecretKey), Box<dyn std::error::Error>> {
    // Generate random entropy (12 words → 128 bit entropy)
    let mut entropy = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut entropy);
    let mnemonic = Mnemonic::from_entropy_in(Language::English, &entropy)?;

    // Create seed from mnemonic (convert to [u8; 64])
    let seed_bytes = mnemonic.to_seed("");
    let seed = Seed::new(seed_bytes);

    // ICP derivation path: m/44'/223'/0'/0/0
    let derivation_path = DerivationPath::from_str("m/44'/223'/0'/0/0")?;

    // Extended Private Key
    let xprv = XPrv::derive_from_path(seed.as_bytes(), &derivation_path)?;

    // Get SecretKey from xprv
    let secret_key = SecretKey::from_bytes(&xprv.private_key().to_bytes())?;

    // Create ICP Identity
    let identity = Secp256k1Identity::from_private_key(secret_key.clone());
    let principal = identity.sender()?.to_text();

    Ok((principal, mnemonic, secret_key))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target_prefix = "aaaaa"; // Change to your desired prefix
    let start_time = Instant::now();
    
    // Shared counters and flags
    let iter_counter = Arc::new(AtomicU64::new(0));
    let found_flag = Arc::new(AtomicBool::new(false));
    
    println!("Searching for ICP Principal with prefix: '{}'", target_prefix);
    println!("Estimated attempts needed: ~{:.0}", 32_f64.powi(target_prefix.len() as i32));
    println!("Using {} CPU threads with Rayon", rayon::current_num_threads());
    println!();

    // Use batch processing with Rayon
    let chunk_size = 10000;
    let mut batch_start = 0u64;
    
    loop {
        // Create a batch of work
        let batch_range: Vec<u64> = (batch_start..batch_start + chunk_size).collect();
        
        // Process batch in parallel
        let result = batch_range.par_iter().find_map_any(|_| {
            // Check if another thread found a match
            if found_flag.load(Ordering::Relaxed) {
                return None;
            }

            let iter = iter_counter.fetch_add(1, Ordering::Relaxed) + 1;
            
            match generate_identity() {
                Ok((principal, mnemonic, secret_key)) => {
                    if principal.starts_with(target_prefix) {
                        // Mark as found to stop other threads
                        found_flag.store(true, Ordering::Relaxed);
                        
                        let elapsed = start_time.elapsed();
                        println!("MATCH FOUND after {} iterations!", iter);
                        println!("Time elapsed: {:.2?}", elapsed);
                        println!("Principal : {}", principal);
                        println!("Mnemonic  : {}", mnemonic);
                        println!("Rate: {:.0} attempts/second", iter as f64 / elapsed.as_secs_f64());
                        
                        return Some((principal, mnemonic, secret_key));
                    }

                    // Progress update every 100k iterations
                    if iter % 100_000 == 0 {
                        let elapsed = start_time.elapsed();
                        let rate = iter as f64 / elapsed.as_secs_f64();
                        println!("[{}] sample: {} | rate: {:.0} attempts/sec", 
                            iter, principal, rate);
                    }
                    
                    None
                }
                Err(e) => {
                    eprintln!("Error generating identity: {}", e);
                    None
                }
            }
        });

        // If found, break
        if result.is_some() {
            println!("Vanity address generation completed successfully!");
            break;
        }

        // If search was stopped by another thread
        if found_flag.load(Ordering::Relaxed) {
            break;
        }

        // Move to next batch
        batch_start += chunk_size;
    }

    Ok(())
}