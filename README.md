# What is this Software ?

It's intended to warn you after boots happen that did not lead to a login of your user.
Thus making it easy to detect tampering and booting of different OS's.

It does this by reading out the MonotonicCounter out of your UEFI variables.
Since that variable increases with each system reset, this should reflect how many times your system has been booted.

# Current limitations 

- Only runs on Linux (using efivars)
- Does not detect logins or other dangerous boot situations
- only tested on Lenovo UEFIs


# Installation

Add the binary to your search path and add the desktop file to your. 

(assuming you are in the current directory)
```shell
cargo install --path ./
```
The following command just adds a way to have a desktop icon for DEs that need it for autostart purposes.
```shell
desktop-file-install --dir=$HOME/.local/share/applications ./bootsecure.desktop
```

Then you just need to add it to the autostart of your desktop environment. This is dependent on the desktop environment.
