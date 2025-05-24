// src/kyc.rs

/// Simulates a basic KYC validation check.
///
/// # Arguments
/// * `legal_name` - The legal name of the individual or entity.
/// * `wallet_address` - The blockchain wallet address.
/// * `signature_hash` - A signature or hash representing consent or verification.
///
/// # Returns
/// * `Ok(())` if validation passes.
/// * `Err(String)` with an error message if validation fails.
pub fn simulate_kyc_validation(
    legal_name: &str,
    wallet_address: &str,
    signature_hash: &str,
) -> Result<(), String> {
    if legal_name.trim().is_empty() {
        return Err("Legal name cannot be empty.".to_string());
    }

    if wallet_address.trim().is_empty() {
        return Err("Wallet address cannot be empty.".to_string());
    }

    if signature_hash.trim().is_empty() {
        return Err("Signature or hash cannot be empty.".to_string());
    }

    // Basic wallet address format check (e.g., Ethereum address)
    if !wallet_address.starts_with("0x") || wallet_address.len() != 42 {
        return Err("Invalid wallet address format. Expected '0x' prefix and 42 characters total.".to_string());
    }

    // Optionally, add more sophisticated checks for wallet_address characters (e.g., hex)
    // For now, the prefix and length check is sufficient as per requirements.

    Ok(())
}

// Example usage (can be part of tests or examples elsewhere)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kyc_validation_success() {
        assert_eq!(
            simulate_kyc_validation(
                "Alice Wonderland",
                "0x1234567890abcdef1234567890abcdef12345678",
                "some_signature_hash"
            ),
            Ok(())
        );
    }

    #[test]
    fn test_kyc_empty_legal_name() {
        assert_eq!(
            simulate_kyc_validation(
                " ",
                "0x1234567890abcdef1234567890abcdef12345678",
                "some_signature_hash"
            ),
            Err("Legal name cannot be empty.".to_string())
        );
    }

    #[test]
    fn test_kyc_empty_wallet_address() {
        assert_eq!(
            simulate_kyc_validation("Bob The Builder", "  ", "some_signature_hash"),
            Err("Wallet address cannot be empty.".to_string())
        );
    }

    #[test]
    fn test_kyc_empty_signature_hash() {
        assert_eq!(
            simulate_kyc_validation(
                "Charlie Brown",
                "0x1234567890abcdef1234567890abcdef12345678",
                ""
            ),
            Err("Signature or hash cannot be empty.".to_string())
        );
    }

    #[test]
    fn test_kyc_invalid_wallet_address_prefix() {
        assert_eq!(
            simulate_kyc_validation(
                "David Copperfield",
                "1x1234567890abcdef1234567890abcdef12345678", // Invalid prefix
                "some_signature_hash"
            ),
            Err("Invalid wallet address format. Expected '0x' prefix and 42 characters total.".to_string())
        );
    }

    #[test]
    fn test_kyc_invalid_wallet_address_length_short() {
        assert_eq!(
            simulate_kyc_validation(
                "Eve Online",
                "0x12345", // Too short
                "some_signature_hash"
            ),
            Err("Invalid wallet address format. Expected '0x' prefix and 42 characters total.".to_string())
        );
    }

    #[test]
    fn test_kyc_invalid_wallet_address_length_long() {
        assert_eq!(
            simulate_kyc_validation(
                "Frank Herbert",
                "0x1234567890abcdef1234567890abcdef1234567890ab", // Too long
                "some_signature_hash"
            ),
            Err("Invalid wallet address format. Expected '0x' prefix and 42 characters total.".to_string())
        );
    }
}
