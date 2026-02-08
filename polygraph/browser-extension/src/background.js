// Background service worker for Social Media Polygraph extension

const API_URL = 'http://localhost:8000/api/v1';

// Context menu setup
chrome.runtime.onInstalled.addListener(() => {
  chrome.contextMenus.create({
    id: 'verifySelection',
    title: 'Verify with Polygraph',
    contexts: ['selection']
  });
});

// Handle context menu clicks
chrome.contextMenus.onClicked.addListener((info, tab) => {
  if (info.menuItemId === 'verifySelection' && info.selectionText) {
    verifyClaim(info.selectionText, tab.url);
  }
});

// Handle messages from content scripts and popup
chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
  if (request.action === 'verifyClaim') {
    verifyClaim(request.text, request.url)
      .then(result => sendResponse({ success: true, result }))
      .catch(error => sendResponse({ success: false, error: error.message }));
    return true; // Keep channel open for async response
  }

  if (request.action === 'getSettings') {
    chrome.storage.sync.get(['apiUrl', 'apiKey'], (settings) => {
      sendResponse(settings);
    });
    return true;
  }
});

// Verify a claim via API
async function verifyClaim(text, sourceUrl = null) {
  try {
    // Get API settings
    const settings = await chrome.storage.sync.get(['apiUrl', 'apiKey']);
    const apiUrl = settings.apiUrl || API_URL;
    const apiKey = settings.apiKey || '';

    const headers = {
      'Content-Type': 'application/json'
    };

    if (apiKey) {
      headers['X-API-Key'] = apiKey;
    }

    const response = await fetch(`${apiUrl}/claims/verify`, {
      method: 'POST',
      headers,
      body: JSON.stringify({
        text,
        url: sourceUrl,
        platform: getPlatformFromUrl(sourceUrl)
      })
    });

    if (!response.ok) {
      throw new Error(`API error: ${response.status}`);
    }

    const data = await response.json();

    // Store result in local storage for popup access
    await chrome.storage.local.set({
      lastVerification: {
        timestamp: Date.now(),
        data
      }
    });

    return data;
  } catch (error) {
    console.error('Verification error:', error);
    throw error;
  }
}

// Extract platform from URL
function getPlatformFromUrl(url) {
  if (!url) return 'unknown';

  if (url.includes('twitter.com') || url.includes('x.com')) {
    return 'twitter';
  } else if (url.includes('facebook.com')) {
    return 'facebook';
  } else if (url.includes('instagram.com')) {
    return 'instagram';
  } else if (url.includes('tiktok.com')) {
    return 'tiktok';
  } else if (url.includes('reddit.com')) {
    return 'reddit';
  }

  return 'unknown';
}

// Badge to show verification status
function updateBadge(verdict) {
  const colors = {
    'true': '#10b981',
    'mostly_true': '#10b981',
    'mixed': '#f59e0b',
    'mostly_false': '#ef4444',
    'false': '#ef4444',
    'unverifiable': '#6b7280'
  };

  const text = {
    'true': '✓',
    'mostly_true': '✓',
    'mixed': '?',
    'mostly_false': '✗',
    'false': '✗',
    'unverifiable': '-'
  };

  chrome.action.setBadgeBackgroundColor({ color: colors[verdict] || '#6b7280' });
  chrome.action.setBadgeText({ text: text[verdict] || '-' });
}
