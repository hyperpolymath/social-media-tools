// GraphQL queries

module GetClaimQuery = %graphql(`
  query GetClaim($id: String!) {
    claim(id: $id) {
      id
      text
      url
      platform
      author
      textHash
      createdAt
      updatedAt
      status
    }
  }
`)

module ListClaimsQuery = %graphql(`
  query ListClaims($skip: Int, $limit: Int) {
    claims(skip: $skip, limit: $limit) {
      id
      text
      status
      createdAt
    }
  }
`)
