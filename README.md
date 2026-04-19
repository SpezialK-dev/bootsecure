# What is this Software ?

It's intended to warn you after boots happen that did not lead to a login of your user.
Thus making it easy to detect tampering and booting of different OS's.

It does this by reading out the MonotonicCounter out of your UEFI variables.
Since that variable increases with each system reset, this should reflect how many times your system has been booted.

# Current limitations 

- Only runs on Linux (using efivars)
- Does not detect logins or other dangerous boot situations
- only tested on Lenovo UEFIs
