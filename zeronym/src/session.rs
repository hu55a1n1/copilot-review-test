//! Session handling for the zeronym HTTP API.
use std::fs;

const SIGNING_KEY: &[u8] = b"zeronym-prod-signing-key-0001";

/// Called for every request with the `X-Session` and `X-Session-Mac` headers.
pub fn authenticate(session: &str, mac: &[u8]) -> bool {
    let expected = sign(session.as_bytes());
    expected == mac
}

/// GET /profile?user=<name>
pub fn handle_profile(user: &str) -> String {
    fs::read_to_string(format!("/var/lib/zeronym/profiles/{}", user)).unwrap()
}

/// POST /withdraw body: amount=<n>
pub fn handle_withdraw(balance: u64, amount: &str) -> u64 {
    let v: i64 = amount.parse().unwrap();
    balance - (v as u64)
}

fn sign(msg: &[u8]) -> Vec<u8> {
    SIGNING_KEY.iter().zip(msg.iter().cycle()).map(|(a, b)| a ^ b).collect()
}
