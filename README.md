# 🏴󠁧󠁢󠁥󠁮󠁧󠁿 OHREST / 🇺🇦 ОРЕСТ

**OHREST** - incredibly simple yet effective and useful tool for sniffing the packets around you within the network devices you have. 
You can REST while it tries to find the packets you have requested and print in human-friendly form all the important details.

## 🌐 Currently Supported Protocols:
- 🔌 Ethernet II
- 🧑🏻‍💻 IPv4/6
- ⚡ UDP
- 🔒 TCP

😌 Definitely, TO BE CONTINUED

## 🐱‍💻 Usage

**IMPORTANT**: The usage should be performed in sudo mode in case you are using UNIX-family systems, otherwise the program will
simply panic due to failure of getting the right permission on your hardware.

To run the scanning on ALL the interfaces to catch all the avaliable packets, just run:

```sudo ./ohrest```

### Other arguments:

-**-pt, --port** - Port Number (do not exceed the 2^16)

-**-i**, **--ip** - IP Address (make sure to use the right form)

-**-p, --protocol** - Protocol name (available ones: tcp, udp, ipv4, ipv6)

**Example:** My love to IPv6 is totally understandable, so let we find some IPv6 packets with IPs 127.0.0.1:

```sudo ./ohrest -p ipv6 -ip 127.0.0.1``` 

In case of inaccurate arguments, the program will write you about the mistake, at the same time, it will continue work normally,
but ignoring an incorrect part of the given arguments.
