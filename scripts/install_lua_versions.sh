#!/bin/bash
# Script to download and build multiple Lua versions for testing dynamic loading
# This installs Lua 5.1, 5.2, 5.3, and 5.4 to /usr/local/lib

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Lua versions to install
VERSIONS=("5.1.5" "5.2.4" "5.3.6" "5.4.7")
BASE_URL="https://www.lua.org/ftp"
INSTALL_DIR="/usr/local"
BUILD_DIR="/tmp/lua-builds"

# Detect platform
if [[ "$OSTYPE" == "darwin"* ]]; then
    PLATFORM="macosx"
    INSTALL_PREFIX="/usr/local"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    PLATFORM="linux"
    INSTALL_PREFIX="/usr/local"
else
    echo -e "${RED}Unsupported platform: $OSTYPE${NC}"
    exit 1
fi

echo -e "${GREEN}Installing Lua versions for dynamic loading tests${NC}"
echo "Platform: $PLATFORM"
echo "Install prefix: $INSTALL_PREFIX"
echo ""

# Create build directory
mkdir -p "$BUILD_DIR"
cd "$BUILD_DIR"

# Function to build and install a Lua version
install_lua_version() {
    local version=$1
    local major_minor=$(echo $version | cut -d. -f1,2)
    local tarball="lua-${version}.tar.gz"
    local dir="lua-${version}"

    echo -e "${YELLOW}=== Installing Lua ${version} ===${NC}"

    # Download if not already present
    if [ ! -f "$tarball" ]; then
        echo "Downloading Lua ${version}..."
        curl -L -o "$tarball" "${BASE_URL}/${tarball}"
    else
        echo "Using cached tarball: $tarball"
    fi

    # Extract
    echo "Extracting..."
    rm -rf "$dir"
    tar -xzf "$tarball"
    cd "$dir"

    # Build
    echo "Building..."
    if [[ "$PLATFORM" == "macosx" ]]; then
        make macosx
    else
        make linux
    fi

    # Install
    echo "Installing to ${INSTALL_PREFIX}..."
    if [ "$EUID" -ne 0 ]; then
        echo "Running with sudo for installation..."
        sudo make install INSTALL_TOP="${INSTALL_PREFIX}"
    else
        make install INSTALL_TOP="${INSTALL_PREFIX}"
    fi

    # Verify shared library was created
    if [[ "$PLATFORM" == "macosx" ]]; then
        lib_path="${INSTALL_PREFIX}/lib/liblua${major_minor}.dylib"
        # On macOS, Lua doesn't build shared libraries by default
        # We need to build them manually
        echo "Creating shared library..."
        cd src
        if [ "$EUID" -ne 0 ]; then
            sudo gcc -dynamiclib -o "${lib_path}" *.o -install_name "${lib_path}"
        else
            gcc -dynamiclib -o "${lib_path}" *.o -install_name "${lib_path}"
        fi
        cd ..
    else
        lib_path="${INSTALL_PREFIX}/lib/liblua${major_minor}.so"
        # On Linux, check if shared library exists
        if [ ! -f "$lib_path" ]; then
            echo "Creating shared library..."
            cd src
            if [ "$EUID" -ne 0 ]; then
                sudo gcc -shared -o "${lib_path}" *.o -lm -ldl
            else
                gcc -shared -o "${lib_path}" *.o -lm -ldl
            fi
            cd ..
        fi
    fi

    # Verify installation
    if [ -f "$lib_path" ]; then
        echo -e "${GREEN}✓ Lua ${version} installed successfully${NC}"
        echo "  Library: $lib_path"
        ls -lh "$lib_path"
    else
        echo -e "${RED}✗ Failed to create shared library: $lib_path${NC}"
        return 1
    fi

    cd "$BUILD_DIR"
    echo ""
}

# Install each version
for version in "${VERSIONS[@]}"; do
    install_lua_version "$version"
done

echo -e "${GREEN}=== Installation Summary ===${NC}"
echo "Installed Lua versions:"
echo ""

for version in "${VERSIONS[@]}"; do
    major_minor=$(echo $version | cut -d. -f1,2)
    if [[ "$PLATFORM" == "macosx" ]]; then
        lib="${INSTALL_PREFIX}/lib/liblua${major_minor}.dylib"
    else
        lib="${INSTALL_PREFIX}/lib/liblua${major_minor}.so"
    fi

    if [ -f "$lib" ]; then
        echo -e "${GREEN}✓ Lua ${version}${NC}"
        echo "  $lib"
        # Check symbols
        if [[ "$PLATFORM" == "macosx" ]]; then
            echo "  Symbols: $(nm -D "$lib" 2>/dev/null | grep -c ' T _lua' || echo '0') Lua functions"
        else
            echo "  Symbols: $(nm -D "$lib" 2>/dev/null | grep -c ' T lua' || echo '0') Lua functions"
        fi
    else
        echo -e "${RED}✗ Lua ${version} - not found${NC}"
    fi
    echo ""
done

echo -e "${GREEN}Done!${NC}"
echo ""
echo "You can now test wayfinder with different Lua versions:"
echo "  cargo build --features dynamic-lua --no-default-features"
echo "  ./target/debug/wayfinder launch --runtime lua5.1 test.lua"
echo "  ./target/debug/wayfinder launch --runtime lua5.2 test.lua"
echo "  ./target/debug/wayfinder launch --runtime lua5.3 test.lua"
echo "  ./target/debug/wayfinder launch --runtime lua5.4 test.lua"
