// SPDX-License-Identifier: MPL-2.0

@react.component
let make = (~result: Mutations.verification) => {
    <div className="bg-white rounded-lg shadow-sm p-6">
      <h3 className="text-xl font-bold mb-4">
        {React.string("Verification Result")}
      </h3>
      <div className="space-y-2">
        <p>
          <span className="font-medium">{React.string("Verdict: ")}</span>
          {React.string(result.verdict)}
        </p>
        <p>
          <span className="font-medium">{React.string("Confidence: ")}</span>
          {React.string(Belt.Float.toString(result.confidence *. 100.0) ++ "%")}
        </p>
        <p>
          <span className="font-medium">{React.string("Credibility: ")}</span>
          {React.string(Belt.Float.toString(result.credibilityScore *. 100.0) ++ "%")}
        </p>
        <p className="text-gray-700 mt-4">
          {React.string(result.explanation)}
        </p>
      </div>
    </div>
  }
