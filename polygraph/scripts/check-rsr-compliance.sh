#!/usr/bin/env bash

# RSR (Rhodium Standard Repository) Compliance Checker
# Validates repository against RSR framework standards

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Counters
PASS=0
FAIL=0
WARN=0

# Print functions
print_header() {
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}  $1${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

check_pass() {
    echo -e "  ${GREEN}✓${NC} $1"
    ((PASS++))
}

check_fail() {
    echo -e "  ${RED}✗${NC} $1"
    ((FAIL++))
}

check_warn() {
    echo -e "  ${YELLOW}⚠${NC} $1"
    ((WARN++))
}

# Check functions
check_file() {
    if [ -f "$1" ]; then
        check_pass "$1 exists"
        return 0
    else
        check_fail "$1 missing"
        return 1
    fi
}

check_optional_file() {
    if [ -f "$1" ]; then
        check_pass "$1 exists"
    else
        check_warn "$1 missing (recommended)"
    fi
}

check_dir() {
    if [ -d "$1" ]; then
        check_pass "$1/ directory exists"
        return 0
    else
        check_fail "$1/ directory missing"
        return 1
    fi
}

print_header "RSR Framework Compliance Check"
echo ""

# Category 1: Documentation
print_header "1. Documentation Compliance"
check_file "README.md"
check_file "LICENSE"
check_file "LICENSE-PALIMPSEST.txt"
check_file "SECURITY.md"
check_file "CODE_OF_CONDUCT.md"
check_file "CONTRIBUTING.md"
check_file "MAINTAINERS.md"
check_file "CHANGELOG.md"
echo ""

# Category 2: .well-known Directory
print_header "2. .well-known Directory (RFC 9116)"
check_dir ".well-known"
check_file ".well-known/security.txt"
check_file ".well-known/ai.txt"
check_file ".well-known/humans.txt"
echo ""

# Category 3: Build System
print_header "3. Build System"
check_file "justfile"
check_optional_file "Makefile"
check_optional_file "flake.nix"

# Check for language-specific build files
if [ -f "backend/pyproject.toml" ]; then
    check_pass "Python: pyproject.toml (Poetry)"
fi
if [ -f "frontend/package.json" ]; then
    check_pass "JavaScript: package.json"
fi
echo ""

# Category 4: Testing
print_header "4. Testing Infrastructure"
if [ -d "backend/tests" ]; then
    check_pass "Backend tests directory exists"

    # Check for test files
    test_count=$(find backend/tests -name "test_*.py" 2>/dev/null | wc -l)
    if [ "$test_count" -gt 0 ]; then
        check_pass "Found $test_count Python test files"
    else
        check_warn "No Python test files found"
    fi
fi

# Check test configuration
check_optional_file "backend/pytest.ini"
check_optional_file "backend/.coveragerc"
echo ""

# Category 5: CI/CD
print_header "5. CI/CD Pipeline"
check_optional_file ".github/workflows/ci.yml"
check_optional_file ".gitlab-ci.yml"
check_optional_file ".circleci/config.yml"
echo ""

# Category 6: Security
print_header "6. Security Practices"
check_file "SECURITY.md"
check_file ".well-known/security.txt"

# Check for .env.example
if [ -f "backend/.env.example" ]; then
    check_pass "Backend .env.example exists"
fi
if [ -f "frontend/.env.example" ]; then
    check_pass "Frontend .env.example exists"
fi

# Check that .env is gitignored
if grep -q "\.env" .gitignore 2>/dev/null; then
    check_pass ".env files are gitignored"
else
    check_warn ".env should be in .gitignore"
fi
echo ""

# Category 7: Containerization
print_header "7. Containerization"
check_optional_file "backend/Containerfile"
check_optional_file "backend/Dockerfile"
check_optional_file "frontend/Containerfile"
check_optional_file "infrastructure/podman/compose.yaml"
check_optional_file "docker-compose.yml"
echo ""

# Category 8: Type Safety
print_header "8. Type Safety"
# Python type hints
if command -v mypy &> /dev/null; then
    echo "  Checking Python type hints..."
    if cd backend && mypy app --no-error-summary --quiet 2>/dev/null; then
        check_pass "Python type checking passes"
    else
        check_warn "Python type checking has issues"
    fi
    cd ..
else
    check_warn "mypy not installed (cannot verify Python types)"
fi

# TypeScript
if [ -f "frontend/tsconfig.json" ]; then
    check_pass "TypeScript configuration exists"
fi
echo ""

# Category 9: Licensing
print_header "9. Dual Licensing"
check_file "LICENSE"
check_file "LICENSE-PALIMPSEST.txt"

# Check for license headers
if grep -q "MIT" LICENSE 2>/dev/null; then
    check_pass "MIT License detected"
fi
if grep -q "Palimpsest" LICENSE-PALIMPSEST.txt 2>/dev/null; then
    check_pass "Palimpsest License detected"
fi
echo ""

# Category 10: Community Governance (TPCF)
print_header "10. Tri-Perimeter Contribution Framework (TPCF)"
if grep -q -i "perimeter" MAINTAINERS.md 2>/dev/null; then
    check_pass "TPCF perimeter definitions found in MAINTAINERS.md"
else
    check_warn "TPCF perimeter definitions not clearly documented"
fi

if grep -q -i "perimeter" CONTRIBUTING.md 2>/dev/null; then
    check_pass "TPCF mentioned in CONTRIBUTING.md"
else
    check_warn "TPCF should be mentioned in CONTRIBUTING.md"
fi
echo ""

# Category 11: Offline-First
print_header "11. Offline-First Capabilities"
if grep -q -i "cache" README.md 2>/dev/null || grep -q -i "offline" README.md 2>/dev/null; then
    check_pass "Caching/offline capabilities mentioned"
else
    check_warn "Limited offline-first support (requires network APIs)"
fi

# Check for service worker
if [ -f "frontend/src/service-worker.ts" ] || [ -f "frontend/src/sw.js" ]; then
    check_pass "Service worker for offline support"
else
    check_warn "No service worker detected (consider adding for PWA)"
fi
echo ""

# Summary
print_header "Compliance Summary"
total=$((PASS + FAIL + WARN))
compliance_percent=$((PASS * 100 / total))

echo ""
echo -e "  ${GREEN}Passed:${NC}   $PASS"
echo -e "  ${RED}Failed:${NC}   $FAIL"
echo -e "  ${YELLOW}Warnings:${NC} $WARN"
echo -e "  ${BLUE}Total:${NC}    $total"
echo ""
echo -e "  ${BLUE}Compliance Score:${NC} ${compliance_percent}%"
echo ""

# Determine level
if [ "$FAIL" -eq 0 ] && [ "$compliance_percent" -ge 90 ]; then
    echo -e "  ${GREEN}★ GOLD Level RSR Compliance ★${NC}"
elif [ "$FAIL" -le 2 ] && [ "$compliance_percent" -ge 75 ]; then
    echo -e "  ${BLUE}★ SILVER Level RSR Compliance ★${NC}"
elif [ "$FAIL" -le 5 ] && [ "$compliance_percent" -ge 60 ]; then
    echo -e "  ${YELLOW}★ BRONZE Level RSR Compliance ★${NC}"
else
    echo -e "  ${RED}⚠ Below Bronze Level - Improvements Needed${NC}"
fi
echo ""

# Recommendations
if [ "$FAIL" -gt 0 ] || [ "$WARN" -gt 0 ]; then
    print_header "Recommendations"
    echo ""
    if [ "$FAIL" -gt 0 ]; then
        echo "  Critical:"
        echo "    • Address failed checks to improve compliance"
        echo "    • Focus on missing documentation and security files"
    fi
    if [ "$WARN" -gt 0 ]; then
        echo "  Suggested:"
        echo "    • Add recommended files for higher compliance"
        echo "    • Consider implementing offline-first features"
        echo "    • Add Nix flake for reproducible builds"
    fi
    echo ""
fi

# Exit code
if [ "$FAIL" -eq 0 ]; then
    exit 0
else
    exit 1
fi
