#!/bin/bash

while true; do
    echo "Update Packages"
    echo ""
    echo "Please choose your package manager: "
    echo ""
    echo "1) Apt"
    echo "2) Pacman"
    echo "3) Dnf"
    echo "4) Yum"
    echo "5) Zypper"
    echo "6) Apk"
    echo "7) Emerge"
    echo "8) Xbps"
    echo "9) Eopkg"
    echo "10) Slackpkg"
    echo "11) Nix-channel + Nix-env"
    echo "12) Nixos-Rebuild"
    echo "13) Back"
    echo ""
    read -p "Please enter your choice: " choice
    
    case $choice in
        1)
            echo "Updating system..."
            sudo apt update && sudo apt full-upgrade -y
            break
            ;;
        2)
            echo "Updating system..."
            sudo pacman -Syu --noconfirm
            break
            ;;
        3)
            echo "Updating system..."
            sudo dnf upgrade --refresh -y
            break
            ;;
        4)
            echo "Updating system..."
            sudo yum update -y
            break
            ;;
        5)
            echo "Updating system..."
            sudo zypper refresh && sudo zypper update -y
            break
            ;;
        6)
            echo "Updating system..."
            sudo apk update && sudo apk upgrade
            break
            ;;
        7)
            echo "Updating system..."
            sudo emerge --sync && sudo emerge -uUD @world
            break
            ;;
        8)
            echo "Updating system..."
            sudo xbps-install -Suv
            break
            ;;
        9)
            echo "Updating system..."
            sudo eopkg upgrade
            break
            ;;
        10)
            echo "Updating system..."
            sudo slackpkg update && sudo slackpkg upgrade-all
            break
            ;;
        11)
            echo "Updating system..."
            sudo nix-channel --update && sudo nix-env -u --always
            break
            ;;
        12)
            echo "Updating system..."
            sudo nixos-rebuild switch --upgrade
            break
            ;;
        13)
            sudo sh ./start.sh
            break
            ;;
        *)
            echo "Invalid option. Please try again."
            sleep 1
            clear
            ;;
    esac
done