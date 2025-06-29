#!/bin/bash
set -e

# Comprehensive release script for Shlesha
# Combines testing, version updates, tagging, and guidance

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

# Print colored output
print_info() { echo -e "${BLUE}ℹ️  $1${NC}"; }
print_success() { echo -e "${GREEN}✅ $1${NC}"; }
print_warning() { echo -e "${YELLOW}⚠️  $1${NC}"; }
print_error() { echo -e "${RED}❌ $1${NC}"; }
print_header() { echo -e "${PURPLE}🚀 $1${NC}"; }

print_header "Shlesha Release Manager"
echo ""

# Check if we're in the right directory
if [[ ! -f "Cargo.toml" ]] || [[ ! -f "scripts/test-release.sh" ]]; then
    print_error "Please run this script from the project root directory"
    exit 1
fi

# Check if scripts exist and are executable
scripts=("scripts/test-release.sh" "scripts/prepare-release.sh" "scripts/tag-release.sh")
for script in "${scripts[@]}"; do
    if [[ ! -f "$script" ]]; then
        print_error "Required script not found: $script"
        exit 1
    fi
    if [[ ! -x "$script" ]]; then
        print_warning "Making $script executable..."
        chmod +x "$script"
    fi
done

echo "Welcome to the Shlesha release process!"
echo ""
echo "This script will guide you through:"
echo "1. 🧪 Running pre-release tests"
echo "2. 📝 Updating version numbers"
echo "3. 🏷️  Creating release tags"
echo "4. 🚀 Triggering CI/CD pipeline"
echo ""

read -p "Ready to start? (y/N): " -n 1 -r
echo ""
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    print_info "Release process cancelled"
    exit 0
fi

# Step 1: Run tests
echo ""
print_header "Step 1: Running Pre-Release Tests"
echo ""

read -p "Run the test suite? (Y/n): " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Nn]$ ]]; then
    print_warning "Skipping tests (not recommended)"
else
    print_info "Running test-release.sh..."
    echo ""
    ./scripts/test-release.sh
    echo ""
    print_success "All tests passed!"
fi

# Step 2: Version preparation
echo ""
print_header "Step 2: Version Preparation"
echo ""

current_version=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
print_info "Current version: $current_version"
echo ""

read -p "Do you need to update the version in Cargo.toml? (y/N): " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
    print_info "Running prepare-release.sh..."
    echo ""
    ./scripts/prepare-release.sh
    echo ""
else
    print_info "Skipping version update"
fi

# Step 3: Tagging
echo ""
print_header "Step 3: Release Tagging"
echo ""

print_info "Now we'll create the release tag and push it to trigger CI/CD"
echo ""

read -p "Create and push release tag? (Y/n): " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Nn]$ ]]; then
    print_warning "Skipping tag creation"
    print_info "You can manually run: ./scripts/tag-release.sh"
else
    print_info "Running tag-release.sh..."
    echo ""
    ./scripts/tag-release.sh
fi

# Step 4: Final guidance
echo ""
print_header "Step 4: Release Complete!"
echo ""

print_success "🎉 Release process finished!"
echo ""
print_info "What happens next:"
echo ""
echo "📋 GitHub Actions will automatically:"
echo "  - Run comprehensive tests across all platforms"
echo "  - Build Python wheels for multiple architectures"
echo "  - Build WASM packages"
echo "  - Publish to PyPI/TestPyPI (based on tag type)"
echo "  - Publish to npm with appropriate tags"
echo "  - Create GitHub release draft"
echo ""
echo "🔍 Monitor progress:"
echo "  - GitHub Actions: https://github.com/udapaana/shlesha/actions"
echo "  - PyPI releases: https://pypi.org/project/shlesha/"
echo "  - npm releases: https://www.npmjs.com/package/shlesha-wasm"
echo ""
echo "✏️  Post-release tasks:"
echo "  - Edit and publish the GitHub release notes"
echo "  - Test installation from PyPI/npm"
echo "  - Update documentation if needed"
echo "  - Announce the release"
echo ""

print_success "Happy releasing! 🚀"