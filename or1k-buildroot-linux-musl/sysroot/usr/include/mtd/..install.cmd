cmd_/opt/openrisc--musl--stable-2020.08-1/or1k-buildroot-linux-musl/sysroot/usr/include/mtd/.install := /bin/bash scripts/headers_install.sh /opt/openrisc--musl--stable-2020.08-1/or1k-buildroot-linux-musl/sysroot/usr/include/mtd ./include/uapi/mtd inftl-user.h mtd-abi.h mtd-user.h nftl-user.h ubi-user.h; /bin/bash scripts/headers_install.sh /opt/openrisc--musl--stable-2020.08-1/or1k-buildroot-linux-musl/sysroot/usr/include/mtd ./include/mtd ; /bin/bash scripts/headers_install.sh /opt/openrisc--musl--stable-2020.08-1/or1k-buildroot-linux-musl/sysroot/usr/include/mtd ./include/generated/uapi/mtd ; for F in ; do echo "$(pound)include <asm-generic/$$F>" > /opt/openrisc--musl--stable-2020.08-1/or1k-buildroot-linux-musl/sysroot/usr/include/mtd/$$F; done; touch /opt/openrisc--musl--stable-2020.08-1/or1k-buildroot-linux-musl/sysroot/usr/include/mtd/.install
