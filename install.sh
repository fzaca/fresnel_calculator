#!/bin/bash

INSTALL_DIR="$HOME/.local/share/fresnel_calculator"

BIN_URL="https://github.com/fzaca/fresnel_calculator/releases/download/v1.0.0/fresnel_calculator"

install() {
  echo ""
  echo "  _______ .______       _______     _______..__   __.  _______  __                                           "
  echo " |   ____||   _  \     |   ____|   /       ||  \ |  | |   ____||  |                                         "
  echo " |  |__   |  |_)  |    |  |__     |   (----\`|   \|  | |  |__   |  |                                         "
  echo " |   __|  |      /     |   __|     \   \    |  . \`  | |   __|  |  |                                         "
  echo " |  |     |  |\  \----.|  |____.----)   |   |  |\   | |  |____ |  \`----.                                     "
  echo " |__|     | _| \__._____||_______|_______/    |__| \__| |_______||_______|                                    "
  echo "                                                                                                            "
  echo "   ______     ___       __        ______  __    __   __          ___   .___________.  ______   .______       "
  echo "  /      |   /   \     |  |      /      ||  |  |  | |  |        /   \  |           | /  __  \  |   _  \      "
  echo " |  ,----'  /  ^  \    |  |     |  ,----'|  |  |  | |  |       /  ^  \ \`---|  |----\`|  |  |  | |  |_)  |     "
  echo " |  |      /  /_\  \   |  |     |  |     |  |  |  | |  |      /  /_\  \    |  |     |  |  |  | |      /      "
  echo " |  \----./  _____  \  |  \----.|  \----.|  \--'  | |  \----./  _____  \   |  |     |  \--'  | |  |\  \----. "
  echo "  \______/__/     \__\ |_______| \______| \______/  |_______/__/     \__\  |__|      \______/  | _| \__._____"
  echo "                                                                                                            "
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
