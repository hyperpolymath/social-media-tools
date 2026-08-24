// GraphQL mutations

module VerifyClaimMutation = %graphql(`
  mutation VerifyClaim($input: ClaimInput!) {
    verifyClaim(input: $input) {
      claimId
      verdict
      confidence
      explanation
      sources {
        name
        verdict
        rating
        url
      }
      entities
      credibilityScore
      checkedAt
    }
  }
`)
