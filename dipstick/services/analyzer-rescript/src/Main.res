// NUJ Analyzer Service - ReScript + Deno + WASM
// Replaces Python analyzer with type-safe functional approach

open Promise

module WasmNlp = {
  // WASM-optimized NLP operations for performance-critical analysis
  type t

  @module("./wasm/nlp_wasm.js")
  external init: unit => Promise.t<t> = "init"

  @module("./wasm/nlp_wasm.js")
  external analyzeSentiment: (t, string) => Promise.t<float> = "analyze_sentiment"

  @module("./wasm/nlp_wasm.js")
  external extractKeyTerms: (t, string, int) => Promise.t<array<string>> = "extract_key_terms"

  @module("./wasm/nlp_wasm.js")
  external detectChanges: (t, string, string) => Promise.t<float> = "detect_changes"
}

module SeverityClassifier = {
  type severity = Critical | High | Medium | Low | Info

  type features = {
    sentimentChange: float,
    keyTermMatches: int,
    textualChange: float,
    contextualRelevance: float,
  }

  let classifySeverity = (features: features): severity => {
    let score =
      features.sentimentChange *. 0.3 +.
      Float.fromInt(features.keyTermMatches) *. 0.25 +.
      features.textualChange *. 0.25 +.
      features.contextualRelevance *. 0.20

    if score >= 0.85 {
      Critical
    } else if score >= 0.70 {
      High
    } else if score >= 0.55 {
      Medium
    } else if score >= 0.40 {
      Low
    } else {
      Info
    }
  }

  let severityToString = (s: severity): string => {
    switch s {
    | Critical => "critical"
    | High => "high"
    | Medium => "medium"
    | Low => "low"
    | Info => "info"
    }
  }
}

module GuidanceGenerator = {
  type context = {
    platform: string,
    policyChange: string,
    severity: SeverityClassifier.severity,
    memberImpact: string,
    pestleFactors: array<string>,
  }

  let generateGuidance = async (ctx: context, openaiKey: string): Promise.t<string> => {
    let prompt = `As a journalism union advisor, draft guidance for NUJ members about this policy change:

Platform: ${ctx.platform}
Change: ${ctx.policyChange}
Severity: ${SeverityClassifier.severityToString(ctx.severity)}
Member Impact: ${ctx.memberImpact}
PESTLE Factors: ${ctx.pestleFactors->Array.joinWith(", ")}

Draft concise, actionable guidance (2-3 paragraphs) focusing on:
1. What changed and why it matters
2. Specific actions members should take
3. Resources and support available

Tone: Professional, supportive, empowering.`

    // OpenAI API call would go here
    // For now, return placeholder
    Promise.resolve(`[AI-Generated Guidance]\n\n${prompt}`)
  }
}

module AnalysisApi = {
  type request = {
    policyId: string,
    oldText: string,
    newText: string,
    platform: string,
  }

  type response = {
    policyId: string,
    severity: string,
    changeScore: float,
    keyTerms: array<string>,
    sentiment: float,
    guidance: option<string>,
  }

  let analyzeChange = async (req: request, nlpWasm: WasmNlp.t): Promise.t<response> => {
    let changeScore = await nlpWasm->WasmNlp.detectChanges(req.oldText, req.newText)
    let sentiment = await nlpWasm->WasmNlp.analyzeSentiment(req.newText)
    let keyTerms = await nlpWasm->WasmNlp.extractKeyTerms(req.newText, 10)

    let journalismTerms = ["journalist", "reporter", "defamation", "privacy", "harassment", "verification"]
    let keyTermMatches = keyTerms->Array.filter(term =>
      journalismTerms->Array.some(jTerm => term->String.includes(jTerm))
    )->Array.length

    let features: SeverityClassifier.features = {
      sentimentChange: sentiment,
      keyTermMatches,
      textualChange: changeScore,
      contextualRelevance: Float.fromInt(keyTermMatches) /. Float.fromInt(journalismTerms->Array.length),
    }

    let severity = SeverityClassifier.classifySeverity(features)

    Promise.resolve({
      policyId: req.policyId,
      severity: SeverityClassifier.severityToString(severity),
      changeScore,
      keyTerms,
      sentiment,
      guidance: None, // Generated separately on demand
    })
  }
}

module HttpServer = {
  let handleRequest = async (nlpWasm: WasmNlp.t, request: 'request): Promise.t<'response> => {
    Console.log("Handling analysis request")
    // Deno HTTP server integration would go here
    Promise.resolve(Obj.magic({"status": "ok"}))
  }

  let start = async (port: int): Promise.t<unit> => {
    Console.log(`Starting NUJ Analyzer Service on port ${Int.toString(port)}`)

    let nlpWasm = await WasmNlp.init()
    Console.log("WASM NLP engine initialized")

    // Deno.serve integration
    Console.log("Analyzer service ready")
    Promise.resolve()
  }
}

// Main entry point
let main = async () => {
  let port = 3002
  await HttpServer.start(port)
}

// Execute
main()->ignore
