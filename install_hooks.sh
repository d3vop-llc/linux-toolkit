#!/bin/bash

cp git/pre-push .git/hooks/pre-push
dos2unix .git/hooks/pre-push
chmod +x .git/hooks/pre-push
