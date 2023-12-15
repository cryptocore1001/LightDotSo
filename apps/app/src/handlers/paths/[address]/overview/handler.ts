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

import { handler as addressHandler } from "@/handlers/paths/[address]/handler";
import { handler as historyHandler } from "@/handlers/paths/[address]/overview/history/handler";
import { handler as nftsHandler } from "@/handlers/paths/[address]/overview/nfts/handler";
import { handler as tokensHandler } from "@/handlers/paths/[address]/overview/tokens/handler";
import { validateAddress } from "@/handlers/validators/address";

// -----------------------------------------------------------------------------
// Handler
// -----------------------------------------------------------------------------

export const handler = async (params: { address: string }) => {
  // ---------------------------------------------------------------------------
  // Validators
  // ---------------------------------------------------------------------------

  validateAddress(params.address);

  // ---------------------------------------------------------------------------
  // Fetch
  // ---------------------------------------------------------------------------

  const addressHandlerPromise = addressHandler(params);
  const tokensHandlerPromise = tokensHandler(params);
  const nftsHandlerPromise = nftsHandler(params);
  const historyHandlerPromise = historyHandler(params);

  const [
    { walletSettings },
    { tokens, tokensCount, portfolio },
    { nfts, nftValuation },
    { transactions, transactionsCount },
  ] = await Promise.all([
    addressHandlerPromise,
    tokensHandlerPromise,
    nftsHandlerPromise,
    historyHandlerPromise,
  ]);

  // ---------------------------------------------------------------------------
  // Parse
  // ---------------------------------------------------------------------------

  return {
    walletSettings: walletSettings,
    tokens: tokens,
    tokensCount: tokensCount,
    portfolio: portfolio,
    nfts: nfts,
    nftValuation: nftValuation,
    transactions: transactions,
    transactionsCount: transactionsCount,
  };
};
