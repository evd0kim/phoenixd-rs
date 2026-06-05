//! Main Lightning API methods.

use anyhow::Result;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::Phoenixd;

/// Channel details from `getinfo`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelInfo {
    /// Channel state.
    pub state: String,
    /// Channel id.
    pub channel_id: Option<String>,
    /// Spendable balance in sats.
    pub balance_sat: Option<u64>,
    /// Inbound liquidity in sats.
    pub inbound_liquidity_sat: Option<u64>,
    /// Channel capacity in sats.
    pub capacity_sat: Option<u64>,
    /// Funding transaction id.
    pub funding_tx_id: Option<String>,
}

/// Node info response from `getinfo`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNodeInfoResponse {
    /// Lightning node id.
    pub node_id: String,
    /// Active and historical channels.
    pub channels: Vec<ChannelInfo>,
    /// Chain name.
    pub chain: String,
    /// Current known block height.
    pub block_height: Option<u64>,
    /// phoenixd version.
    pub version: String,
}

/// Balance response from `getbalance`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceResponse {
    /// Available outbound balance in sats.
    pub balance_sat: u64,
    /// Fee credit in sats.
    pub fee_credit_sat: u64,
}

/// Liquidity fee estimate response from `estimateliquidityfees`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EstimateLiquidityFeesResponse {
    /// Estimated mining fee in sats.
    pub mining_fee_sat: u64,
    /// Estimated service fee in sats.
    pub service_fee_sat: u64,
}

/// Optional filters for list incoming payments.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListIncomingPaymentsRequest {
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
    /// Include all (including unpaid / failed where relevant).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    /// Filter by external id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
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

/// Incoming payment shape returned by phoenixd.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IncomingPaymentResponse {
    /// Payment type.
    pub sub_type: String,
    /// Payment hash.
    pub payment_hash: String,
    /// Payment preimage.
    pub preimage: String,
    /// Optional external id.
    pub external_id: Option<String>,
    /// Optional description.
    pub description: Option<String>,
    /// Optional invoice string.
    pub invoice: Option<String>,
    /// Paid flag.
    pub is_paid: bool,
    /// Expired flag.
    pub is_expired: bool,
    /// Requested amount in sats.
    pub requested_sat: Option<u64>,
    /// Received amount in sats.
    pub received_sat: u64,
    /// Fees in millisats.
    pub fees: u64,
    /// Optional payer note.
    pub payer_note: Option<String>,
    /// Optional payer pubkey.
    pub payer_key: Option<String>,
    /// Invoice expiry timestamp (ms).
    pub expires_at: Option<u64>,
    /// Completion timestamp (ms).
    pub completed_at: Option<u64>,
    /// Creation timestamp (ms).
    pub created_at: u64,
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

/// Create offer request body.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOfferRequest {
    /// Amount in sats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_sat: Option<u64>,
    /// Offer description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl Phoenixd {
    /// Fetch node info.
    pub async fn get_node_info(&self) -> Result<GetNodeInfoResponse> {
        let url = self.api_url.join("/getinfo")?;
        Ok(serde_json::from_value(self.make_get(url).await?)?)
    }

    /// Fetch spendable balance and fee credit.
    pub async fn get_balance(&self) -> Result<GetBalanceResponse> {
        let url = self.api_url.join("/getbalance")?;
        Ok(serde_json::from_value(self.make_get(url).await?)?)
    }

    /// Estimate liquidity fees for a target amount.
    pub async fn estimate_liquidity_fees(
        &self,
        amount_sat: u64,
    ) -> Result<EstimateLiquidityFeesResponse> {
        let mut url = self.api_url.join("/estimateliquidityfees")?;
        url.query_pairs_mut()
            .append_pair("amountSat", &amount_sat.to_string());
        Ok(serde_json::from_value(self.make_get(url).await?)?)
    }

    /// List channels as raw JSON values.
    pub async fn list_channels(&self) -> Result<Vec<Value>> {
        let url = self.api_url.join("/listchannels")?;
        Ok(serde_json::from_value(self.make_get(url).await?)?)
    }

    /// Create a bolt12 offer.
    pub async fn create_offer(&self, create_offer: CreateOfferRequest) -> Result<String> {
        let url = self.api_url.join("/createoffer")?;
        let res = self
            .client
            .post(url)
            .basic_auth("", Some(&self.api_password))
            .form(&create_offer)
            .send()
            .await?
            .text()
            .await?;
        Ok(res)
    }

    /// Get the default bolt12 offer.
    pub async fn get_offer(&self) -> Result<String> {
        let url = self.api_url.join("/getoffer")?;
        let res = self
            .client
            .get(url)
            .basic_auth("", Some(&self.api_password))
            .send()
            .await?
            .text()
            .await?;
        Ok(res)
    }

    /// Get the node Lightning Address.
    pub async fn get_ln_address(&self) -> Result<String> {
        let url = self.api_url.join("/getlnaddress")?;
        let res = self
            .client
            .get(url)
            .basic_auth("", Some(&self.api_password))
            .send()
            .await?
            .text()
            .await?;
        Ok(res)
    }

    /// List incoming payments.
    pub async fn list_incoming_payments(
        &self,
        request: ListIncomingPaymentsRequest,
    ) -> Result<Vec<IncomingPaymentResponse>> {
        let mut url = self.api_url.join("/payments/incoming")?;
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
            if let Some(external_id) = request.external_id {
                query.append_pair("externalId", &external_id);
            }
        }

        Ok(serde_json::from_value(self.make_get(url).await?)?)
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

    /// Decode a bolt11 invoice.
    pub async fn decode_invoice(&self, invoice: &str) -> Result<Value> {
        let url = self.api_url.join("/decodeinvoice")?;
        let response = self
            .make_post(
                url,
                Some(serde_json::json!({
                    "invoice": invoice,
                })),
            )
            .await?;
        Ok(response)
    }

    /// Decode a bolt12 offer.
    pub async fn decode_offer(&self, offer: &str) -> Result<Value> {
        let url = self.api_url.join("/decodeoffer")?;
        let response = self
            .make_post(
                url,
                Some(serde_json::json!({
                    "offer": offer,
                })),
            )
            .await?;
        Ok(response)
    }
}
