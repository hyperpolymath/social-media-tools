// SPDX-License-Identifier: MPL-2.0

/**
 * The verify operation is kept next to the Apollo document so the operation
 * and the UI cannot drift apart. These fields are the fields exposed by the
 * backend VerificationResult type; requesting fields from the old REST shape
 * makes GraphQL reject the entire operation before the resolver runs.
 */
module VerifyClaimMutation = {
  let document = Client.parse("mutation VerifyClaim($input: ClaimInput!) { verifyClaim(input: $input) { claimId verdict confidence explanation credibilityScore } }")
}

type input = {text: string}
type variables = {input: input}
type verification = {
  claimId: string,
  verdict: string,
  confidence: float,
  explanation: string,
  credibilityScore: float,
}
type response = {verifyClaim: verification}
