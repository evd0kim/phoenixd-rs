//! Pay Ln

use anyhow::{bail, Result};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::Phoenixd;

/// Pay Invoice Request
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PayInvoiceRequest {
    /// Amount in sats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_sat: Option<u64>,
    /// Bolt11 Invoice
    pub invoice: String,
}

///Pay bolt12 offer
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PayBolt12Request {
    /// Amount in sats
    pub amount_sat: u64,
    /// Bolt12 offer
    pub offer: String,
    /// Message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Pay Invoice Response
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PayInvoiceResponse {
    /// Amount recipient was paid
    pub recipient_amount_sat: u64,
    /// Routing fee paid
    pub routing_fee_sat: u64,
    /// Payment Id
    pub payment_id: String,
    /// Payment hash
    pub payment_hash: String,
    /// Payment preimage
    pub payment_preimage: String,
}

/// Pay a Lightning Address.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PayLnAddressRequest {
    /// Amount in sats.
    pub amount_sat: u64,
    /// Lightning address.
    pub address: String,
    /// Optional payer message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}


/// Outgoing payment shape returned by phoenixd.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutgoingPaymentResponse {
    /// Payment subtype.
    pub sub_type: String,
    /// Payment id (uuid).
    pub payment_id: String,
    /// Payment hash for lightning payments.
    pub payment_hash: Option<String>,
    /// Preimage for successful lightning payments.
    pub preimage: Option<String>,
    /// On-chain tx id for on-chain payments.
    pub tx_id: Option<String>,
    /// Paid flag.
    pub is_paid: bool,
    /// Sent amount in sats.
    pub sent: u64,
    /// Fees in millisats.
    pub fees: u64,
    /// Optional invoice.
    pub invoice: Option<String>,
    /// Completion timestamp (ms).
    pub completed_at: Option<u64>,
    /// Creation timestamp (ms).
    pub created_at: u64,
}

/// Optional filters for list outgoing payments.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListOutgoingPaymentsRequest {
    /// From timestamp (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<u64>,
    /// To timestamp (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<u64>,
    /// Max number of items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    /// Offset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    /// Include all.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
}

impl Phoenixd {
    /// PayInvoice
    pub async fn pay_bolt11_invoice(
        &self,
        invoice: &str,
        amount_sat: Option<u64>,
    ) -> anyhow::Result<PayInvoiceResponse> {
        let url = self.api_url.join("/payinvoice")?;

        let request = PayInvoiceRequest {
            amount_sat,
            invoice: invoice.to_string(),
        };

        let res = self.make_post(url, Some(request)).await?;

        match serde_json::from_value(res.clone()) {
            Ok(res) => Ok(res),
            Err(_) => {
                log::error!("Api error response on payment quote execution");
                log::error!("{}", res);
                bail!("Could not execute payment quote")
            }
        }
    }

    /// Pay offer
    pub async fn pay_bolt12_offer(
        &self,
        offer: String,
        amount_sat: u64,
        message: Option<String>,
    ) -> anyhow::Result<PayInvoiceResponse> {
        let url = self.api_url.join("/payoffer")?;

        let request = PayBolt12Request {
            amount_sat,
            offer,
            message,
        };

        let res = self.make_post(url, Some(request)).await?;

        match serde_json::from_value(res.clone()) {
            Ok(res) => Ok(res),
            Err(_) => {
                log::error!("Api error response on payment quote execution");
                log::error!("{}", res);
                bail!("Could not execute payment quote")
            }
        }
    }

    /// Pay a Lightning Address.
    pub async fn pay_ln_address(&self, request: PayLnAddressRequest) -> Result<PayInvoiceResponse> {
        let url = self.api_url.join("/paylnaddress")?;
        Ok(serde_json::from_value(
            self.make_post(url, Some(request)).await?,
        )?)
    }

    /// Get an outgoing payment by payment hash.
    pub async fn get_outgoing_payment_by_hash(
        &self,
        payment_hash: &str,
    ) -> Result<Option<OutgoingPaymentResponse>> {
        let url = self
            .api_url
            .join(&format!("/payments/outgoingbyhash/{}", payment_hash))?;

        let response = self
            .client
            .get(url)
            .basic_auth("", Some(&self.api_password))
            .send()
            .await?;

        if response.status() == StatusCode::NO_CONTENT {
            return Ok(None);
        }

        let response = response.error_for_status()?;
        Ok(Some(serde_json::from_value(response.json().await?)?))
    }


    /// List outgoing payments.
    pub async fn list_outgoing_payments(
        &self,
        request: ListOutgoingPaymentsRequest,
    ) -> Result<Vec<OutgoingPaymentResponse>> {
        let mut url = self.api_url.join("/payments/outgoing")?;
        {
            let mut query = url.query_pairs_mut();
            if let Some(from) = request.from {
                query.append_pair("from", &from.to_string());
            }
            if let Some(to) = request.to {
                query.append_pair("to", &to.to_string());
            }
            if let Some(limit) = request.limit {
                query.append_pair("limit", &limit.to_string());
            }
            if let Some(offset) = request.offset {
                query.append_pair("offset", &offset.to_string());
            }
            if let Some(all) = request.all {
                query.append_pair("all", &all.to_string());
            }
        }

        Ok(serde_json::from_value(self.make_get(url).await?)?)
    }

    /// Get an outgoing payment by UUID.
    pub async fn get_outgoing_payment_by_uuid(
        &self,
        payment_id: &str,
    ) -> Result<Option<OutgoingPaymentResponse>> {
        let url = self
            .api_url
            .join(&format!("/payments/outgoing/{}", payment_id))?;

        let response = self
            .client
            .get(url)
            .basic_auth("", Some(&self.api_password))
            .send()
            .await?;

        if response.status() == StatusCode::NO_CONTENT {
            return Ok(None);
        }

        let response = response.error_for_status()?;
        Ok(Some(serde_json::from_value(response.json().await?)?))
    }
}
