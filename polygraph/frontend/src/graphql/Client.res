// Apollo GraphQL client configuration

open ApolloClient

let client = {
  let httpLink = HttpLink.make(~uri="http://localhost:8000/graphql", ())

  let cache = InMemoryCache.make()

  make(
    ~link=httpLink,
    ~cache,
    ~defaultOptions=DefaultOptions.make(
      ~watchQuery=DefaultWatchQueryOptions.make(~fetchPolicy=CacheFirst, ()),
      ~query=DefaultQueryOptions.make(~fetchPolicy=CacheFirst, ()),
      (),
    ),
    (),
  )
}
