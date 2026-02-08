// Content script for Social Media Polygraph
// Runs on social media pages to enable in-context verification

(function() {
  'use strict';

  // Add verification button to posts
  function addVerifyButtons() {
    const platform = detectPlatform();

    if (platform === 'twitter') {
      addTwitterVerifyButtons();
    } else if (platform === 'facebook') {
      addFacebookVerifyButtons();
    } else if (platform === 'instagram') {
      addInstagramVerifyButtons();
    }
  }

  // Detect current platform
  function detectPlatform() {
    const hostname = window.location.hostname;

    if (hostname.includes('twitter.com') || hostname.includes('x.com')) {
      return 'twitter';
    } else if (hostname.includes('facebook.com')) {
      return 'facebook';
    } else if (hostname.includes('instagram.com')) {
      return 'instagram';
    }

    return 'unknown';
  }

  // Add verify buttons to Twitter posts
  function addTwitterVerifyButtons() {
    const tweets = document.querySelectorAll('article[data-testid="tweet"]');

    tweets.forEach(tweet => {
      // Skip if button already added
      if (tweet.querySelector('.polygraph-verify-btn')) {
        return;
      }

      const tweetText = tweet.querySelector('[data-testid="tweetText"]');
      if (!tweetText) return;

      // Create verify button
      const button = createVerifyButton();
      button.addEventListener('click', (e) => {
        e.stopPropagation();
        e.preventDefault();
        verifyTweet(tweetText.textContent, tweet);
      });

      // Add button to tweet actions
      const actions = tweet.querySelector('[role="group"]');
      if (actions) {
        const buttonContainer = document.createElement('div');
        buttonContainer.appendChild(button);
        actions.appendChild(buttonContainer);
      }
    });
  }

  // Add verify buttons to Facebook posts (simplified)
  function addFacebookVerifyButtons() {
    // Facebook's DOM is complex and frequently changes
    // This is a placeholder implementation
    console.log('Facebook verify buttons would be added here');
  }

  // Add verify buttons to Instagram posts (simplified)
  function addInstagramVerifyButtons() {
    // Instagram's DOM is complex and frequently changes
    // This is a placeholder implementation
    console.log('Instagram verify buttons would be added here');
  }

  // Create verify button element
  function createVerifyButton() {
    const button = document.createElement('button');
    button.className = 'polygraph-verify-btn';
    button.innerHTML = `
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"></path>
        <path d="M9 12l2 2 4-4"></path>
      </svg>
      <span>Verify</span>
    `;
    button.title = 'Verify this claim with Social Media Polygraph';
    return button;
  }

  // Verify a tweet
  async function verifyTweet(text, tweetElement) {
    try {
      // Show loading state
      const button = tweetElement.querySelector('.polygraph-verify-btn');
      if (button) {
        button.disabled = true;
        button.innerHTML = '<span>Verifying...</span>';
      }

      // Send to background script for API call
      const response = await chrome.runtime.sendMessage({
        action: 'verifyClaim',
        text: text,
        url: window.location.href
      });

      if (response.success && response.result.success) {
        showVerificationResult(response.result, tweetElement);
      } else {
        showError('Verification failed', tweetElement);
      }
    } catch (error) {
      console.error('Verification error:', error);
      showError(error.message, tweetElement);
    }
  }

  // Show verification result
  function showVerificationResult(result, element) {
    const { analysis } = result;
    if (!analysis) return;

    const { verification } = analysis;

    // Create result popup
    const popup = document.createElement('div');
    popup.className = 'polygraph-result-popup';
    popup.innerHTML = `
      <div class="polygraph-result-header ${getVerdictClass(verification.verdict)}">
        <strong>${formatVerdict(verification.verdict)}</strong>
        <span>${Math.round(verification.confidence * 100)}% confidence</span>
      </div>
      <div class="polygraph-result-body">
        <p>${verification.explanation}</p>
        ${verification.fact_checks.length > 0 ? `
          <div class="polygraph-sources">
            <strong>Sources:</strong>
            <ul>
              ${verification.fact_checks.map(fc => `
                <li>${fc.source}: ${formatVerdict(fc.verdict)}</li>
              `).join('')}
            </ul>
          </div>
        ` : ''}
      </div>
      <button class="polygraph-close-btn">Close</button>
    `;

    // Add close handler
    popup.querySelector('.polygraph-close-btn').addEventListener('click', () => {
      popup.remove();
    });

    // Position near the tweet
    element.appendChild(popup);
  }

  // Show error message
  function showError(message, element) {
    const errorDiv = document.createElement('div');
    errorDiv.className = 'polygraph-error';
    errorDiv.textContent = `Error: ${message}`;
    element.appendChild(errorDiv);

    setTimeout(() => errorDiv.remove(), 5000);
  }

  // Get CSS class for verdict
  function getVerdictClass(verdict) {
    switch (verdict) {
      case 'true':
      case 'mostly_true':
        return 'verdict-true';
      case 'false':
      case 'mostly_false':
        return 'verdict-false';
      case 'mixed':
        return 'verdict-mixed';
      default:
        return 'verdict-unknown';
    }
  }

  // Format verdict for display
  function formatVerdict(verdict) {
    return verdict.split('_').map(word =>
      word.charAt(0).toUpperCase() + word.slice(1)
    ).join(' ');
  }

  // Initialize when DOM is ready
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', addVerifyButtons);
  } else {
    addVerifyButtons();
  }

  // Re-run when new content is loaded (for infinite scroll)
  const observer = new MutationObserver(() => {
    addVerifyButtons();
  });

  observer.observe(document.body, {
    childList: true,
    subtree: true
  });
})();
