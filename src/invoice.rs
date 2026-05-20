//! Handle invoice

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::validation::LnValidation;
use crate::Phoenixd;

/// Invoice Request
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceRequest {
    /// Correlation ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// Invoice description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// description Hash
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_hash: Option<String>,
    /// Invoice Amount in sats
    pub amount_sat: u64,
    /// webhook Url
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
}

impl InvoiceRequest {
    /// Create a new validated invoice request
    pub fn new(
        amount_sat: u64,
        external_id: Option<String>,
        description: Option<String>,
        description_hash: Option<String>,
        webhook_url: Option<String>,
    ) -> Result<Self> {
        // Validate all inputs
        LnValidation::validate_amount_sat(amount_sat)?;
        LnValidation::validate_external_id(&external_id)?;
        LnValidation::validate_description(&description)?;
        LnValidation::validate_webhook_url(&webhook_url)?;

        // Validate description hash if provided (should be hex)
        if let Some(ref hash) = description_hash {
            if !hash.is_empty() && !hash.chars().all(|c| c.is_ascii_hexdigit()) {
                bail!("Description hash must be valid hexadecimal");
            }
        }

        Ok(Self {
            external_id,
            description,
            description_hash,
            amount_sat,
            webhook_url,
        })
    }

    /// Validate the invoice request
    pub fn validate(&self) -> Result<()> {
        LnValidation::validate_amount_sat(self.amount_sat)?;
        LnValidation::validate_external_id(&self.external_id)?;
        LnValidation::validate_description(&self.description)?;
        LnValidation::validate_webhook_url(&self.webhook_url)?;

        // Validate description hash if provided
        if let Some(ref hash) = self.description_hash {
            if !hash.is_empty() && !hash.chars().all(|c| c.is_ascii_hexdigit()) {
                bail!("Description hash must be valid hexadecimal");
            }
        }

        Ok(())
    }
}

/// Invoice Response
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceResponse {
    /// Invoice Amount in sat
    pub amount_sat: u64,
    /// Payment Hash
    pub payment_hash: String,
    /// Bolt11
    pub serialized: String,
}

/// Find Incoming Response
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIncomingInvoiceResponse {
    /// Payment Hash
    pub payment_hash: String,
    /// Preimage
    pub preimage: String,
    /// External Id
    pub external_id: Option<String>,
    /// Description
    pub description: String,
    /// Bolt11 invoice
    pub invoice: String,
    /// Paid flag
    pub is_paid: bool,
    /// Sats received
    pub received_sat: u64,
    /// Fees
    pub fees: u64,
    /// Completed at
    pub completed_at: Option<u64>,
    /// Time created
    pub created_at: u64,
}

impl Phoenixd {
    /// Create Invoice with validation
    pub async fn create_invoice(&self, invoice_request: InvoiceRequest) -> Result<InvoiceResponse> {
        // Validate the invoice request before sending to API
        invoice_request.validate()?;

        let url = self.api_url.join("/createinvoice")?;

        let res = self
            .make_post(url, Some(serde_json::to_value(invoice_request)?))
            .await?;

        match serde_json::from_value(res.clone()) {
            Ok(res) => Ok(res),
            Err(_) => {
                log::error!("Api error response on invoice creation");
                log::error!("{}", res);
                bail!("Could not create invoice")
            }
        }
    }

    /// Find incoming invoice with payment hash validation
    pub async fn get_incoming_invoice(
        &self,
        payment_hash: &str,
    ) -> Result<GetIncomingInvoiceResponse> {
        // Validate payment hash format
        LnValidation::validate_payment_hash(payment_hash)?;

        let url = self
            .api_url
            .join(&format!("payments/incoming/{}", payment_hash))?;

        let res = self.make_get(url).await?;

        match serde_json::from_value(res.clone()) {
            Ok(res) => Ok(res),
            Err(err) => {
                log::error!("Api error response on find invoice");
                log::error!("{}", err);
                log::error!("{}", res);
                bail!("Could not find incoming invoice")
            }
        }
    }
}
