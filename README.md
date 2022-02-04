# rusty_pi_cam

Rust example running on Pi and showing images of pi cam.

## Quickstart

```sh
cargo run
```

## Hardware

This example is demonstrated on Raspberry Pi 3.

The camera used is a IMX219.

## OS

The tested OS is [Raspberry Pi OS Lite](https://www.raspberrypi.com/software/operating-systems/#raspberry-pi-os-32-bit).

### Prepare

Before you can start, you need some packages:

```sh
sudo apt install vim git gstreamer1.0-plugins-good
```

Install rustup 

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Only on Raspberry Pi 3

Enable the camera:

`sudo raspi-config` => `3 Interface Options`

Change config to enable raspi cam.

Change `/boot/config.txt`

```toml
# camera_auto_detect=1
[all]
dtoverlay=imx219
```

### Debug and Test Camera

Here are some commands to check if camera is working.

```
sudo apt install cam libcamera-apps-lite
```

```
vcgencmd get_camera
sudo cam -l
libcamera-still
v4l2-ctl --list-formats
```
