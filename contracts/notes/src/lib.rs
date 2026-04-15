#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data yang menyimpan catatan trading / crypto journal
#[contracttype]
#[derive(Clone, Debug)]
pub struct JournalEntry {
    id: u64, 
    token_pair: String,  // Contoh: "BTC/USDT" atau "MEME/XLM"
    trade_type: String,  // Contoh: "BUY" atau "SELL"
    notes: String,       // Contoh: "Entry saat EMA cross, antisipasi pantulan Bollinger Band"
}

// Storage key maksimal 9 karakter untuk symbol_short
const JRNL_DATA: Symbol = symbol_short!("JRNL_DATA");

#[contract]
pub struct CryptoJournalContract;

#[contractimpl]
impl CryptoJournalContract {
    // Fungsi untuk mendapatkan semua journal entry
    pub fn get_entries(env: Env) -> Vec<JournalEntry> {
        env.storage().instance().get(&JRNL_DATA).unwrap_or(Vec::new(&env))
    }

    // Fungsi untuk menambah jurnal baru
    pub fn add_entry(env: Env, token_pair: String, trade_type: String, notes: String) -> String {
        let mut entries: Vec<JournalEntry> = env.storage().instance().get(&JRNL_DATA).unwrap_or(Vec::new(&env));
        
        // Buat object entry baru
        let entry = JournalEntry {
            id: env.prng().gen::<u64>(),
            token_pair,
            trade_type,
            notes,
        };
        
        entries.push_back(entry);
        
        // Simpan ke storage
        env.storage().instance().set(&JRNL_DATA, &entries);
        
        String::from_str(&env, "Crypto journal entry added successfully")
    }

    // Fungsi untuk menghapus jurnal berdasarkan ID
    pub fn delete_entry(env: Env, id: u64) -> String {
        let mut entries: Vec<JournalEntry> = env.storage().instance().get(&JRNL_DATA).unwrap_or(Vec::new(&env));

        for i in 0..entries.len() {
            if entries.get(i).unwrap().id == id {
                entries.remove(i);
                env.storage().instance().set(&JRNL_DATA, &entries);
                return String::from_str(&env, "Journal entry deleted successfully");
            }
        }

        String::from_str(&env, "Journal entry not found")
    }
}
mod test;