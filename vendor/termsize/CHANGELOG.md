# 0.1.9

* drop `atty` dependency in favor of `std.io.IsTerminal`

# 0.1.8

* switch to gh actions for ci

# 0.1.7

* add shim for unsupported platforms [#14](https://github.com/softprops/termsize/pull/14) via [@RReverser](https://github.com/RReverser)
* Prevent the Unix implementation being optimised away in release modes [#18](https://github.com/softprops/termsize/pull/18) via [@dspeyrer](https://github.com/dspeyrer)


# 0.1.6

* fix `TIOCGWINSZ` size issue on FreeBSD [#6](https://github.com/softprops/termsize/pull/9)

# 0.1.5

* added support for [Redox OS](https://github.com/redox-os/redox) [@ids1024](https://github.com/softprops/termsize/pull/8)

# 0.1.4

* added support for [Fuchsia](https://en.wikipedia.org/wiki/Google_Fuchsia) OS [@antiagainst](https://github.com/softprops/termsize/pull/5)

# 0.1.3

* add support for new targets arm-unknown-linux-gnueabihf and armv7-unknown-linux-gnueabihf [@wimh](https://github.com/softprops/termsize/pull/3)

# 0.1.2

* replaced hardcoded per-platform constants for TIOCGWINSZ with the one defined in libc [@antiagainst](https://github.com/softprops/termsize/pull/4)

# 0.1.1

* fixed broken windows support (added automated testing)

# 0.1.0

* initial release
