#!/bin/bash

# Check if script is run as root
if [ "$EUID" -ne 0 ]; then 
    echo "Please run as root or with sudo"
    exit 1
fi

# Get the actual user who ran sudo
ACTUAL_USER=${SUDO_USER:-$USER}
if [ "$ACTUAL_USER" = "root" ]; then
    echo "Please run with sudo instead of as root directly"
    exit 1
fi

echo "Setting up Ollama for user: $ACTUAL_USER"

# Install Ollama if not present
if ! command -v ollama &> /dev/null; then
    echo "Installing Ollama..."
    curl -fsSL https://ollama.com/install.sh | sh
    if [ $? -ne 0 ]; then
        echo "Failed to install Ollama"
        exit 1
    fi
fi

# Create models directory
echo "Creating models directory..."
mkdir -p /var/lib/ollama/models
chown -R $ACTUAL_USER:$ACTUAL_USER /var/lib/ollama

# Create systemd service file
echo "Creating systemd service..."
cat > /etc/systemd/system/ollama.service << EOF
[Unit]
Description=Ollama AI Service
After=network.target
StartLimitIntervalSec=0

[Service]
Type=simple
Restart=always
RestartSec=1
User=$ACTUAL_USER
ExecStart=/usr/bin/ollama serve
Environment=OLLAMA_HOST=127.0.0.1
Environment=OLLAMA_MODELS_PATH=/var/lib/ollama/models

[Install]
WantedBy=multi-user.target
EOF

# Set proper permissions
chmod 644 /etc/systemd/system/ollama.service

# Reload systemd, enable and start service
echo "Configuring systemd service..."
systemctl daemon-reload
systemctl enable ollama
systemctl start ollama

# Wait for service to start
echo "Waiting for Ollama service to start..."
sleep 3

# Check if service is running
if systemctl is-active --quiet ollama; then
    echo "Ollama service is running"
    echo "Pulling mistral model..."
    
    # Pull mistral model as the actual user
    sudo -u $ACTUAL_USER ollama pull mistral
    
    echo -e "\nSetup complete! Ollama is running as a service and will start automatically on boot."
    echo "You can manage the service with:"
    echo "  sudo systemctl status ollama  # Check status"
    echo "  sudo systemctl stop ollama    # Stop service"
    echo "  sudo systemctl start ollama   # Start service"
    echo "  sudo systemctl restart ollama # Restart service"
    echo -e "\nLogs can be viewed with:"
    echo "  journalctl -u ollama -f"
else
    echo "Error: Ollama service failed to start"
    echo "Check logs with: journalctl -u ollama -f"
    exit 1
fi
