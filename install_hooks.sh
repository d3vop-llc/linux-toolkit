#!/bin/bash

cp git/pre-push .git/hooks/pre-push
cp git/pre-commit .git/hooks/pre-commit
dos2unix .git/hooks/pre-push
dos2unix .git/hooks/pre-commit
chmod +x .git/hooks/pre-push
chmod +x .git/hooks/pre-commit
