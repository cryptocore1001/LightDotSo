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

use super::types::Paymaster;
use crate::{
    error::RouteError, result::AppJsonResult, routes::paymaster::error::PaymasterError,
    state::AppState,
};
use autometrics::autometrics;
use axum::{
    extract::{Query, State},
    Json,
};
use lightdotso_prisma::paymaster;
use lightdotso_tracing::tracing::info;
use serde::Deserialize;
use utoipa::IntoParams;

// -----------------------------------------------------------------------------
// Query
// -----------------------------------------------------------------------------

#[derive(Debug, Deserialize, Default, IntoParams)]
#[serde(rename_all = "snake_case")]
#[into_params(parameter_in = Query)]
pub struct GetQuery {
    pub id: String,
}

// -----------------------------------------------------------------------------
// Handler
// -----------------------------------------------------------------------------

/// Get a paymaster
#[utoipa::path(
        get,
        path = "/paymaster/get",
        params(
            GetQuery
        ),
        responses(
            (status = 200, description = "Paymaster returned successfully", body = Paymaster),
            (status = 404, description = "Paymaster not found", body = PaymasterError),
        )
    )]
#[autometrics]
pub(crate) async fn v1_paymaster_get_handler(
    get_query: Query<GetQuery>,
    State(state): State<AppState>,
) -> AppJsonResult<Paymaster> {
    // Get the get query.
    let Query(query) = get_query;

    info!("Get paymaster for address: {:?}", query);

    // Get the paymasters from the database.
    let paymaster =
        state.client.paymaster().find_unique(paymaster::id::equals(query.id)).exec().await?;

    // If the paymaster is not found, return a 404.
    let paymaster = paymaster.ok_or(RouteError::PaymasterError(PaymasterError::NotFound(
        "Paymaster not found".to_string(),
    )))?;

    // Change the paymaster to the format that the API expects.
    let paymaster: Paymaster = paymaster.into();

    Ok(Json::from(paymaster))
}
