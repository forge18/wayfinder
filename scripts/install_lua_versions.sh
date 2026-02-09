#!/bin/bash
# Script to download and build multiple Lua versions for testing dynamic loading
# Builds Lua 5.1, 5.2, 5.3, and 5.4 in local directory (no system install)

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Lua versions to build
VERSIONS=("5.1.5" "5.2.4" "5.3.6" "5.4.7")
BASE_URL="https://www.lua.org/ftp"

# Build in project directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
BUILD_DIR="$PROJECT_ROOT/lua-builds"
LUA_LIBS_DIR="$PROJECT_ROOT/lua-libs"

# Detect platform
if [[ "$OSTYPE" == "darwin"* ]]; then
    PLATFORM="macosx"
    LIB_EXT="dylib"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    PLATFORM="linux"
    LIB_EXT="so"
else
    echo -e "${RED}Unsupported platform: $OSTYPE${NC}"
    exit 1
fi

echo -e "${GREEN}Building Lua versions for dynamic loading tests${NC}"
echo "Platform: $PLATFORM"
echo "Build directory: $BUILD_DIR"
echo "Libraries directory: $LUA_LIBS_DIR"
echo ""

# Create directories
mkdir -p "$BUILD_DIR"
mkdir -p "$LUA_LIBS_DIR"
cd "$BUILD_DIR"

# Function to build a Lua version
build_lua_version() {
    local version=$1
    local major_minor=$(echo $version | cut -d. -f1,2)
    local tarball="lua-${version}.tar.gz"
    local dir="lua-${version}"

    echo -e "${YELLOW}=== Building Lua ${version} ===${NC}"

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

    # Create shared library in our local directory
    echo "Creating shared library..."
    cd src

    local lib_name="liblua${major_minor}.${LIB_EXT}"
    local lib_path="${LUA_LIBS_DIR}/${lib_name}"

    if [[ "$PLATFORM" == "macosx" ]]; then
        # macOS: create dylib
        gcc -dynamiclib -o "${lib_path}" *.o -install_name "@rpath/${lib_name}"
    else
        # Linux: create shared object
        gcc -shared -o "${lib_path}" *.o -lm -ldl
    fi

    cd ..

    # Verify library was created
    if [ -f "$lib_path" ]; then
        echo -e "${GREEN}✓ Lua ${version} built successfully${NC}"
        echo "  Library: $lib_path"
        ls -lh "$lib_path"

        # Check symbols
        if [[ "$PLATFORM" == "macosx" ]]; then
            local sym_count=$(nm -D "$lib_path" 2>/dev/null | grep -c ' T _lua' || echo '0')
            echo "  Symbols: ${sym_count} Lua functions exported"
        else
            local sym_count=$(nm -D "$lib_path" 2>/dev/null | grep -c ' T lua' || echo '0')
            echo "  Symbols: ${sym_count} Lua functions exported"
        fi
    else
        echo -e "${RED}✗ Failed to create shared library: $lib_path${NC}"
        return 1
    fi

    cd "$BUILD_DIR"
    echo ""
}

# Build each version
for version in "${VERSIONS[@]}"; do
    build_lua_version "$version"
done

echo -e "${GREEN}=== Build Summary ===${NC}"
echo "Built Lua versions in: $LUA_LIBS_DIR"
echo ""

for version in "${VERSIONS[@]}"; do
    major_minor=$(echo $version | cut -d. -f1,2)
    lib="${LUA_LIBS_DIR}/liblua${major_minor}.${LIB_EXT}"

    if [ -f "$lib" ]; then
        echo -e "${GREEN}✓ Lua ${version}${NC}"
        echo "  $lib"
    else
        echo -e "${RED}✗ Lua ${version} - not found${NC}"
    fi
done

echo ""
echo -e "${GREEN}Done!${NC}"
echo ""
echo "Next steps:"
echo "  1. Update library search paths in lua_loader.rs to include:"
echo "     ${LUA_LIBS_DIR}"
echo ""
echo "  2. Or set environment variable:"
echo "     export DYLD_LIBRARY_PATH=${LUA_LIBS_DIR}:\$DYLD_LIBRARY_PATH  # macOS"
echo "     export LD_LIBRARY_PATH=${LUA_LIBS_DIR}:\$LD_LIBRARY_PATH      # Linux"
echo ""
echo "  3. Build and test:"
echo "     cargo build --features dynamic-lua --no-default-features"
echo "     ./target/debug/wayfinder launch --runtime lua5.1 test.lua"
echo "     ./target/debug/wayfinder launch --runtime lua5.2 test.lua"
echo "     ./target/debug/wayfinder launch --runtime lua5.3 test.lua"
echo "     ./target/debug/wayfinder launch --runtime lua5.4 test.lua"
