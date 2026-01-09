#!/bin/bash

set -e

REPO="KevinEdry/chloe"
BINARY="chloe"
INSTALL_DIR="${CHLOE_INSTALL_DIR:-/usr/local/bin}"
GITHUB_API="https://api.github.com/repos/${REPO}"
GITHUB_RELEASES="https://github.com/${REPO}/releases/download"

# Parse command line arguments
VERSION="$1"

# Validate version if provided
if [[ -n "$VERSION" ]] && [[ ! "$VERSION" =~ ^v?[0-9]+\.[0-9]+\.[0-9]+(-[^[:space:]]+)?$ ]]; then
    echo "Usage: $0 [VERSION]" >&2
    echo "Example: $0 v0.1.1" >&2
    exit 1
fi

# Check for required dependencies
DOWNLOADER=""
if command -v curl >/dev/null 2>&1; then
    DOWNLOADER="curl"
elif command -v wget >/dev/null 2>&1; then
    DOWNLOADER="wget"
else
    echo "Error: Either curl or wget is required but neither is installed" >&2
    exit 1
fi

# Download function that works with both curl and wget
download_file() {
    local url="$1"
    local output="$2"

    if [ "$DOWNLOADER" = "curl" ]; then
        if [ -n "$output" ]; then
            curl -fsSL -o "$output" "$url"
        else
            curl -fsSL "$url"
        fi
    elif [ "$DOWNLOADER" = "wget" ]; then
        if [ -n "$output" ]; then
            wget -q -O "$output" "$url"
        else
            wget -q -O - "$url"
        fi
    else
        return 1
    fi
}

# Detect platform
detect_platform() {
    local os arch

    case "$(uname -s)" in
        Darwin) os="macos" ;;
        Linux) os="linux" ;;
        MINGW*|MSYS*|CYGWIN*)
            echo "Error: Windows is not supported. Please use WSL." >&2
            exit 1
            ;;
        *)
            echo "Error: Unsupported operating system: $(uname -s)" >&2
            exit 1
            ;;
    esac

    case "$(uname -m)" in
        x86_64|amd64) arch="x86_64" ;;
        arm64|aarch64) arch="aarch64" ;;
        *)
            echo "Error: Unsupported architecture: $(uname -m)" >&2
            exit 1
            ;;
    esac

    # Check for unsupported combinations
    if [ "$os" = "linux" ] && [ "$arch" = "aarch64" ]; then
        echo "Error: Linux ARM64 is not currently supported" >&2
        exit 1
    fi

    echo "${os}-${arch}"
}

# Get latest version from GitHub API
get_latest_version() {
    local response
    response=$(download_file "${GITHUB_API}/releases/latest")

    # Extract tag_name using grep and sed (no jq dependency)
    echo "$response" | grep -o '"tag_name"[[:space:]]*:[[:space:]]*"[^"]*"' | sed 's/.*"\([^"]*\)"$/\1/'
}

# Verify checksum if checksums file exists
verify_checksum() {
    local file="$1"
    local checksums_url="$2"
    local expected_name="$3"
    local checksums_content
    local expected_checksum
    local actual_checksum

    # Try to download checksums file (optional)
    if ! checksums_content=$(download_file "$checksums_url" 2>/dev/null); then
        echo "  Skipping checksum verification (no checksums file)"
        return 0
    fi

    # Extract expected checksum
    expected_checksum=$(echo "$checksums_content" | grep "$expected_name" | awk '{print $1}')

    if [ -z "$expected_checksum" ]; then
        echo "  Skipping checksum verification (asset not in checksums)"
        return 0
    fi

    # Calculate actual checksum
    if command -v sha256sum >/dev/null 2>&1; then
        actual_checksum=$(sha256sum "$file" | awk '{print $1}')
    elif command -v shasum >/dev/null 2>&1; then
        actual_checksum=$(shasum -a 256 "$file" | awk '{print $1}')
    else
        echo "  Skipping checksum verification (no sha256sum or shasum)"
        return 0
    fi

    if [ "$actual_checksum" != "$expected_checksum" ]; then
        echo "Error: Checksum verification failed!" >&2
        echo "  Expected: $expected_checksum" >&2
        echo "  Actual:   $actual_checksum" >&2
        return 1
    fi

    echo "  Checksum verified"
    return 0
}

main() {
    echo "Chloe Installer"
    echo "==============="
    echo ""

    # Detect platform
    echo "Detecting platform..."
    platform=$(detect_platform)
    echo "  Platform: $platform"

    # Get version
    if [ -z "$VERSION" ]; then
        echo "Fetching latest version..."
        VERSION=$(get_latest_version)
    fi

    # Ensure version starts with 'v'
    if [[ ! "$VERSION" =~ ^v ]]; then
        VERSION="v${VERSION}"
    fi
    echo "  Version: $VERSION"

    # Construct download URL
    asset_name="${BINARY}-${platform}.tar.gz"
    download_url="${GITHUB_RELEASES}/${VERSION}/${asset_name}"
    checksums_url="${GITHUB_RELEASES}/${VERSION}/SHA256SUMS"

    # Create temp directory
    tmpdir=$(mktemp -d)
    trap 'rm -rf "$tmpdir"' EXIT

    # Download
    echo "Downloading ${asset_name}..."
    archive_path="${tmpdir}/${asset_name}"
    if ! download_file "$download_url" "$archive_path"; then
        echo "Error: Download failed" >&2
        echo "  URL: $download_url" >&2
        echo "" >&2
        echo "This could mean:" >&2
        echo "  - The version doesn't exist" >&2
        echo "  - Your platform ($platform) is not supported" >&2
        echo "  - Network issues" >&2
        exit 1
    fi

    # Verify checksum (optional)
    verify_checksum "$archive_path" "$checksums_url" "$asset_name"

    # Extract
    echo "Extracting..."
    tar -xzf "$archive_path" -C "$tmpdir"

    # Install
    echo "Installing to ${INSTALL_DIR}..."
    if [ -w "$INSTALL_DIR" ]; then
        mv "${tmpdir}/${BINARY}" "${INSTALL_DIR}/"
        chmod +x "${INSTALL_DIR}/${BINARY}"
    else
        echo "  (requires sudo)"
        sudo mv "${tmpdir}/${BINARY}" "${INSTALL_DIR}/"
        sudo chmod +x "${INSTALL_DIR}/${BINARY}"
    fi

    # Verify installation
    if ! command -v "$BINARY" >/dev/null 2>&1; then
        echo ""
        echo "Warning: '${BINARY}' is not in your PATH"
        echo "  Add ${INSTALL_DIR} to your PATH, or set CHLOE_INSTALL_DIR"
        echo "  Example: export PATH=\"${INSTALL_DIR}:\$PATH\""
    fi

    echo ""
    echo "Chloe ${VERSION} installed successfully!"
    echo ""
    echo "Run 'chloe --help' to get started."
}

main
