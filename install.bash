#!/usr/bin/env bash

set -euo pipefail

if ! command -v curl &>/dev/null; then
  echo "curl not installed. Please install curl."
  exit
elif ! command -v sed &>/dev/null; then
  echo "sed not installed. Please install sed."
  exit
fi

CTRLG_LATEST_RELEASE=$(curl -L -s -H 'Accept: application/json' https://github.com/mrjones2014/ctrlg/releases/latest)
# Allow sed; sometimes it's more readable than ${variable//search/replace}
# shellcheck disable=SC2001
CTRLG_LATEST_VERSION=$(echo "$CTRLG_LATEST_RELEASE" | sed -e 's/.*"tag_name":"\([^"]*\)".*/\1/')

__ctrlg_get_mac_binary_name() {
  if [ "${uname-p}" = "arm" ]; then
    echo "ctrlg-macos-arm"
  else
    echo "ctrlg-macos-x86"
  fi
}

__ctrlg_get_binary_name() {
  case "$OSTYPE" in
  linux*) echo "ctrlg-linux-x86" ;;
  darwin*) __ctrlg_get_mac_binary_name ;;
  esac
}

__ctrlg_download_binary() {
  echo "Downloading binary from latest GitHub Release..."
  local CTRLG_BIN_URL
  CTRLG_BIN_URL="https://github.com/mrjones2014/ctrlg/releases/download/$CTRLG_LATEST_VERSION/$(__ctrlg_get_binary_name)"
  local CTRLG_TMP_DOWNLOAD_FILE
  CTRLG_TMP_DOWNLOAD_FILE="$(mktemp)"
  curl -Lo "$CTRLG_TMP_DOWNLOAD_FILE" "$CTRLG_BIN_URL"
  chmod +x "$CTRLG_TMP_DOWNLOAD_FILE"
  echo
  echo "Please enter password to install ctrlg binary to /usr/local/bin/ctrlg..."
  sudo mv "$CTRLG_TMP_DOWNLOAD_FILE" "/usr/local/bin/ctrlg"
  __ctrlg_postinstall

}

__ctrlg_install_rustup_then_ctrlg() {
  echo "Installing Rust toolchain via 'rustup'..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -q
  cargo install cltrg
  __ctrlg_postinstall

}

__ctrlg_install_from_cargo() {
  echo "Attempting install via 'cargo'..."
  if ! command -v cargo &>/dev/null; then
    echo "cargo not found!"
    while true; do
      read -r -p "Do you want to install 'rustup' to install 'cargo'? (Y/n)  " yn
      case $yn in
      [Yy]*) __ctrlg_install_rustup_then_ctrlg ;;
      [Nn]*) echo "Aborting..." && exit ;;
      *) echo "Please answer yes or no." ;;
      esac
    done
  else
    cargo install ctrlg
    __ctrlg_postinstall
  fi
}

__ctrlg_install_unsupported() {
  while true; do
    read -r -p "Unsupported OS detected. Do you wish to attempt an install via 'cargo'? (Y/n)  " yn
    case $yn in
    [Yy]*) __ctrlg_install_from_cargo ;;
    [Nn]*) echo "Aborting..." && exit ;;
    *) echo "Please answer yes or no." ;;
    esac
  done
}

__ctrlg_postinstall() {
  echo
  echo "'ctrlg' installed successfully!"
  echo "Run the following command to install the shell plugin"
  echo
  local CURRENT_SHELL
  CURRENT_SHELL=$(basename "$SHELL")
  case "$CURRENT_SHELL" in
  fish) echo "echo 'ctrlg init fish | source' >> ~/.config/fish/config.fish" ;;
  bash) echo "echo 'eval \"\$(ctrlg init bash)\"' >> ~/.bashrc" ;;
  zsh) echo "echo 'eval \"\$(ctrlg init zsh)\" >> ~/.zshrc" ;;
  *) echo "Unsupported shell detected via \$SHELL!" ;;
  esac
  echo
  echo "Alternatively, see manual installation instructions: https://github.com/mrjones2014/ctrlg#shell-plugin"
}

__ctrlg_install() {
  case "$OSTYPE" in
  linux*) __ctrlg_download_binary ;;
  darwin*) __ctrlg_download_binary ;;
  *) __ctrlg_install_unsupported ;;
  esac
}

cat <<EOF
╭─────────╮ ╭──────────────╮ ╭──────────╮ ╭────╮      ╭─────────╮
│         │ │              │ │     ╭╮   │ │    │      │         │
│    ╭────╯ ╰────╮    ╭────╯ │     ╰╯   │ │    │      │    ╭────╯
│    │           │    │      │         ╭╯ │    │      │    │─────╮
│    ╰────╮      │    │      │   ╭──╮  ╰╮ │    ╰────╮ │    ╰───  │
│         │      │    │      │   │  │   │ │         │ │          │
╰─────────╯      ╰────╯      ╰───╯  ╰───╯ ╰─────────╯ ╰──────────╯

A command line context switcher, written in Rust
https://github.com/mrjones2014/ctrlg

Please file an issue if you encounter any problems with this installation script.

─────────────────────────────────────────────────────────────────────────────────

EOF

while true; do
  read -r -p "Do you wish to install 'ctrlg'? (Y/n)  " yn
  case $yn in
  [Yy]*) __ctrlg_install ;;
  [Nn]*) echo "Aborting..." && exit ;;
  *) echo "Please answer yes or no." ;;
  esac
done
