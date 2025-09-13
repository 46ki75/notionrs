use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub fn verify_signature(verification_token: &[u8], body: &[u8], x_notion_signature: &[u8]) -> bool {
    let mut mac = HmacSha256::new_from_slice(verification_token).expect("key length");
    mac.update(body);
    // x_notion_signature is &[u8], so we need to handle the prefix as bytes
    let tag = if x_notion_signature.starts_with(b"sha256=") {
        &x_notion_signature[7..]
    } else {
        x_notion_signature
    };
    match hex::decode(tag) {
        Ok(tag_bytes) => mac.verify_slice(&tag_bytes).is_ok(),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sign(key: &[u8], msg: &[u8]) -> String {
        let mut mac = HmacSha256::new_from_slice(key).expect("key length");
        mac.update(msg);
        let tag = mac.finalize().into_bytes();
        format!("sha256={}", hex::encode(tag))
    }

    #[test]
    fn verify_success_with_correct_key_and_message() {
        let key = b"test-key";
        let msg = b"important message";
        let tag = sign(key, msg);
        assert!(verify_signature(key, msg, tag.as_bytes()));
    }

    #[test]
    fn verify_fails_with_wrong_message() {
        let key = b"test-key";
        let msg = b"important message";
        let tag = sign(key, msg);
        let wrong_msg = b"tampered message";
        assert!(!verify_signature(key, wrong_msg, tag.as_bytes()));
    }

    #[test]
    fn verify_fails_with_wrong_key() {
        let key = b"test-key";
        let wrong_key = b"other-key";
        let msg = b"important message";
        let tag = sign(key, msg);
        assert!(!verify_signature(wrong_key, msg, tag.as_bytes()));
    }

    #[test]
    fn verify_fails_with_wrong_tag() {
        let key = b"test-key";
        let msg = b"important message";
        let tag = sign(key, msg);
        let mut tag_bytes = hex::decode(tag.strip_prefix("sha256=").unwrap()).unwrap();
        tag_bytes[0] ^= 0xff;
        let wrong_tag = format!("sha256={}", hex::encode(tag_bytes));
        assert!(!verify_signature(key, msg, wrong_tag.as_bytes()));
    }
}
