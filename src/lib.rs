use std::collections::HashMap;

// Name Assignment (variables and constants)
// TODO: Assign the current bitcoin mining reward
pub const MINING_REWARD: f64 = 3.125;
// TODO: Assign the current block height
pub const CURRENT_BLOCK_HEIGHT: u64 = 850_000;
// TODO: Assign the number of satoshis in one Bitcoin
pub const BTC_TO_SATS: u64 = 100_000_000;

#[derive(Debug, Clone, PartialEq)]
pub struct Utxo {
    pub txid: String,
    pub vout: u32,
    pub value: u64,
}

/// Calculate the total Bitcoin reward for a given number of mined blocks.
pub fn calculate_total_reward(blocks_mined: u64) -> f64 {
    // TODO: Multiply blocks_mined by MINING_REWARD and return result
    blocks_mined as f64 * MINING_REWARD
}

/// Return true if the transaction fee is between 0.00001 and 0.01 BTC.
pub fn is_valid_tx_fee(fee: f64) -> bool {
    // TODO: Check if fee is between 0.00001 and 0.01 BTC (inclusive)
    fee >= 0.00001 && fee <= 0.01
}

/// Return true if the wallet balance is greater than 50.0 BTC.
pub fn is_large_balance(balance: f64) -> bool {
    // TODO: Compare balance to 50.0 and return result
    balance > 50.0
}

/// Return the priority of a transaction ("high", "medium", "low") based on fee rate.
pub fn tx_priority(size_bytes: u64, fee_btc: f64) -> &'static str {
    // TODO: Calculate fee rate (fee_btc / size_bytes) and use if/else if/else
    // High: > 0.00005, Medium: > 0.00001, otherwise Low
    if size_bytes == 0 {
        return "low";
    } 

    let fee_rate = fee_btc / size_bytes as f64;

    if fee_rate > 0.00005 {
        "high"
    } else if fee_rate > 0.00001 {
        "medium"
    } else {
        "low" 
    }
}

/// Return true if the network string equals "mainnet" (case-insensitive).

pub fn is_mainnet(network: &str) -> bool {
    // TODO: Convert network to lowercase and compare with "mainnet"
    network.to_lowercase() == "mainnet"
}

/// Return true if value is in the inclusive range 100..=200.
pub fn is_in_range(value: i64) -> bool {
    // TODO: Check if 100 <= value <= 200
    value >= 100 && value <= 200
}

/// Return true if both references point to the exact same object in memory.
pub fn is_same_wallet<T>(wallet1: &T, wallet2: &T) -> bool {
    // TODO: Use std::ptr::eq to compare reference identity
    std::ptr::eq(wallet1, wallet2)
}

/// Normalize a Bitcoin address by trimming whitespace and lowercasing.
pub fn normalize_address(address: &str) -> String {
    // TODO: Trim leading/trailing whitespace and convert to lowercase
    address.trim().to_lowercase()
}

/// Append a new UTXO to the list and return the updated list.
pub fn add_utxo(mut utxos: Vec<Utxo>, new_utxo: Utxo) -> Vec<Utxo> {
    // TODO: Push new_utxo into utxos and return it
    utxos.push(new_utxo);
    utxos
}

/// Find the first transaction with a fee greater than 0.005 BTC.
pub fn find_high_fee(fee_list: &[f64]) -> Option<(usize, f64)> {
    // TODO: Iterate with enumerate and return the first (index, fee) where fee > 0.005
    for (index, fee) in fee_list.iter().enumerate(){
        if *fee > 0.005 {
            return Some((index, *fee));
        }
    }
    None
}

/// Return basic wallet details as a tuple of (name, balance).
pub fn get_wallet_details() -> (String, f64) {
    // TODO: Return a tuple with wallet name and balance
    let basic_wallet: (String, f64) = (name, balance);

    let name: &str = "Devfoma Wallet";
    let balance = 100.00;

    return basic_wallet
}

/// Get the status of a transaction from the mempool or "not found".
pub fn get_tx_status(tx_pool: &HashMap<String, String>, txid: &str) -> String {
    // TODO: Look up txid in tx_pool, returning the status or "not found"
    tx_pool
        .get(txid)
        .cloned()
        .unwrap_or("not found".to_string())
}

/// Destructure wallet_info and format a status string.
pub fn unpack_wallet_info(wallet_info: (String, f64)) -> String {
    // TODO: Destructure the tuple into (name, balance) and format the result
    // Expected format: "Wallet <name> has balance: <balance> BTC"
    let wallet_info: (String, f64) = (name, balance);

    let name = "devfoma";
    let balance = 100_000_000_000.00;

    format!("Wallet {} has balance: {} BTC", name, balance)
}

/// Convert BTC to satoshis (1 BTC = 100,000,000 sats).
pub fn calculate_sats(btc: f64) -> u64 {
    (btc * BTC_TO_SATS as f64) as u64
}

/// Generate a mock Bitcoin address of length 32 with the given prefix.
pub fn generate_address(prefix: &str) -> String {
    let charset = "abcdefghijklmnopqrstuvwxyz0123456789";
    let mut suffix = String::new();

    let remaining = 32usize.saturating_sub(prefix.len());

    for i in 0..remaining {
        let ch = charset.chars().nth(i % charset.len()).unwrap();
        suffix.push(ch);
    }

    format!("{}{}", prefix, suffix)
}

/// Validate a Bitcoin block height. Returns (is_valid, message).
pub fn validate_block_height(height: i64) -> (bool, String) {
    if height < 0 {
        return (false, "Block height cannot be negative".to_string());
    }

    if height > 1_000_000 {
        return (false, "Block height exceeds realistic range".to_string());
    }

    (true, "Valid block height".to_string())
}

/// Compute the block reward (in sats) for each block height based on the halving schedule.
pub fn halving_schedule(blocks: &[u64]) -> HashMap<u64, u64> {
    let mut rewards = HashMap::new();

    // 50 BTC in satoshis
    let base_reward: u64 = 50 * 100_000_000;

    for &block in blocks {
        let halvings = block / 210_000;

        // Right shift halves the reward
        let reward = base_reward >> halvings;

        rewards.insert(block, reward);
    }

    rewards
}

/// Find the UTXO with the smallest value that meets or exceeds target.
pub fn find_utxo_with_min_value(utxos: &[Utxo], target: u64) -> Option<Utxo> {
    utxos
        .iter()
        .filter(|u| u.value >= target)
        .min_by_key(|u| u.value)
        .cloned()
}

/// Create a UTXO map from txid, vout, and arbitrary extra string fields.
pub fn create_utxo(
    txid: &str,
    vout: u32,
    extra: HashMap<String, String>,
) -> HashMap<String, String> {
    let mut utxo_map = HashMap::new();

    utxo_map.insert("txid".to_string(), txid.to_string());
    utxo_map.insert("vout".to_string(), vout.to_string());

    // Merge extra fields
    for (key, value) in extra {
        utxo_map.insert(key, value);
    }

    utxo_map
}

// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    // Bitcoin tx version is first 4 bytes (8 hex chars)

    if raw_tx_hex.len() < 8 {
        return Err("Transaction hex too short".to_string());
    }

    let version_hex = &raw_tx_hex[0..8];

    match u32::from_str_radix(version_hex, 16) {
        Ok(version) => Ok(version),
        Err(_) => Err("Invalid transaction hex".to_string()),
    }
}