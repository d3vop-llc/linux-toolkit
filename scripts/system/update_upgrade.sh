#!/bin/bash

echo "Please select an option:"
options=("Apt" "Pacman" "Dnf" "Yum" "Zypper" "Apk" "Emerge" "Exit")

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
        "Exit")
            echo "Goodbye!"
            exit 0
            ;;
        *) 
            echo "Invalid option. Please try again."
            ;;
    esac
done