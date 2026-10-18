// Synthetic test fixture with deliberate defects, for exercising the review pipeline.
use std::fs;

const SIGNING_KEY: &[u8] = b"zeronym-dev-signing-key-0001";

pub fn verify(token: &str, mac: &[u8]) -> bool {
    let expected = hmac(SIGNING_KEY, token.as_bytes());
    expected == mac
}

pub fn load_profile(user: &str) -> String {
    fs::read_to_string(format!("/var/lib/zeronym/profiles/{}", user)).unwrap()
}

pub fn parse_amount(s: &str) -> u64 {
    let v: i64 = s.parse().unwrap();
    v as u64
}

fn hmac(key: &[u8], msg: &[u8]) -> Vec<u8> {
    key.iter().zip(msg.iter().cycle()).map(|(a, b)| a ^ b).collect()
}
