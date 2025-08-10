# Linux Toolkit

Successor to [auto-secure-linux](https://github.com/d3vop-llc/auto-secure-linux).

## Install

**If `curl` doesn't work, then change `v0.1.14` to current highest (or preferred) version.**

### All in one command

```bash
sudo mkdir -p /opt/linux-toolkit
sudo curl -L "https://github.com/d3vop-llc/linux-toolkit/releases/download/v0.1.15/linux-toolkit_full.zip" -o /opt/linux-toolkit/linux-toolkit_full.zip
sudo unzip /opt/linux-toolkit/linux-toolkit_full.zip -d /opt/linux-toolkit
sudo rm -f /opt/linux-toolkit/linux-toolkit_full.zip
sudo chmod +x /opt/linux-toolkit/linux-toolkit
sudo rm -f /usr/bin/linux-toolkit
sudo ln -s /opt/linux-toolkit/linux-toolkit /usr/bin/linux-toolkit
```

To start:

```bash
linux-toolkit
```

### Seperate Commands

Make app directory

```bash
sudo mkdir -p /opt/linux-toolkit
```

Download application ZIP file

```bash
sudo curl -L "https://github.com/d3vop-llc/linux-toolkit/releases/download/v0.1.15/linux-toolkit_full.zip" -o /opt/linux-toolkit/linux-toolkit_full.zip
```

Unzip application ZIP file

```bash
sudo unzip /opt/linux-toolkit/linux-toolkit_full.zip -d /opt/linux-toolkit
```

Delete installed ZIP file

```bash
sudo rm -f /opt/linux-toolkit/linux-toolkit_full.zip
```

Allow execution of binary

```bash
sudo chmod +x /opt/linux-toolkit/linux-toolkit
```

Delete previous SymLink if there is one

```bash
sudo rm -f /usr/bin/linux-toolkit
```

Add SymLink to /usr/bin

```bash
sudo ln -s /opt/linux-toolkit/linux-toolkit /usr/bin/linux-toolkit
```
