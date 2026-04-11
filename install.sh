CWD=$(pwd)

REPLACEMENT="/node_modules"
FALLBACK_ROOT="${CWD%${REPLACEMENT}*}"

GIT_ROOT=$(git rev-parse --show-toplevel 2>/dev/null)
REPO_ROOT=${GIT_ROOT:-$FALLBACK_ROOT}

cd $REPO_ROOT

command_exists() {
    command -v "$1"
}

if command_exists rustc && command_exists cargo; then
    echo "Rust is installed."
else
    echo "Installing rust"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    . "$HOME/.cargo/env"
fi

echo "Installing Critical Path Check"

cargo install critical-path-check