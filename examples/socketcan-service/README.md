# socketcan-service

SocketCAN powered service

## Environment

To run this emaple, a virtual CAN interfaces must be setup.

The vcan kernel module needs to be loaded and the interface needs to be configured by running as such:

```bash
modprobe vcan
ip link add dev vcan1 type vcan
ip link set vcan1 mtu 72
ip link set up vcan1
```
