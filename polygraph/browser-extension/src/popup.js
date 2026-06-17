// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
// Popup script for Social Media Polygraph extension

function safeSetHTML(element, htmlString) {
  const parser = new DOMParser();
  const doc = parser.parseFromString(htmlString, 'text/html');
  element.replaceChildren(...doc.body.childNodes);
}

function escapeHtml(text) {
  if (!text) return text;
  return String(text)
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#039;");
}

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
    safeSetHTML(resultDiv, '<div class="loading">Analyzing claim...</div>');

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
    safeSetHTML(resultDiv, `<div class="error">Error: ${escapeHtml(error.message)}</div>`);
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

  safeSetHTML(resultDiv, `
    <div class="result ${escapeHtml(verdictClass)}">
      <strong>${escapeHtml(formatVerdict(verification.verdict))}</strong>
      <p style="margin: 8px 0 0 0; font-size: 13px;">
        ${escapeHtml(verification.explanation)}
      </p>
      <p style="margin: 8px 0 0 0; font-size: 12px; opacity: 0.8;">
        Confidence: ${escapeHtml(Math.round(verification.confidence * 100))}%
      </p>
    </div>
  `);
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
