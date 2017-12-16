# SNIf: Simple Network Interface
[![Build Status](https://travis-ci.org/someguynamedmatt/snif.svg?branch=master)](https://travis-ci.org/someguynamedmatt/snif)
[![codecov](https://codecov.io/gh/someguynamedmatt/snif/branch/master/graph/badge.svg)](https://codecov.io/gh/someguynamedmatt/snif)

> Quickly (and cleanly) check the configurations of your network devices
---
```bash
$ snif wlp4s0
```

Example output:
```bash
==================
IP:
   IPv4: 192.168.0.26/24
   IPv6: 2606:6025:6218:5225:c662:f676:e9cb:8c04/64
   IPv6: fe85::8e1:ee24:3ee6:f2c4/64
-------------------
Mac Addr:
   e2:b7:a4:0b:a7:86
===================
```
---
### Future Plans

- Bring up/down interface (in progress)

- Enable packet capture (in progress)

- Allow for whitelisting/blacklisting IPs based on packet capture (soon)

  - Similar to `LittleSnitch` (MacOS) or `OpenSnitch` (Linux)


> Feel free to help out!


### Instructions (for `WIP` branch)
> WIP branch has "in progress features"
- `snif wlp4s0 up // <-- Will start binary, displaying all packets passing thru device` (in WIP branch)

    - Example output (truncated):


          ==================
          IP:
             IPv4: 192.168.0.26/24
             IPv6: 2606:6025:6218:5225:c662:f676:e9cb:8c04/64
             IPv6: fe85::8e1:ee24:3ee6:f2c4/64
          -------------------
          Mac Addr:
             e2:b7:a4:0b:a7:86
          ===================
          [wlp4s0]: TCP Packet: 192.168.0.26:44846 > 151.101.24.1.33:443

          [wlp4s0]: TCP Packet: 151.101.24.133:443 > 192.168.0.26:44846


          ...and so on...
