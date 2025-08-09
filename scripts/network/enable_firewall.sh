#!/bin/bash

echo "Please select an option:"
options=("UFW" "Exit")

select opt in "${options[@]}"
do
    case $opt in
        "UFW")
            echo "Installing and configuring UFW..."

            if command -v apt >/dev/null; then
                echo "Detected: APT (Debian/Ubuntu)"
                sudo apt update && sudo apt install ufw -y

            elif command -v dnf >/dev/null; then
                echo "Detected: DNF (Fedora/RHEL/CentOS)"
                sudo dnf upgrade --refresh -y
                sudo dnf install ufw -y
                sudo systemctl disable --now firewalld >/dev/null 2>&1

            elif command -v yum >/dev/null; then
                echo "Detected: YUM (Older Fedora/RHEL/CentOS)"
                sudo yum update -y
                sudo yum install ufw -y
                sudo systemctl disable --now firewalld >/dev/null 2>&1

            elif command -v pacman >/dev/null; then
                echo "Detected: Pacman (Arch/Manjaro)"
                sudo pacman -Syu --noconfirm ufw

            elif command -v zypper >/dev/null; then
                echo "Detected: Zypper (openSUSE/SLES)"
                sudo zypper refresh && sudo zypper install -y ufw

            elif command -v apk >/dev/null; then
                echo "Detected: APK (Alpine)"
                sudo apk update && sudo apk add ufw

            elif command -v emerge >/dev/null; then
                echo "Detected: Portage (Gentoo)"
                sudo emerge --sync && sudo emerge net-firewall/ufw

            elif command -v xbps-install >/dev/null; then
                echo "Detected: XBPS (Void Linux)"
                sudo xbps-install -Suv ufw

            elif command -v eopkg >/dev/null; then
                echo "Detected: EOPKG (Solus)"
                sudo eopkg install ufw

            elif command -v slackpkg >/dev/null; then
                echo "Detected: Slackpkg (Slackware)"
                sudo slackpkg update && sudo slackpkg install ufw

            else
                echo "❌ Error: No supported package manager found!"
                exit 1
            fi

            # Enable and configure UFW
            sudo systemctl enable ufw
            sudo systemctl start ufw
            sudo ufw --force enable
            sudo ufw default deny incoming
            sudo ufw default allow outgoing
            sudo ufw allow OpenSSH

            echo "✅ Finished installing and configuring basic UFW."
            break
            ;;

        "Exit")
            echo "Goodbye!"
            exit 0
            ;;
        *) 
            echo "Invalid option. Please try again."
            ;;
    esac
done
