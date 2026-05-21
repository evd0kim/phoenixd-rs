//! Input validation utilities

use std::str;

use anyhow::{bail, Result};

/// Lightning Network constants and validation rules
pub struct LnValidation;

impl LnValidation {
    /// Minimum amount in satoshis (1 sat)
    pub const MIN_AMOUNT_SAT: u64 = 1;

    /// Maximum amount in satoshis (21M BTC = 21,000,000 * 100,000,000 sats)
    pub const MAX_AMOUNT_SAT: u64 = 21_000_000_000_000_000;

    /// Maximum description length
    pub const MAX_DESCRIPTION_LENGTH: usize = 639; // BOLT-11 limit

    /// Maximum external ID length
    pub const MAX_EXTERNAL_ID_LENGTH: usize = 255;

    /// Maximum message length for BOLT-12
    pub const MAX_MESSAGE_LENGTH: usize = 1024;

    /// Payment hash length in hex (32 bytes = 64 hex chars)
    pub const PAYMENT_HASH_HEX_LENGTH: usize = 64;

    /// Validate amount in satoshis
    pub fn check_amount_sat(amount: u64) -> Result<()> {
        if amount == 0 {
            bail!("Amount cannot be zero");
        }
        if amount < Self::MIN_AMOUNT_SAT {
            bail!("Amount too small: minimum is {} sat", Self::MIN_AMOUNT_SAT);
        }
        if amount > Self::MAX_AMOUNT_SAT {
            bail!("Amount too large: maximum is {} sat", Self::MAX_AMOUNT_SAT);
        }
        Ok(())
    }

    /// Validate optional amount in satoshis
    pub fn check_optional_amount_sat(amount: Option<u64>) -> Result<()> {
        if let Some(amt) = amount {
            Self::check_amount_sat(amt)?;
        }
        Ok(())
    }

    /// Validate description string
    pub fn check_description(description: &Option<String>) -> Result<()> {
        if let Some(desc) = description {
            if desc.len() > Self::MAX_DESCRIPTION_LENGTH {
                bail!(
                    "Description too long: maximum {} characters",
                    Self::MAX_DESCRIPTION_LENGTH
                );
            }
            // Check for null bytes or other problematic characters
            if desc.contains('\0') {
                bail!("Description contains invalid null character");
            }
        }
        Ok(())
    }

    /// Validate external ID string
    pub fn check_external_id(external_id: &Option<String>) -> Result<()> {
        if let Some(id) = external_id {
            if id.is_empty() {
                bail!("External ID cannot be empty");
            }
            if id.len() > Self::MAX_EXTERNAL_ID_LENGTH {
                bail!(
                    "External ID too long: maximum {} characters",
                    Self::MAX_EXTERNAL_ID_LENGTH
                );
            }
            // Check for null bytes or other problematic characters
            if id.contains('\0') {
                bail!("External ID contains invalid null character");
            }
        }
        Ok(())
    }

    /// Validate message string for BOLT-12
    pub fn check_message(message: &Option<String>) -> Result<()> {
        if let Some(msg) = message {
            if msg.len() > Self::MAX_MESSAGE_LENGTH {
                bail!(
                    "Message too long: maximum {} characters",
                    Self::MAX_MESSAGE_LENGTH
                );
            }
            // Check for null bytes or other problematic characters
            if msg.contains('\0') {
                bail!("Message contains invalid null character");
            }
        }
        Ok(())
    }

    /// Validate BOLT-11 invoice format
    pub fn check_bolt11_invoice(invoice: &str) -> Result<()> {
        if invoice.is_empty() {
            bail!("Invoice cannot be empty");
        }

        // BOLT-11 invoices should start with 'ln' followed by currency prefix
        if !invoice.starts_with("ln") {
            bail!("Invalid BOLT-11 invoice format: must start with 'ln'");
        }

        // Check minimum length (a valid invoice should be at least several dozen
        // characters)
        if invoice.len() < 20 {
            bail!("Invoice too short: likely invalid BOLT-11 format");
        }

        // Check maximum reasonable length (invoices shouldn't be extremely long)
        if invoice.len() > 2000 {
            bail!("Invoice too long: exceeds reasonable BOLT-11 length limit");
        }

        // Check for null bytes or other problematic characters
        if invoice.contains('\0') {
            bail!("Invoice contains invalid null character");
        }

        Ok(())
    }

    /// Validate BOLT-12 offer format
    pub fn check_bolt12_offer(offer: &str) -> Result<()> {
        if offer.is_empty() {
            bail!("Offer cannot be empty");
        }

        // BOLT-12 offers should start with 'lno'
        if !offer.starts_with("lno") {
            bail!("Invalid BOLT-12 offer format: must start with 'lno'");
        }

        // Check minimum length
        if offer.len() < 20 {
            bail!("Offer too short: likely invalid BOLT-12 format");
        }

        // Check maximum reasonable length
        if offer.len() > 2000 {
            bail!("Offer too long: exceeds reasonable BOLT-12 length limit");
        }

        Ok(())
    }

    /// Validate payment hash format (64-character hex string)
    pub fn check_payment_hash(payment_hash: &str) -> Result<()> {
        if payment_hash.is_empty() {
            bail!("Payment hash cannot be empty");
        }

        if payment_hash.len() != Self::PAYMENT_HASH_HEX_LENGTH {
            bail!(
                "Payment hash must be exactly {} characters",
                Self::PAYMENT_HASH_HEX_LENGTH
            );
        }

        // Verify it's valid hexadecimal
        if !payment_hash.chars().all(|c| c.is_ascii_hexdigit()) {
            bail!("Payment hash must contain only hexadecimal characters");
        }

        Ok(())
    }

    /// Validate webhook URL format
    pub fn check_webhook_url(webhook_url: &Option<String>) -> Result<()> {
        if let Some(url) = webhook_url {
            if url.is_empty() {
                bail!("Webhook URL cannot be empty");
            }

            // Basic URL validation - must start with http:// or https://
            if !url.starts_with("http://") && !url.starts_with("https://") {
                bail!("Webhook URL must start with http:// or https://");
            }

            // Check reasonable length limit
            if url.len() > 2048 {
                bail!("Webhook URL too long: maximum 2048 characters");
            }

            // Check for null bytes
            if url.contains('\0') {
                bail!("Webhook URL contains invalid null character");
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_amount_sat() {
        // Valid amounts
        assert!(LnValidation::check_amount_sat(1).is_ok());
        assert!(LnValidation::check_amount_sat(100).is_ok());
        assert!(LnValidation::check_amount_sat(1_000_000).is_ok());
        assert!(LnValidation::check_amount_sat(21_000_000_000_000_000).is_ok());

        // Invalid amounts
        assert!(LnValidation::check_amount_sat(0).is_err());
        assert!(LnValidation::check_amount_sat(21_000_000_000_000_001).is_err());
    }

    #[test]
    fn test_check_optional_amount_sat() {
        assert!(LnValidation::check_optional_amount_sat(None).is_ok());
        assert!(LnValidation::check_optional_amount_sat(Some(100)).is_ok());
        assert!(LnValidation::check_optional_amount_sat(Some(0)).is_err());
    }

    #[test]
    fn test_check_description() {
        // Valid descriptions
        assert!(LnValidation::check_description(&None).is_ok());
        assert!(LnValidation::check_description(&Some("Valid description".to_string())).is_ok());

        // Invalid descriptions
        let long_description = "a".repeat(640);
        assert!(LnValidation::check_description(&Some(long_description)).is_err());
        assert!(
            LnValidation::check_description(&Some("Invalid\0description".to_string())).is_err()
        );
    }

    #[test]
    fn test_check_external_id() {
        // Valid external IDs
        assert!(LnValidation::check_external_id(&None).is_ok());
        assert!(LnValidation::check_external_id(&Some("valid-id".to_string())).is_ok());

        // Invalid external IDs
        assert!(LnValidation::check_external_id(&Some("".to_string())).is_err());
        let long_id = "a".repeat(256);
        assert!(LnValidation::check_external_id(&Some(long_id)).is_err());
        assert!(LnValidation::check_external_id(&Some("invalid\0id".to_string())).is_err());
    }

    #[test]
    fn test_check_bolt11_invoice() {
        // Valid BOLT-11 invoices (simplified format)
        let valid_invoice = "lnbc1u1p0xyzabcdefghijklmnopqrstuvwxyz0123456789".to_string();
        assert!(LnValidation::check_bolt11_invoice(&valid_invoice).is_ok());

        // Invalid invoices
        assert!(LnValidation::check_bolt11_invoice("").is_err());
        assert!(LnValidation::check_bolt11_invoice("bc1qxyz").is_err()); // Not BOLT-11
        assert!(LnValidation::check_bolt11_invoice("ln").is_err()); // Too short
    }

    #[test]
    fn test_check_bolt12_offer() {
        // Valid BOLT-12 offers (simplified format)
        let valid_offer = "lnoxyz1234567890abcdefghijklmnopqrstuvwxyzABCDEF".to_string();
        assert!(LnValidation::check_bolt12_offer(&valid_offer).is_ok());

        // Invalid offers
        assert!(LnValidation::check_bolt12_offer("").is_err());
        assert!(LnValidation::check_bolt12_offer("lnbc").is_err()); // Not BOLT-12
        assert!(LnValidation::check_bolt12_offer("lno").is_err()); // Too short
    }

    #[test]
    fn test_check_payment_hash() {
        // Valid payment hash (64 hex chars)
        let valid_hash = "a1b2c3d4e5f67890abcdef1234567890abcdef1234567890abcdef1234567890";
        assert!(LnValidation::check_payment_hash(valid_hash).is_ok());

        // Invalid payment hashes
        assert!(LnValidation::check_payment_hash("").is_err());
        assert!(LnValidation::check_payment_hash("short").is_err());
        assert!(LnValidation::check_payment_hash("invalidhex!@#$").is_err());
        let too_long = "a".repeat(65);
        assert!(LnValidation::check_payment_hash(&too_long).is_err());
    }

    #[test]
    fn test_check_webhook_url() {
        // Valid webhook URLs
        assert!(LnValidation::check_webhook_url(&None).is_ok());
        assert!(LnValidation::check_webhook_url(&Some(
            "https://example.com/webhook".to_string()
        ))
        .is_ok());
        assert!(LnValidation::check_webhook_url(&Some(
            "http://localhost:3000/webhook".to_string()
        ))
        .is_ok());

        // Invalid webhook URLs
        assert!(LnValidation::check_webhook_url(&Some("".to_string())).is_err());
        assert!(
            LnValidation::check_webhook_url(&Some("ftp://example.com".to_string())).is_err()
        );
        assert!(
            LnValidation::check_webhook_url(&Some("https://\0example.com".to_string())).is_err()
        );
    }
}
