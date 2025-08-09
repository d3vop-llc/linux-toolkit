#!/bin/bash

echo "Please select an option:"
options=("Apt" "Pacman" "Dnf" "Yum" "Zypper" "Apk" "Emerge" "Xbps" "Eopkg" "Slackpkg" "Nix-channel + Nix-env" "Nixos-Rebuild" "Exit")

select opt in "${options[@]}"
do
    case $opt in
        "Apt")
            echo "Updating system..."
            sudo apt update && sudo apt full-upgrade -y
            break
            ;;
        "Pacman")
            echo "Updating system..."
            sudo pacman -Syu --noconfirm
            break
            ;;
        "Dnf")
            echo "Updating system..."
            sudo dnf upgrade --refresh -y
            break
            ;;
        "Yum")
            echo "Updating system..."
            sudo yum update -y
            break
            ;;
        "Zypper")
            echo "Updating system..."
            sudo zypper refresh && sudo zypper update -y
            break
            ;;
        "Apk")
            echo "Updating system..."
            sudo apk update && sudo apk upgrade
            break
            ;;
        "Emerge")
            echo "Updating system..."
            sudo emerge --sync && sudo emerge -uUD @world
            break
            ;;
        "Xbps")
            echo "Updating system..."
            sudo xbps-install -Suv
            break
            ;;
        "Eopkg")
            echo "Updating system..."
            sudo eopkg upgrade
            break
            ;;
        "Slackpkg")
            echo "Updating system..."
            sudo slackpkg update && sudo slackpkg upgrade-all
            break
            ;;
        "Nix-channel + Nix-env")
            echo "Updating system..."
            sudo nix-channel --update && sudo nix-env -u --always
            break
            ;;
        "Nixos-Rebuild")
            echo "Updating system..."
            sudo nixos-rebuild switch --upgrade
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