// SPDX-License-Identifier: MPL-2.0

/**
 * The single Apollo client used by the application.
 *
 * The endpoint is deliberately relative. A browser must not call
 * localhost:8000: that address points at the user's machine, not the
 * deployment running the page. The reverse proxy (or the local dev proxy)
 * owns the routing to the GraphQL service.
 */
type t
  type link
  type cache
  type documentNode

  type httpLinkOptions = {uri: string}
  type clientOptions = {link: link, cache: cache}
  type providerProps = {client: t, children: React.element}

  type mutationOptions = {client: t}
  type mutationResult<'data> = {
    loading: bool,
    data: option<'data>,
    error: option<JSON.t>,
  }
  type fetchResult<'data> = {data: option<'data>}
  type mutation<'variables, 'data> = (
    'variables => Promise.t<fetchResult<'data>>,
    mutationResult<'data>,
  )

  @module("@apollo/client") @new
  external makeHttpLink: httpLinkOptions => link = "HttpLink"

  @module("@apollo/client") @new
  external makeInMemoryCache: unit => cache = "InMemoryCache"

  @module("@apollo/client") @new
  external makeApolloClient: clientOptions => t = "ApolloClient"

  @module("graphql")
  external parse: string => documentNode = "parse"

  @module("@apollo/client")
  external useMutation: (documentNode, mutationOptions) => mutation<'variables, 'data> = "useMutation"

  @module("@apollo/client") @react.component
  external provider: (~client: t, ~children: React.element) => React.element = "ApolloProvider"

let client = {
  let link = makeHttpLink({uri: "/graphql"})
  let cache = makeInMemoryCache()
  makeApolloClient({link: link, cache: cache})
}
