// SPDX-License-Identifier: MPL-2.0

/**
 * Claim verification UI.
 *
 * The mutation hook is created once for the component and receives its
 * variables at submit time. Apollo owns request state; this component owns
 * the domain result and the user-facing validation/network errors.
 */
@react.component
let make = () => {
    let (claimText, setClaimText) = React.useState(() => "")
    let (result, setResult) = React.useState(() => None)
    let (error, setError) = React.useState(() => None)
    let (runMutation, mutationState): Client.mutation<Mutations.variables, Mutations.response> = Client.useMutation(
      Mutations.VerifyClaimMutation.document,
      {client: Client.client},
    )

    let handleSubmit = _evt => {
      if claimText == "" {
        setResult(_ => None)
        setError(_ => Some("Enter a claim before verifying it."))
      } else {
        setError(_ => None)
        let variables: Mutations.variables = {input: {text: claimText}}
        runMutation(variables)
        ->Promise.then(response => {
          switch response.data {
          | Some(payload) => {
              setResult(_ => Some(payload.verifyClaim))
              Promise.resolve()
            }
          | None => {
              setError(_ => Some("The verification service returned no result."))
              Promise.resolve()
            }
          }
        })
        ->Promise.catch(_ => {
          setError(_ => Some("The verification service could not be reached. Try again."))
          Promise.resolve()
        })
        ->ignore
      }
    }

    <div className="max-w-4xl mx-auto">
      <div className="bg-white rounded-lg shadow-sm p-6 mb-8">
        <h2 className="text-2xl font-bold mb-4">
          {React.string("Verify a Claim")}
        </h2>

        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              {React.string("Claim Text")}
            </label>
            <textarea
              className="w-full min-h-[100px] px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent"
              placeholder="Enter the claim you want to verify..."
              value={claimText}
              onChange={evt => {
                let value = ReactEvent.Form.target(evt)["value"]
                setClaimText(_ => value)
              }}
            />
          </div>

          {switch error {
          | Some(message) => <p className="text-red-600" role="alert">{React.string(message)}</p>
          | None => React.null
          }}

          <button
            className="w-full bg-primary-600 text-white px-6 py-3 rounded-lg font-medium hover:bg-primary-700 disabled:opacity-50"
            disabled={mutationState.loading}
            onClick={handleSubmit}
          >
            {React.string(mutationState.loading ? "Verifying..." : "Verify Claim")}
          </button>
        </div>
      </div>

      {switch result {
      | Some(res) => <VerificationResult result={res} />
      | None => React.null
      }}
    </div>
  }
