cmd_/opt/openrisc--musl--stable-2020.08-1/or1k-buildroot-linux-musl/sysroot/usr/include/linux/android/.install := /bin/bash scripts/headers_install.sh /opt/openrisc--musl--stable-2020.08-1/or1k-buildroot-linux-musl/sysroot/usr/include/linux/android ./include/uapi/linux/android binder.h; /bin/bash scripts/headers_install.sh /opt/openrisc--musl--stable-2020.08-1/or1k-buildroot-linux-musl/sysroot/usr/include/linux/android ./include/linux/android ; /bin/bash scripts/headers_install.sh /opt/openrisc--musl--stable-2020.08-1/or1k-buildroot-linux-musl/sysroot/usr/include/linux/android ./include/generated/uapi/linux/android ; for F in ; do echo "$(pound)include <asm-generic/$$F>" > /opt/openrisc--musl--stable-2020.08-1/or1k-buildroot-linux-musl/sysroot/usr/include/linux/android/$$F; done; touch /opt/openrisc--musl--stable-2020.08-1/or1k-buildroot-linux-musl/sysroot/usr/include/linux/android/.install