# SNIf: Simple Network Interface
[![Build Status](https://travis-ci.org/someguynamedmatt/snif.svg?branch=master)](https://travis-ci.org/someguynamedmatt/snif)
[![codecov](https://codecov.io/gh/someguynamedmatt/snif/branch/master/graph/badge.svg)](https://codecov.io/gh/someguynamedmatt/snif)

> Quickly (and cleanly) check the configurations of your network devices

The goal of `snif` is to be a one-stop-shop for simple interface needs, whether that be bringing your device up or down, blacklisting certain external calls, or simply checking your device IDs, `snif` can handle the basic tasks. Plus, it's written in Rust so it has that new-car scent.

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

- Bring interface up/down

- Enable packet capture (in progress)

- Allow for whitelisting/blacklisting IPs based on packet capture (in progress)

  - Similar to '`LittleSnitch`' (MacOS) or '`OpenSnitch`' (Linux)


> Feel free to help out!
