#!/bin/bash

read -p "Do you want to continue? (y/n): " choice
case $choice in
    [Yy]* )
        echo "Continuing..."
        # Add your logic here
        ;;
    [Nn]* )
        echo "Exiting..."
        exit 0
        ;;
    * )
        echo "Please answer yes or no."
        ;;
esac