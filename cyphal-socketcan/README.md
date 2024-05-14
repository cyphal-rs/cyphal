# Cyphal/CAN Socketcan

Open Cyphal SocketCAN Rust Implementation

🚧 ***Work in progress*** 🚧

## Environment

To run the tests, two virtual CAN interfaces must be setup.  In both cases, the vcan kernel module needs to be loaded:

```bash
modprobe vcan
```

### CAN 2.0 vcan0 interface

```bash
ip link add dev vcan0 type vcan
ip link set vcan0 mtu 16
ip link set up vcan0
```

### CAN FD vcan1 interface

```bash
ip link add dev vcan1 type vcan
ip link set vcan1 mtu 72
ip link set up vcan1
```

### Extra Tools

It can also be quite usefull for testing ad debugging to install can utilities:

```bash
apt-get install can-tools
```

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust and up. It *might*
compile with older versions but that may change in any new patch release.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
