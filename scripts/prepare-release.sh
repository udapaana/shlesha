#!/bin/bash
set -e

# Script to prepare a release: update version in Cargo.toml and create commit
# This complements tag-release.sh by handling the version file updates

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

print_info "📦 Shlesha Release Preparation"
echo ""

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    print_error "Not in a git repository"
    exit 1
fi

# Check if Cargo.toml exists
if [[ ! -f "Cargo.toml" ]]; then
    print_error "Cargo.toml not found in current directory"
    exit 1
fi

# Function to get current version from Cargo.toml
get_current_version() {
    grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/'
}

# Function to update version in Cargo.toml
update_cargo_version() {
    local new_version=$1
    # Use sed to update the first version line (should be the package version)
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS sed syntax
        sed -i '' "s/^version = \".*\"/version = \"$new_version\"/" Cargo.toml
    else
        # Linux sed syntax
        sed -i "s/^version = \".*\"/version = \"$new_version\"/" Cargo.toml
    fi
}

current_version=$(get_current_version)
print_info "Current version in Cargo.toml: $current_version"

echo ""
print_info "What type of version update do you need?"
echo ""
echo "1) Keep current version (just create commit for tagging)"
echo "2) Set specific version (e.g., 0.1.0-rc1, 0.1.0)"
echo "3) Auto-increment based on git tags"
echo ""

read -p "Select option (1-3): " -n 1 -r
echo ""

case $REPLY in
    1)
        new_version="$current_version"
        print_info "Keeping current version: $new_version"
        ;;
    2)
        echo ""
        read -p "Enter new version (without 'v' prefix): " new_version
        if [[ -z "$new_version" ]]; then
            print_error "Version cannot be empty"
            exit 1
        fi
        print_info "Setting version to: $new_version"
        ;;
    3)
        print_info "Analyzing git tags to determine next version..."
        
        # Get the latest tag
        latest_tag=$(git tag -l | grep -E '^v[0-9]+\.[0-9]+\.[0-9]+(-rc[0-9]+)?$' | sort -V | tail -1)
        
        if [[ -z "$latest_tag" ]]; then
            new_version="0.1.0-rc.1"
            print_info "No existing tags found. Setting initial version: $new_version"
        else
            # Parse the latest tag to suggest next version
            clean_tag=${latest_tag#v}  # Remove 'v' prefix
            
            if [[ "$clean_tag" == *"-rc"* ]]; then
                # Current is RC, suggest next RC or stable
                base_version=${clean_tag%-rc*}
                rc_num=${clean_tag##*-rc}
                next_rc=$((rc_num + 1))
                
                echo ""
                echo "Latest tag: $latest_tag"
                echo "Suggested options:"
                echo "  1) Next RC: ${base_version}-rc.${next_rc}"
                echo "  2) Stable: ${base_version}"
                echo ""
                read -p "Select (1-2): " -n 1 -r
                echo ""
                
                if [[ $REPLY == "1" ]]; then
                    new_version="${base_version}-rc.${next_rc}"
                else
                    new_version="$base_version"
                fi
            else
                # Current is stable, suggest next patch
                IFS='.' read -ra ADDR <<< "$clean_tag"
                major=${ADDR[0]}
                minor=${ADDR[1]}
                patch=${ADDR[2]}
                next_patch=$((patch + 1))
                
                new_version="${major}.${minor}.${next_patch}"
                print_info "Latest stable tag: $latest_tag"
                print_info "Suggested next version: $new_version"
            fi
        fi
        ;;
    *)
        print_error "Invalid option selected"
        exit 1
        ;;
esac

echo ""

# Update Cargo.toml if version changed
if [[ "$new_version" != "$current_version" ]]; then
    print_info "Updating Cargo.toml version from $current_version to $new_version"
    update_cargo_version "$new_version"
    print_success "Cargo.toml updated"
    
    # Verify the change
    updated_version=$(get_current_version)
    if [[ "$updated_version" != "$new_version" ]]; then
        print_error "Failed to update version in Cargo.toml"
        exit 1
    fi
else
    print_info "Version unchanged, no Cargo.toml update needed"
fi

# Check if there are changes to commit
if [[ -n $(git status --porcelain) ]]; then
    echo ""
    print_info "Current changes to be committed:"
    git status --short
    echo ""
    
    commit_msg="chore: prepare v${new_version} release"
    
    read -p "Create commit with message '$commit_msg'? (y/N): " -n 1 -r
    echo ""
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        git add Cargo.toml
        git commit -m "$commit_msg"
        print_success "Release preparation commit created"
        
        echo ""
        print_info "Next steps:"
        echo "1. Run the tagging script: ./scripts/tag-release.sh"
        echo "2. Or manually create tag: git tag -a v${new_version} -m 'Release v${new_version}'"
    else
        print_info "No commit created. You can commit manually when ready."
    fi
else
    print_info "No changes to commit"
    echo ""
    print_info "Ready to create tag for v${new_version}"
    echo "Run: ./scripts/tag-release.sh"
fi

echo ""
print_success "Release preparation complete! 🎉"