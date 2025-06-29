#!/bin/bash
set -e

# Script to automatically increment version tags for Shlesha releases
# Supports both release candidates (rc) and stable releases

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print colored output
print_info() { echo -e "${BLUE}ℹ️  $1${NC}"; }
print_success() { echo -e "${GREEN}✅ $1${NC}"; }
print_warning() { echo -e "${YELLOW}⚠️  $1${NC}"; }
print_error() { echo -e "${RED}❌ $1${NC}"; }

print_info "🏷️  Shlesha Release Tag Manager"
echo ""

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    print_error "Not in a git repository"
    exit 1
fi

# Check if working directory is clean
if [[ -n $(git status --porcelain) ]]; then
    print_warning "Working directory has uncommitted changes:"
    git status --short
    echo ""
    read -p "Continue anyway? (y/N): " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_info "Aborting. Please commit or stash your changes first."
        exit 1
    fi
fi

# Function to parse semantic version
parse_version() {
    local version=$1
    local regex="^v?([0-9]+)\.([0-9]+)\.([0-9]+)(-rc([0-9]+))?$"
    
    if [[ $version =~ $regex ]]; then
        echo "${BASH_REMATCH[1]} ${BASH_REMATCH[2]} ${BASH_REMATCH[3]} ${BASH_REMATCH[5]:-0}"
    else
        echo "0 0 0 0"
    fi
}

# Function to compare versions
version_gt() {
    local v1_parts=($1)
    local v2_parts=($2)
    
    # Compare major.minor.patch
    for i in 0 1 2; do
        if [[ ${v1_parts[i]} -gt ${v2_parts[i]} ]]; then
            return 0
        elif [[ ${v1_parts[i]} -lt ${v2_parts[i]} ]]; then
            return 1
        fi
    done
    
    # If base versions are equal, compare RC numbers
    # No RC (stable) > RC with number
    if [[ ${v1_parts[3]} -eq 0 && ${v2_parts[3]} -gt 0 ]]; then
        return 0  # v1 is stable, v2 is RC -> v1 > v2
    elif [[ ${v1_parts[3]} -gt 0 && ${v2_parts[3]} -eq 0 ]]; then
        return 1  # v1 is RC, v2 is stable -> v1 < v2
    elif [[ ${v1_parts[3]} -gt ${v2_parts[3]} ]]; then
        return 0  # Both RC, compare RC numbers
    else
        return 1
    fi
}

# Get all tags and find the latest version
print_info "Analyzing existing tags..."
all_tags=$(git tag -l | grep -E '^v[0-9]+\.[0-9]+\.[0-9]+(-rc[0-9]+)?$' | sort -V)

if [[ -z "$all_tags" ]]; then
    latest_version="v0.0.0"
    latest_parsed="0 0 0 0"
    print_info "No existing version tags found. Starting from v0.1.0"
else
    # Find the latest version by comparing parsed versions
    latest_version=""
    latest_parsed="0 0 0 0"
    
    while IFS= read -r tag; do
        if [[ -n "$tag" ]]; then
            parsed=$(parse_version "$tag")
            if version_gt "$parsed" "$latest_parsed"; then
                latest_version="$tag"
                latest_parsed="$parsed"
            fi
        fi
    done <<< "$all_tags"
fi

print_success "Latest version: $latest_version"

# Parse the latest version
parsed_latest=($latest_parsed)
major=${parsed_latest[0]}
minor=${parsed_latest[1]}
patch=${parsed_latest[2]}
rc=${parsed_latest[3]}

echo ""
print_info "Current version breakdown:"
echo "  Major: $major"
echo "  Minor: $minor" 
echo "  Patch: $patch"
if [[ $rc -gt 0 ]]; then
    echo "  RC: $rc"
else
    echo "  Type: Stable release"
fi

echo ""
echo "What type of release do you want to create?"
echo ""
echo "1) Release candidate (RC) - for TestPyPI + crates.io"
echo "2) Stable release - for production PyPI + crates.io"
echo "3) Patch release (bug fixes)"
echo "4) Minor release (new features)"
echo "5) Major release (breaking changes)"
echo "6) Minor RC release (new features, testing)"
echo "7) Major RC release (breaking changes, testing)"
echo ""

read -p "Select option (1-7): " -n 1 -r
echo ""

case $REPLY in
    1)
        # Release candidate
        if [[ $rc -gt 0 ]]; then
            # Increment RC number
            new_rc=$((rc + 1))
            new_version="v${major}.${minor}.${patch}-rc.${new_rc}"
            release_type="Release Candidate"
            target_pypi="TestPyPI"
        else
            # First RC for this version
            new_version="v${major}.${minor}.${patch}-rc.1"
            release_type="Release Candidate"
            target_pypi="TestPyPI"
        fi
        ;;
    2)
        # Stable release
        if [[ $rc -gt 0 ]]; then
            # Promote RC to stable
            new_version="v${major}.${minor}.${patch}"
            release_type="Stable Release (promoted from RC)"
            target_pypi="Production PyPI"
        else
            # New patch version
            new_patch=$((patch + 1))
            new_version="v${major}.${minor}.${new_patch}"
            release_type="Stable Release"
            target_pypi="Production PyPI"
        fi
        ;;
    3)
        # Patch release
        new_patch=$((patch + 1))
        new_version="v${major}.${minor}.${new_patch}"
        release_type="Patch Release"
        target_pypi="Production PyPI"
        ;;
    4)
        # Minor release
        new_minor=$((minor + 1))
        new_version="v${major}.${new_minor}.0"
        release_type="Minor Release"
        target_pypi="Production PyPI"
        ;;
    5)
        # Major release
        new_major=$((major + 1))
        new_version="v${new_major}.0.0"
        release_type="Major Release"
        target_pypi="Production PyPI"
        ;;
    6)
        # Minor RC release
        new_minor=$((minor + 1))
        new_version="v${major}.${new_minor}.0-rc.1"
        release_type="Minor Release Candidate"
        target_pypi="TestPyPI"
        ;;
    7)
        # Major RC release
        new_major=$((major + 1))
        new_version="v${new_major}.0.0-rc.1"
        release_type="Major Release Candidate"
        target_pypi="TestPyPI"
        ;;
    *)
        print_error "Invalid option selected"
        exit 1
        ;;
esac

echo ""
print_info "New version will be: $new_version"
print_info "Release type: $release_type"
print_info "Target: $target_pypi"
echo ""

# Check if tag already exists
if git tag -l | grep -q "^${new_version}$"; then
    print_error "Tag $new_version already exists!"
    exit 1
fi

# Confirm with user
read -p "Create tag $new_version? (y/N): " -n 1 -r
echo ""
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    print_info "Aborted by user"
    exit 0
fi

# Create the tag
commit_message="$release_type: $new_version"
if [[ "$target_pypi" == "TestPyPI" ]]; then
    commit_message="$commit_message (TestPyPI)"
fi

print_info "Creating tag..."
git tag -a "$new_version" -m "$commit_message"
print_success "Tag $new_version created locally"

echo ""
print_info "Next steps:"
echo ""

if [[ "$target_pypi" == "TestPyPI" ]]; then
    echo "1. Push the tag: git push origin $new_version"
    echo "2. GitHub Actions will automatically:"
    echo "   - Build and test the code"
    echo "   - Publish Python wheels to TestPyPI"
    echo "   - Publish WASM package to npm with @rc tag"
    echo "   - Publish Rust crate to crates.io as pre-release"
    echo ""
    echo "3. Test the release:"
    echo "   Python: pip install -i https://test.pypi.org/simple/ shlesha==${new_version/v/}"
    echo "   WASM: npm install shlesha-wasm@rc"
    echo "   Rust: cargo add shlesha@${new_version/v/}"
    echo ""
    echo "4. When ready for stable release, run this script again and select option 2"
else
    echo "1. Push the tag: git push origin $new_version"
    echo "2. GitHub Actions will automatically:"
    echo "   - Build and test the code"
    echo "   - Publish Python wheels to Production PyPI"
    echo "   - Publish WASM package to npm as latest"
    echo "   - Publish Rust crate to crates.io as stable"
    echo ""
    echo "3. Verify the release:"
    echo "   Python: pip install shlesha==${new_version/v/}"
    echo "   WASM: npm install shlesha-wasm"
    echo "   Rust: cargo add shlesha@${new_version/v/}"
    echo ""
    echo "4. Update GitHub release notes at:"
    echo "   https://github.com/udapaana/shlesha/releases/tag/$new_version"
fi

echo ""
read -p "Push the tag now? (y/N): " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
    print_info "Pushing tag to remote..."
    git push origin "$new_version"
    print_success "Tag pushed successfully!"
    echo ""
    print_success "🚀 Release $new_version is now building in GitHub Actions!"
    print_info "Monitor progress at: https://github.com/udapaana/shlesha/actions"
else
    print_info "Tag created locally but not pushed."
    print_info "Run 'git push origin $new_version' when ready."
fi

echo ""
print_success "Release tagging complete! 🎉"