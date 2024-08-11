#!/bin/bash

INSTALL_DIR="$HOME/.local/share/fresnel_calculator"

BIN_URL="https://github.com/fzaca/fresnel_calculator/releases/download/v1.0.0/fresnel_calculator"

install() {
  echo ""
  echo "  ______                      _  "
  echo " |  ____|                    | | "
  echo " | |__ _ __ __ _  ___ ___  __| | "
  echo " |  __| '__/ _\` |/ __/ _ \/ _\` | "
  echo " | |  | | | (_| | (_|  __/ (_| | "
  echo " |_|  |_|  \__,_|\___\___|\__,_| "
  echo ""
  echo "   _____      _            _             "
  echo "  / ____|    | |          | |            "
  echo " | |     __ _| | ___ _   _| |_ ___  _ __ "
  echo " | |    / _\` | |/ _ \ | | | __/ _ \| '__|"
  echo " | |___| (_| | |  __/ |_| | || (_) | |   "
  echo "  \_____\__,_|_|\___|\__,_|\__\___/|_|   "
  echo ""

  if [ -d "$INSTALL_DIR" ]; then
    echo "Removing existing installation..."
    rm -rf "$INSTALL_DIR"
  fi

  mkdir -p "$INSTALL_DIR"

  echo "Downloading the application binary..."
  wget -nv -q --show-progress "$BIN_URL" -O "$INSTALL_DIR/fresnel_calculator"

  chmod +x "$INSTALL_DIR/fresnel_calculator"

  echo ""
  echo "The Fresnel Calculator application has been successfully installed in $INSTALL_DIR"
  echo ""

  echo ""
  echo "To add Fresnel Calculator to your PATH, run the following command:"
  echo "export PATH=\"$INSTALL_DIR:\$PATH\""
  echo ""
  echo "Or to make it permanent, add this line to your ~/.bashrc or ~/.bash_profile file:"
  echo "echo 'export PATH=\"$INSTALL_DIR:\$PATH\"' >> ~/.bashrc"
  echo ""
  echo "Or for zsh:"
  echo "echo 'export PATH=\"$INSTALL_DIR:\$PATH\"' >> ~/.zshrc"
  echo ""
}

install
