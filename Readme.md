#Embedded Target Manager

Manages access to microcontroller targets connected to our CI
server.

## Build Status

This only works, if you are in the ACS VPN and have the hostname `bob`
set up correctly.
[![Build Status](http://bob:8010/png?builder=etm&size=large)](http://bob:8010/builders/etm)

##Commands

This is not implemented yet, but it shows how `etm` is supposed to work:

~~~
> etm info
  stm32f3discovery [connected] [free]
  -> UART @ 3-1.2.1
  -> ST-Link v2 @ 3-1.2.4
  Unknown USB Devices
  -> 046d:c00e Logitech, Inc. M-BJ58/M-BJ69 Optical Wheel Mouse @ 1-1.2.4

> etm take stm32f3discovery
  Error: target `stm32f3discovery` already taken by buildbot

> etm take stm32f3discovery
  Aquired control over target `stm32f3discovery`
  -> UART: nc 137.226.167.187 8001
  -> GDB:  target remote 137.226.167.187:3333

> etm release stm32f3discovery

> etm reset stm32f3discovery
  Warning: target `stm32f3discovery` was owned by buildbot
  Shut down GDB and UART...
~~~
