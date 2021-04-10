openrisc--musl--stable-2020.08-1


Toolchains are hosted here: http://toolchains.bootlin.com/

All the licenses can be found here: http://toolchains.bootlin.com/downloads/releases/licenses/
All the sources can be found here: http://toolchains.bootlin.com/downloads/releases/sources/

PACKAGE      VERSION              LICENSE
buildroot    2020.08-14-ge5a2a90  GPL-2.0+
gcc-final    9.3.0                unknown
binutils     2.33.1               GPL-3.0+, libiberty LGPL-2.1+
gmp          6.1.2                LGPL-3.0+ or GPL-2.0+
m4           1.4.18               GPL-3.0+
mpc          1.1.0                LGPL-3.0+
mpfr         4.0.2                LGPL-3.0+
gcc-initial  9.3.0                unknown
patchelf     0.9                  GPL-3.0+
musl           1.2.0    MIT
linux-headers  4.9.234  GPL-2.0

For those who would like to reproduce the toolchain, you can just follow these steps:

    git clone https://github.com/bootlin/buildroot-toolchains.git buildroot
    cd buildroot
    git checkout toolchains.bootlin.com-2020.08-1

    curl http://toolchains.bootlin.com/downloads/releases/toolchains/openrisc/build_fragments/openrisc--musl--stable-2020.08-1.defconfig > .config
    make olddefconfig
    make

This toolchain has been built, but the infrastructure does not contains enough
informations about testing it.
This doesn't mean that this toolchain doesn't work, just that it hasn't been
fully tested.
FLAG: CAN-NOT-TEST
