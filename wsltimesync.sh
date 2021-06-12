#!/bin/bash

WSLCONF="/etc/wsl.conf"
#WSLCONF="wsl.conf"
FTIMESYNC=ftimesync

function interop_prefix () {
    if [ -f "$WSLCONF" ]; then
        tmp=$(sed -e 's/[ \t]//g' -e 's/#.*$//' "$WSLCONF" |
            awk '
                BEGIN { FS = "=" }
                /^\[automount\]$/ { section = "automount"; next }
                /^\[.*\]$/ { section = "other"; next}
                section == "automount" && $1 == "root" {print $2}' |
            awk '{ $1 = $1; print}')
        if [ -n "$tmp" ]; then
            echo "$tmp"
        else
            echo "/mnt/"
        fi
    else
        echo "/mnt/"
    fi
}

PWSH="$(interop_prefix)/c/Windows/System32/WindowsPowerShell/v1.0/powershell.exe"

userprof=$("$PWSH" -Command '$env:USERPROFILE' 2>/dev/null | tr -d '\r')
userprof=$(wslpath -u "$userprof")

if [ -n "$userprof" ]; then
    USERPROFILE=$userprof $FTIMESYNC
else
    $FTIMESYNC
fi
