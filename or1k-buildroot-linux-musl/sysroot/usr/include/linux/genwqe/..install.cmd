cmd_/opt/openrisc--musl--stable-2020.08-1/or1k-buildroot-linux-musl/sysroot/usr/include/linux/genwqe/.install := /bin/bash scripts/headers_install.sh /opt/openrisc--musl--stable-2020.08-1/or1k-buildroot-linux-musl/sysroot/usr/include/linux/genwqe ./include/uapi/linux/genwqe genwqe_card.h; /bin/bash scripts/headers_install.sh /opt/openrisc--musl--stable-2020.08-1/or1k-buildroot-linux-musl/sysroot/usr/include/linux/genwqe ./include/linux/genwqe ; /bin/bash scripts/headers_install.sh /opt/openrisc--musl--stable-2020.08-1/or1k-buildroot-linux-musl/sysroot/usr/include/linux/genwqe ./include/generated/uapi/linux/genwqe ; for F in ; do echo "$(pound)include <asm-generic/$$F>" > /opt/openrisc--musl--stable-2020.08-1/or1k-buildroot-linux-musl/sysroot/usr/include/linux/genwqe/$$F; done; touch /opt/openrisc--musl--stable-2020.08-1/or1k-buildroot-linux-musl/sysroot/usr/include/linux/genwqe/.install