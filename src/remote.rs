#![cfg(feature = "remote")]

use crate::err;
use licc::client::error::ClientError;
use std::cmp::min;

pub async fn get_codes(
    client: licc::client::CodesClient,
    mut max_retries: u8,
) -> Result<Vec<licc::Code>, &'static str> {
    // avoid significant server flood
    max_retries = min(5, max_retries);

    for i in 0..max_retries {
        let result = client.get_codes_slim().await;

        if let Err(err) = result {
            handle_client_error(err, i, max_retries);
            tokio::time::sleep(tokio::time::Duration::from_secs(2 * i as u64)).await;
            continue;
        };

        return Ok(result.unwrap());
    }

    Err("Could not resolve codes within max retries")
}

pub fn handle_client_error(err: ClientError, retry_count: u8, max_count: u8) -> &'static str {
    match err {
        ClientError::Reqwest(err) => {
            err!(
                "Failed to retrieve codes: {} ({}/{})",
                err,
                retry_count,
                max_count
            );
            "Failed to retrieve codes (http)"
        }
        ClientError::Serde(err) => {
            err!(
                "Failed to parse codes: {} ({}/{})",
                err,
                retry_count,
                max_count
            );
            "Failed to parse codes (serde)"
        }
        ClientError::ServerError(err) => {
            err!(
                "Failed to retrieve codes (HTTP {}) {} ({}/{})",
                err.error.code,
                err.error.description,
                retry_count,
                max_count
            );
            "Failed to retrieve codes (server error)"
        }
    }
}
