// Copyright (C) 2023 Light, Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use serde::{Deserialize, Serialize};

use crate::gas::{GasEstimation, GasEstimationParams};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ApiResponseData {
    slow: u64,
    standard: u64,
    fast: u64,
    rapid: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ApiResponse {
    data: ApiResponseData,
}

impl From<ApiResponseData> for GasEstimation {
    fn from(data: ApiResponseData) -> Self {
        let make_params = |value: u64| -> GasEstimationParams {
            GasEstimationParams {
                max_priority_fee_per_gas: value.into(),
                max_fee_per_gas: value.into(),
            }
        };

        Self {
            low: make_params(data.slow),
            average: make_params(data.standard),
            high: make_params(data.fast),
            instant: make_params(data.rapid),
        }
    }
}

pub async fn ethereum_gas_estimation(chain_id: u64) -> Result<GasEstimation, reqwest::Error> {
    let url = match chain_id {
        1 => "https://beaconcha.in/api/v1/execution/gasnow",
        11155111 => "https://sepolia.beaconcha.in/api/v1/execution/gasnow",
        _ => panic!("Unsupported chain ID"),
    };

    let response = reqwest::get(url).await?.json::<ApiResponse>().await?;

    // Convert to GasEstimation using From trait
    Ok(response.data.into())
}

#[cfg(test)]
mod tests {
    use ethers::types::U256;

    use super::*;

    #[tokio::test]
    async fn test_ethereum_gas_estimation() {
        // Test for mainnet
        let chain_id = 1;
        let result = ethereum_gas_estimation(chain_id).await;
        assert!(result.is_ok());
        let gas_estimation = result.unwrap();
        assert!(gas_estimation.low.max_priority_fee_per_gas > U256::from(0));
        assert!(gas_estimation.low.max_fee_per_gas > U256::from(0));
        assert!(gas_estimation.average.max_priority_fee_per_gas > U256::from(0));
        assert!(gas_estimation.average.max_fee_per_gas > U256::from(0));
        assert!(gas_estimation.high.max_priority_fee_per_gas > U256::from(0));
        assert!(gas_estimation.high.max_fee_per_gas > U256::from(0));
        assert!(gas_estimation.instant.max_priority_fee_per_gas > U256::from(0));
        assert!(gas_estimation.instant.max_fee_per_gas > U256::from(0));
    }
}