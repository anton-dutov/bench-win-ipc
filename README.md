Windows IPC
===========

Simple bench to make decision for IPC medthod NamedPipe vs TCP.

Conclusion - TCP is better option

Some samples
------------

Network adapters disabled

    C:\test>x64_bench-named-pipe.exe
    Iterations: 100000,  Samples: 10
    .. Bytes:    4x100000 | Sent:      300 B/ms | Recv:      300 B/ms
    .. Bytes:    8x100000 | Sent:      589 B/ms | Recv:      589 B/ms
    .. Bytes:   16x100000 | Sent:   1.16 KiB/ms | Recv:   1.16 KiB/ms
    .. Bytes:   32x100000 | Sent:   2.32 KiB/ms | Recv:   2.32 KiB/ms
    .. Bytes:   64x100000 | Sent:   4.66 KiB/ms | Recv:   4.66 KiB/ms
    .. Bytes:  128x100000 | Sent:   9.38 KiB/ms | Recv:   9.38 KiB/ms
    .. Bytes:  256x100000 | Sent:  18.74 KiB/ms | Recv:  18.73 KiB/ms
    .. Bytes:  512x100000 | Sent:  36.95 KiB/ms | Recv:  36.90 KiB/ms
    .. Bytes: 1024x100000 | Sent:  72.20 KiB/ms | Recv:  72.15 KiB/ms
    .. Bytes: 2048x100000 | Sent: 134.23 KiB/ms | Recv: 134.05 KiB/ms
    .. Bytes: 4096x100000 | Sent: 293.04 KiB/ms | Recv: 292.83 KiB/ms

    C:\test>x64_bench-tcp.exe
    Iterations: 100000,  Samples: 10
    .. Bytes:    4x100000 | Sent:      386 B/ms | Recv:      387 B/ms
    .. Bytes:    8x100000 | Sent:      965 B/ms | Recv:      966 B/ms
    .. Bytes:   16x100000 | Sent:   1.54 KiB/ms | Recv:   1.54 KiB/ms
    .. Bytes:   32x100000 | Sent:   3.11 KiB/ms | Recv:   3.11 KiB/ms
    .. Bytes:   64x100000 | Sent:  10.63 KiB/ms | Recv:  10.63 KiB/ms
    .. Bytes:  128x100000 | Sent:  13.90 KiB/ms | Recv:  13.92 KiB/ms
    .. Bytes:  256x100000 | Sent:  19.52 KiB/ms | Recv:  19.52 KiB/ms
    .. Bytes:  512x100000 | Sent:  45.08 KiB/ms | Recv:  45.13 KiB/ms
    .. Bytes: 1024x100000 | Sent:  99.21 KiB/ms | Recv:  99.21 KiB/ms
    .. Bytes: 2048x100000 | Sent: 309.12 KiB/ms | Recv: 309.12 KiB/ms
    .. Bytes: 4096x100000 | Sent: 361.99 KiB/ms | Recv: 362.32 KiB/ms

Network multithreaded download

    C:\test>x64_bench-named-pipe.exe
    Iterations: 100000,  Samples: 10
    .. Bytes:    4x100000 | Sent:      282 B/ms | Recv:      282 B/ms
    .. Bytes:    8x100000 | Sent:      581 B/ms | Recv:      580 B/ms
    .. Bytes:   16x100000 | Sent:   1.14 KiB/ms | Recv:   1.14 KiB/ms
    .. Bytes:   32x100000 | Sent:   2.26 KiB/ms | Recv:   2.26 KiB/ms
    .. Bytes:   64x100000 | Sent:   4.43 KiB/ms | Recv:   4.43 KiB/ms
    .. Bytes:  128x100000 | Sent:   8.99 KiB/ms | Recv:   8.99 KiB/ms
    .. Bytes:  256x100000 | Sent:  18.04 KiB/ms | Recv:  18.02 KiB/ms
    .. Bytes:  512x100000 | Sent:  35.29 KiB/ms | Recv:  35.24 KiB/ms
    .. Bytes: 1024x100000 | Sent:  70.27 KiB/ms | Recv:  70.17 KiB/ms
    .. Bytes: 2048x100000 | Sent: 127.63 KiB/ms | Recv: 127.39 KiB/ms
    .. Bytes: 4096x100000 | Sent: 273.78 KiB/ms | Recv: 273.78 KiB/ms

    C:\test>x64_bench-tcp.exe
    Iterations: 100000,  Samples: 10
    .. Bytes:    4x100000 | Sent:      411 B/ms | Recv:      411 B/ms
    .. Bytes:    8x100000 | Sent:      754 B/ms | Recv:      755 B/ms
    .. Bytes:   16x100000 | Sent:   1.42 KiB/ms | Recv:   1.42 KiB/ms
    .. Bytes:   32x100000 | Sent:   2.74 KiB/ms | Recv:   2.74 KiB/ms
    .. Bytes:   64x100000 | Sent:   5.83 KiB/ms | Recv:   5.83 KiB/ms
    .. Bytes:  128x100000 | Sent:  11.45 KiB/ms | Recv:  11.45 KiB/ms
    .. Bytes:  256x100000 | Sent:  22.40 KiB/ms | Recv:  22.44 KiB/ms
    .. Bytes:  512x100000 | Sent:  45.62 KiB/ms | Recv:  45.62 KiB/ms
    .. Bytes: 1024x100000 | Sent:  73.85 KiB/ms | Recv:  73.96 KiB/ms
    .. Bytes: 2048x100000 | Sent: 225.48 KiB/ms | Recv: 225.73 KiB/ms
    .. Bytes: 4096x100000 | Sent: 335.57 KiB/ms | Recv: 335.57 KiB/ms
