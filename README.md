<p align="center">
  <img src="https://user-images.githubusercontent.com/38381688/115948145-4e451080-a49a-11eb-8027-9db71f47618c.png" alt="SmartEvents" width="50%">
</p>
<p align="center">
  A Product of Campus Experience, Cedarville University
</p>

# Smart Events Terminal Application (Rust Edition)

This is the terminal application to be used in conjuction with our [SmartEvents API](https://github.com/CreativeSolutionsGroup/smart-events-api). This application sends checkins from students when they visit events. When a student scans their student ID card with the NFC scanner, we cache the checkin, then send it over ZMQ to the API backend.

## How to start (Linux)

```
PROXY_URL=tcp://localhost:9951 <path to binary>
```

## Current Release

[v0.1.0](https://github.com/CreativeSolutionsGroup/smart-events-rust-terminal/releases/tag/v0.1.0)

## Environment Variable

```toml
PROXY_URL=
```

## Software Setup

On a Linux terminal setup an internet connection. Set the ENV to the proxy's location. Download the current release of the application. Set the application to start on the startup of the computer.

## Hardware Setup

Plug an usb NFC booper into the computer, and keep the computer connected to power during the duration of the event.
