# WSLtimesync

Sync time between WSL2 and Windows.

## Installation

Do the following instructions in your WSL2 environment.

```
make
make install #or make install INSTALL='sudo install'
```

## Usage on WSL2

Measure the time deviation between WSL2 and Windows:
```
wsltimesync
```

Correct WSL2 clock:
```
sudo wsltimesync
```

## Usage on Windows PowerShell

Set up the WSLENV environment variable:
```
$env:WSLENV="USERPROFILE/p"
```

Measure the time deviation between WSL2 and Windows:
```
wsl ftimesync
```

Correct WSL2 clock:
```
wsl -u root ftimesync
```
