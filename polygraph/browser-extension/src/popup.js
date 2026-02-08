// Popup script for Social Media Polygraph extension

document.getElementById('verifyBtn').addEventListener('click', async () => {
  const text = document.getElementById('claimText').value.trim();

  if (!text) {
    alert('Please enter a claim to verify');
    return;
  }

  const button = document.getElementById('verifyBtn');
  const resultDiv = document.getElementById('result');

  try {
    button.disabled = true;
    button.textContent = 'Verifying...';
    resultDiv.innerHTML = '<div class="loading">Analyzing claim...</div>';

    // Get current tab URL
    const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });

    // Send to background script
    const response = await chrome.runtime.sendMessage({
      action: 'verifyClaim',
      text: text,
      url: tab.url
    });

    if (response.success && response.result.success) {
      displayResult(response.result);
    } else {
      throw new Error(response.error || 'Verification failed');
    }
  } catch (error) {
    resultDiv.innerHTML = `<div class="error">Error: ${error.message}</div>`;
  } finally {
    button.disabled = false;
    button.textContent = 'Verify Claim';
  }
});

function displayResult(result) {
  const { analysis } = result;
  if (!analysis) return;

  const { verification } = analysis;
  const resultDiv = document.getElementById('result');

  const verdictClass = verification.verdict.includes('true') ? 'true' :
                       verification.verdict.includes('false') ? 'false' : 'mixed';

  resultDiv.innerHTML = `
    <div class="result ${verdictClass}">
      <strong>${formatVerdict(verification.verdict)}</strong>
      <p style="margin: 8px 0 0 0; font-size: 13px;">
        ${verification.explanation}
      </p>
      <p style="margin: 8px 0 0 0; font-size: 12px; opacity: 0.8;">
        Confidence: ${Math.round(verification.confidence * 100)}%
      </p>
    </div>
  `;
}

function formatVerdict(verdict) {
  return verdict.split('_').map(word =>
    word.charAt(0).toUpperCase() + word.slice(1)
  ).join(' ');
}

// Load last verification on popup open
chrome.storage.local.get(['lastVerification'], (data) => {
  if (data.lastVerification && data.lastVerification.data) {
    displayResult(data.lastVerification.data);
  }
});
