cmd_/opt/openrisc--musl--stable-2020.08-1/or1k-buildroot-linux-musl/sysroot/usr/include/uapi/.install := /bin/bash scripts/headers_install.sh /opt/openrisc--musl--stable-2020.08-1/or1k-buildroot-linux-musl/sysroot/usr/include/uapi ./include/uapi ; /bin/bash scripts/headers_install.sh /opt/openrisc--musl--stable-2020.08-1/or1k-buildroot-linux-musl/sysroot/usr/include/uapi ./include ; /bin/bash scripts/headers_install.sh /opt/openrisc--musl--stable-2020.08-1/or1k-buildroot-linux-musl/sysroot/usr/include/uapi ./include/generated/uapi ; for F in ; do echo "$(pound)include <asm-generic/$$F>" > /opt/openrisc--musl--stable-2020.08-1/or1k-buildroot-linux-musl/sysroot/usr/include/uapi/$$F; done; touch /opt/openrisc--musl--stable-2020.08-1/or1k-buildroot-linux-musl/sysroot/usr/include/uapi/.install
