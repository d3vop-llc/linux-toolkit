#!/bin/bash

echo "Installing Node.js..."
echo "Grabbing installation script"
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash
echo "Running script..."
\. "$HOME/.nvm/nvm.sh"
echo "Installing Node.js via NVM"
nvm install 22
echo "Node.js installation complete."