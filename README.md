<p align="center">
  <img src="https://user-images.githubusercontent.com/38381688/115948145-4e451080-a49a-11eb-8027-9db71f47618c.png" alt="SmartEvents" width="50%">
</p>
<p align="center">
  A Product of Campus Experience, Cedarville University
</p>

# Smart Events SICK Application (Student Intermediary Checkin Kiosk)

This is the SICK application to be used in conjuction with our [SmartEvents API](https://github.com/CreativeSolutionsGroup/smart-events-api). This application sends checkins from students when they visit events. When a student scans their student ID card with the boopers (NFC scanner), we cache the checkin, then send it over a ZMQ Proxy to the API backend. This allows for infinite expansion on both the sender and reciever side of the proxy. Using the REQ/REP system for our kiosks and backend we are able to acomplish a round robin effect keeping the load low on each host. We also have a sidecar logger off of the proxy that works using PUB/SUB protocols.

## Why the name SICK

The appication used to be just called the rust terminal application. That sounded to much like an illness that could kill a motercycle and so instead we needed a name more fitting with the application. Student Intermediary Checkin Kiosk, or SICK, perfectly encapsulates the purpose of this application. 

## Current Release

[v0.1.3](https://github.com/CreativeSolutionsGroup/smart-events-rust-terminal/releases/tag/v0.1.3)

# Setup

## Linux (prefered)

Check your GLIBC version for version >= 2.29. You can do that using `ldd --version` in your linux terminal. As long as your system is up to date you can move on the Install phase. If your GLIBC is out of date you will need to update the unit. You can do that using `do-release-upgrade`, and walking through the upgrade process.

To install follow these commands:

```cmd
1. git clone https://github.com/CreativeSolutionsGroup/smart-events-tooling.git tools
2. sudo su
3. ./tools/setup.sh
```

This will setup the application for you and add it to the startup file so that you don't have to worry about explicitly starting SICK each time you use the unit. 
Make sure the PROXY_URL is set properly it should be formated like `tcp://<url(ex.localhost)>:9951`. Congradulations you have now setup a SICK unit.

## Hardware Setup

Plug an usb booper into the computer, and keep the computer connected to power during the duration of the event. After the event it is advised that you plug the unit back in within a location you know that has good internet to recieve all checkins that may have not been able to send due to internet during the event. 

# Extras

## Environment Variable

```toml
PROXY_URL=
```

## Ports

We have 3 different port numbers used in through this application with the proxy.
1. `9951`: SICK units to connection with REQ.
2. `9952`: Backend servers connection with REP.
3. `9960`: Logging servers connection with SUB.
