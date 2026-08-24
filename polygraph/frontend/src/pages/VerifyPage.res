// Verify claim page

module VerifyPage = {
  @react.component
  let make = () => {
    let (claimText, setClaimText) = React.useState(() => "")
    let (result, setResult) = React.useState(() => None)
    let (loading, setLoading) = React.useState(() => false)

    let handleSubmit = _evt => {
      setLoading(_ => true)

      // TODO: Call GraphQL mutation via Apollo Client
      Js.Promise.resolve()
      ->Promise.thenResolve(_ => {
        setLoading(_ => false)
        setResult(_ => Some({
          "verdict": "unverifiable",
          "confidence": 0.5,
          "explanation": "Verification complete"
        }))
      })
      ->ignore
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

          <button
            className="w-full bg-primary-600 text-white px-6 py-3 rounded-lg font-medium hover:bg-primary-700 disabled:opacity-50"
            disabled={loading}
            onClick={handleSubmit}
          >
            {React.string(loading ? "Verifying..." : "Verify Claim")}
          </button>
        </div>
      </div>

      {switch result {
      | Some(res) => <VerificationResult result={res} />
      | None => React.null
      }}
    </div>
  }
}

module VerificationResult = {
  @react.component
  let make = (~result) => {
    <div className="bg-white rounded-lg shadow-sm p-6">
      <h3 className="text-xl font-bold mb-4">
        {React.string("Verification Result")}
      </h3>
      <div className="space-y-2">
        <p>
          <span className="font-medium">{React.string("Verdict: ")}</span>
          {React.string(result["verdict"])}
        </p>
        <p>
          <span className="font-medium">{React.string("Confidence: ")}</span>
          {React.string(Belt.Float.toString(result["confidence"] *. 100.0) ++ "%")}
        </p>
        <p className="text-gray-700 mt-4">
          {React.string(result["explanation"])}
        </p>
      </div>
    </div>
  }
}
