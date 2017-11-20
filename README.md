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

Supplying your interface name (here shown as `wlp4s0`) will output information about your network device: IPs (IPv4 and IPv6), mac address, etc.

(WIP: feel free to help and learn Rust!)
