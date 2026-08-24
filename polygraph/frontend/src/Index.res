// Main entry point for ReScript frontend

%%raw(`import "./index.css"`)

module App = {
  @react.component
  let make = () => {
    <div className="min-h-screen bg-gray-50">
      <Header />
      <main className="container mx-auto px-4 py-8">
        <h1 className="text-4xl font-bold mb-4">
          {React.string("Social Media Polygraph")}
        </h1>
        <p className="text-xl text-gray-600 mb-8">
          {React.string("AI-powered fact-checking with memory safety and formal verification")}
        </p>
        <VerifyPage />
      </main>
      <Footer />
    </div>
  }
}

module Header = {
  @react.component
  let make = () => {
    <header className="bg-white border-b border-gray-200">
      <nav className="container mx-auto px-4 h-16 flex items-center justify-between">
        <div className="flex items-center space-x-2">
          <svg className="h-8 w-8 text-primary-600" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
            <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" />
            <path d="M9 12l2 2 4-4" />
          </svg>
          <span className="text-xl font-bold">
            {React.string("Polygraph")}
          </span>
        </div>
        <div className="flex items-center space-x-6">
          <a href="/verify" className="text-gray-700 hover:text-primary-600">
            {React.string("Verify")}
          </a>
          <a href="/about" className="text-gray-700 hover:text-primary-600">
            {React.string("About")}
          </a>
        </div>
      </nav>
    </header>
  }
}

module Footer = {
  @react.component
  let make = () => {
    <footer className="bg-gray-900 text-gray-300 py-8 mt-12">
      <div className="container mx-auto px-4 text-center">
        <p>{React.string("Built with Rust, ReScript, and WASM")}</p>
        <p className="mt-2 text-sm text-gray-400">
          {React.string("Memory-safe, type-safe, formally verified")}
        </p>
      </div>
    </footer>
  }
}

// Initialize React app
switch ReactDOM.querySelector("#root") {
| Some(root) => {
    let reactRoot = ReactDOM.Client.createRoot(root)
    ReactDOM.Client.render(reactRoot, <App />)
  }
| None => Js.Console.error("Root element not found")
}
