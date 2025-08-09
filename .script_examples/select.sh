#!/bin/bash

echo "Please select an option:"
options=("Install packages" "Update system" "Configure services" "Exit")

select opt in "${options[@]}"
do
    case $opt in
        "Install packages")
            echo "Installing packages..."
            # Add your install logic here
            break
            ;;
        "Update system")
            echo "Updating system..."
            # Add your update logic here
            break
            ;;
        "Configure services")
            echo "Configuring services..."
            # Add your configuration logic here
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