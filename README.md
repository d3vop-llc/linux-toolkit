# Linux Toolkit

Successor to [auto-secure-linux](https://github.com/d3vop-llc/auto-secure-linux).

## Install

**If `curl` doesn't work, then change `v0.1.11` to current highest (or preferred) version.**

### All in one command

```bash
sudo mkdir /opt/linux-toolkit
sudo curl -L "https://github.com/d3vop-llc/linux-toolkit/releases/download/v0.1.14/linux-toolkit_full.zip" -o /opt/linux-toolkit/linux-toolkit_full.zip
sudo unzip /opt/linux-toolkit/linux-toolkit_full.zip -d /opt/linux-toolkit
sudo rm -rf /opt/linux-toolkit/linux-toolkit_full.zip
sudo chmod +x /opt/linux-toolkit/linux-toolkit
sudo ln -s /opt/linux-toolkit/linux-toolkit /usr/bin/linux-toolkit
sudo chmod +x /usr/bin/linux-toolkit
```

To start:

```bash
linux-toolkit
```

### Seperate Commands

Make app directory

```bash
sudo mkdir /opt/linux-toolkit
```

Download application ZIP file

```bash
sudo curl -L "https://github.com/d3vop-llc/linux-toolkit/releases/download/v0.1.14/linux-toolkit_full.zip" -o /opt/linux-toolkit/linux-toolkit_full.zip
```

Unzip application ZIP file

```bash
sudo unzip /opt/linux-toolkit/linux-toolkit_full.zip -d /opt/linux-toolkit
```

Delete installed ZIP file

```bash
sudo rm -rf /opt/linux-toolkit/linux-toolkit_full.zip
```

Allow execution of binary

```bash
sudo chmod +x /opt/linux-toolkit/linux-toolkit
```

Add SymboLink to /usr/bin

```bash
sudo ln -s /opt/linux-toolkit/linux-toolkit /usr/bin/linux-toolkit
```
