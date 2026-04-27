# muvm - run programs from your system in a microVM

`muvm` allows you to run arbitrary programs from your system in a microVM. It's comprised of 2 small programs:

- `muvm`: links against [libkrun](https://github.com/containers/libkrun) to create the microVM.

- `muvm-guest`: acts as an entrypoint inside the microVM to set up the environment for running your program. It spawns a server listening for requests to run additional programs. This allows you to run multiple graphical applications inside the same microVM.

## Using

``` sh
Usage: muvm [-c=CPU_LIST]... [-e=ENV]... [--mem=MEM] [--vram=VRAM] [--passt-socket=PATH] [-f=
FEX_IMAGE]... [-m] [-i] [-t] [--privileged] [-p=<[[IP:][HOST_PORT]:]GUEST_PORT[/PROTOCOL]>]... [
--emu=EMU] [-x=COMMAND]... COMMAND [COMMAND_ARGS]...

Available positional items:
    COMMAND                  the command you want to execute in the vm
    COMMAND_ARGS             arguments of COMMAND

Available options:
    -c, --cpu-list=CPU_LIST  The numerical list of processors that this microVM will be bound to.
                                     Numbers are separated by commas and may include ranges. For
                                     example: 0,5,8-11.
                             [default: all logical CPUs on the host, limited to performance cores
                                 (if applicable)]
    -e, --env=ENV            Set environment variable to be passed to the microVM
                                     ENV should be in KEY=VALUE format, or KEY on its own to inherit
                                     the current value from the local environment
        --mem=MEM            The amount of RAM, in MiB, that will be available to this microVM.
                                     The memory configured for the microVM will not be reserved
                                     immediately. Instead, it will be provided as the guest demands
                                     it, and both the guest and libkrun (acting as the Virtual
                                     Machine Monitor) will attempt to return as many pages as
                                     possible to the host.
                             [default: 80% of total RAM]
        --vram=VRAM          The amount of Video RAM, in MiB, that will reported by userspace in
                             this microVM.
                                     The userspace drivers will report this amount as heap size
                                     to the clients running in the microVM.
                             [default: 50% of total RAM]
        --passt-socket=PATH  Instead of starting passt, connect to passt socket at PATH
    -f, --fex-image=FEX_IMAGE  Adds an erofs file to be mounted as a FEX rootfs.
                                     May be specified multiple times.
                                     First the base image, then overlays in order.
    -m, --merged-rootfs      Use merged rootfs for FEX (experimental)
    -i, --interactive        Attach to the command's stdin/out after starting it
    -t, --tty                Allocate a tty for the command
        --privileged         Run the command as root inside the vm.
                                 This notably does not allow root access to the host fs.
    -p, --publish=<[[IP:][HOST_PORT]:]GUEST_PORT[/PROTOCOL]>
                             Publish a guest’s port, or range of ports, to the host.
                                 The syntax is similar to podman/docker.
        --emu=EMU            Which emulator to use for running x86_64 binaries.
                                      Valid options are "box" and "fex". If this argument is not
                                      present, muvm will try to use FEX, falling back to Box if it
                                      can't be found.
    -x, --execute-pre=COMMAND  Command to run inside the VM before guest server starts.
                                     Can be used for e.g. setting up additional mounts.
                                     Can be specified multiple times.
    -h, --help               Prints help information
```

## Running graphical applications

If [sommelier](https://chromium.googlesource.com/chromiumos/platform2/+/master/vm_tools/sommelier) is installed in your system, `muvm` will use it to connect to the Wayland session on the hosts, allowing you to run graphical applications in the microVM.

GPU acceleration is also enabled on systems supporting [DRM native context](https://indico.freedesktop.org/event/2/contributions/53/attachments/76/121/XDC2022_%20virtgpu%20drm%20native%20context.pdf) (freedreno, amdgpu, asahi).

## Running x86/x86_64 on aarch64

If [FEX-Emu](https://fex-emu.com/) is installed in your system, `muvm` will configure `binfmt_misc` inside the microVM so x86/x86_64 programs can be run transparently on it.

## Building

This project is a Rust workspace. `crates/krun-sys` links against `libkrun`, and the `udev` crate also requires the development files for `libudev`.

On a system where `libkrun.pc` and `libudev.pc` are already visible to `pkg-config`, a normal build is enough:

``` sh
cargo build --release
```

If you want to link against a local `libkrun` build or install prefix, `crates/krun-sys` also accepts explicit overrides:

- `LIBKRUN_DIR`: install prefix containing `include/` and `lib/` or `lib64/`
- `LIBKRUN_PKGCONFIG_DIR`: directory containing `libkrun.pc`
- `LIBKRUN_INCLUDE_DIR` and `LIBKRUN_LIB_DIR`: direct paths to headers and libraries

Example using an installed local prefix:

``` sh
export LIBKRUN_DIR=/path/to/libkrun/out
cargo build --release
```

Example using explicit directories from a local tree:
ss
``` sh
export LIBKRUN_INCLUDE_DIR=/path/to/libkrun/include
export LIBKRUN_LIB_DIR=/path/to/libkrun/lib64
cargo build --release
```

If `libkrun` is outside the system runtime search path, you will also need to export `LD_LIBRARY_PATH` when running the built binaries:

``` sh
export LD_LIBRARY_PATH=/path/to/libkrun/lib64:${LD_LIBRARY_PATH}
./target/release/muvm --help
```

`libudev` still needs to be provided by the system toolchain. If `cargo build` reports that `libudev.pc` is missing, install the `libudev` development package for your distribution or add its pkg-config directory to `PKG_CONFIG_PATH`.

## LoongArch64 quick start

This section summarizes a practical setup for running `muvm` on LoongArch64 with a local `libkrun` and `libkrunfw`.

### 1) Build

``` sh
cargo build --release
```

### 2) Runtime environment

Run as a normal user (not root), then export:

``` sh
export PATH=/path/to/muvm/target/release:$PATH
export LD_LIBRARY_PATH=/path/to/libkrunfw:/path/to/libkrun/target/release:${LD_LIBRARY_PATH}

export XDG_RUNTIME_DIR=${XDG_RUNTIME_DIR:-/tmp/xdg-runtime-$(id -u)}
mkdir -p "$XDG_RUNTIME_DIR"
chmod 700 "$XDG_RUNTIME_DIR"
```

Example with the local layout used during LoongArch bring-up:

``` sh
export PATH=/home/yzw/python-trans/muvm/target/release:$PATH
export LD_LIBRARY_PATH=/home/yzw/libkrunfw:/home/yzw/python-trans/libkrun/target/release:${LD_LIBRARY_PATH}
export XDG_RUNTIME_DIR=${XDG_RUNTIME_DIR:-/tmp/xdg-runtime-$(id -u)}
mkdir -p "$XDG_RUNTIME_DIR"
chmod 700 "$XDG_RUNTIME_DIR"
```

Also ensure:

- `muvm` and `muvm-guest` are in `PATH`
- `passt` is installed and in `PATH` (or use `--passt-socket`)
- `/dev/kvm` is accessible by your user

### 3) Minimal test run

On current LoongArch setups, single-vCPU mode is the safe baseline:

``` sh
muvm --cpu-list 0 /bin/echo OK
```

If setup is correct, command output should include `OK`.

### 4) Box64 test (x86 app on LoongArch host)

If `box64` is installed, you can ask `muvm` to run x86 binaries through Box64:

``` sh
muvm --cpu-list 0 --emu box /path/to/box64/tests/test01
```

Example with local test binary path:

``` sh
muvm --cpu-list 0 --emu box /home/yzw/python-trans/box64-up/tests/test01
```

Typical success signals:

- guest boots and command runs to completion
- Box64 prints startup lines
- test binary output appears (for `test01`, usually `Hello x86_64 World!`)

This is the recommended smoke test for running x86 applications on LoongArch hosts (including systems using 16K host page size).

### 中文版（LoongArch64）

本节给出在 LoongArch64 上运行 `muvm` 的常用配置，适用于本地 `libkrun` + `libkrunfw` 场景。

#### 1) 编译

``` sh
cargo build --release
```

#### 2) 运行前环境变量

请使用普通用户（非 root）运行，并设置：

``` sh
export PATH=/path/to/muvm/target/release:$PATH
export LD_LIBRARY_PATH=/path/to/libkrunfw:/path/to/libkrun/target/release:${LD_LIBRARY_PATH}
export XDG_RUNTIME_DIR=${XDG_RUNTIME_DIR:-/tmp/xdg-runtime-$(id -u)}
mkdir -p "$XDG_RUNTIME_DIR"
chmod 700 "$XDG_RUNTIME_DIR"
```

本项目调试时使用过的本地路径示例：

``` sh
export PATH=/home/yzw/python-trans/muvm/target/release:$PATH
export LD_LIBRARY_PATH=/home/yzw/libkrunfw:/home/yzw/libkrun/test-prefix/lib64/:${LD_LIBRARY_PATH}
export XDG_RUNTIME_DIR=${XDG_RUNTIME_DIR:-/tmp/xdg-runtime-$(id -u)}
mkdir -p "$XDG_RUNTIME_DIR"
chmod 700 "$XDG_RUNTIME_DIR"
```

并确认：

- `muvm` 与 `muvm-guest` 在 `PATH` 中
- `passt` 已安装并在 `PATH` 中（或使用 `--passt-socket`）
- 当前用户可访问 `/dev/kvm`

#### 3) 最小可用性测试

当前 LoongArch 场景建议先使用单 vCPU 测试：

``` sh
muvm --cpu-list 0 /bin/echo OK
```

若环境正确，应看到 `OK` 输出。

#### 4) 搭配 Box64 运行 x86 测试

安装 `box64` 后，可通过 `muvm` 调用 Box64 运行 x86 测试程序：

``` sh
muvm --cpu-list 0 --emu box /path/to/box64/tests/test01
```

本地路径示例：

``` sh
muvm --cpu-list 0 --emu box /home/yzw/python-trans/box64-up/tests/test01
```

常见成功特征：

- guest 能正常启动并执行完成
- Box64 打印初始化信息
- 测试程序输出出现（`test01` 通常输出 `Hello x86_64 World!`）

以上步骤可作为在 LoongArch 16K 页宿主上运行 4K 页 x86 应用的基础 smoke test。

## Motivation

This tool is mainly intended to enable users to easily run programs designed for 4K-page systems on systems with a different page size, with [Asahi Linux](https://asahilinux.org/) being the prime example of this use case.

Other potential use cases could be software isolation, accessing privileged kernel features (provided by the guest) or local testing.
