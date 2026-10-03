// SPDX-License-Identifier: MPL-2.0

/** GraphQL documents used by read-side screens. */
module GetClaimQuery = {
  let document = Client.parse("query GetClaim($id: String!) { claim(id: $id) { id text platform textHash status } }")
}

module ListClaimsQuery = {
  let document = Client.parse("query ListClaims($skip: Int, $limit: Int) { claims(skip: $skip, limit: $limit) { id text status } }")
}
