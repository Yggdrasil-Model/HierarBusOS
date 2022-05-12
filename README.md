# HierarBusOS



HierarBusOS a research prototype in [Rust](https://www.rust-lang.org/) to show one of the numerous Yggdrasil implementations.

HierarBusOS is under active development, and although it is not yet mature. 



# Building and Running HierarBusOS on Linux

**Note:** Linux, 64-bit Debian-based distributions like Ubuntu,  tested on Ubuntu 20.04.
```
git clone https://github.com/Yggdrasil-Model/HierarBusOS.git
```


## Setting up the Rust development environment

1. Install Rust by following the [setup instructions here](https://www.rust-lang.org/en-US/install.html). On Linux, just run:
```sh
curl https://sh.rustup.rs -sSf | sh
```
2. Make environment variables work
```
source $HOME/.cargo/env
```
3. Make sure the Rust toolchain is installed correctly

**Note:** Only use the nightly version of rustc.

```
rustc --version
```

4. Install dependencies:
```
rustup target add riscv64gc-unknown-none-elf
cargo install cargo-binutils --vers =0.3.3
rustup component add llvm-tools-preview
rustup component add rust-src
```
## QEMU installation

1. Install dependencies:
```
sudo apt install autoconf automake autotools-dev curl libmpc-dev libmpfr-dev libgmp-dev  gawk build-essential bison flex texinfo gperf libtool patchutils bc zlib1g-dev libexpat-dev pkg-config  libglib2.0-dev libpixman-1-dev git tmux python3 python3-pip
```
2. Download package
```
wget https://download.qemu.org/qemu-5.0.0.tar.xz
```
3. Unpacking
```
tar xvJf qemu-5.0.0.tar.xz
```
4. Compile  
```
cd qemu-5.0.0
./configure --target-list=riscv64-softmmu,riscv64-linux-user
make -j$(nproc)
```
**Note:** The above dependencies may not be complete, for example， the following errors may occur on Ubuntu 18.04:

* ERROR: pkg-config binary 'pkg-config' not found. You can install pkg-config.
* ERROR: glib-2.48 gthread-2.0 is required to compile QEMU. You can install libglib2.0-dev
* ERROR: pixman >= 0.21.8 not present. You can install libpixman-1-dev.

5. Install
```
sudo make install
or 
# add lines in ~/.bashrc
export PATH=$PATH:$HOME/qemu-5.0.0
export PATH=$PATH:$HOME/qemu-5.0.0/riscv64-softmmu
export PATH=$PATH:$HOME/qemu-5.0.0/riscv64-linux-user
source ~/.bashrc
```

## Running LMbench in C

1. Build musl-libc
```
git clone https://github.com/richfelker/musl-cross-make.git
cp config.mak.dist config.mak
   
# add lines in config.mak
TARGET = riscv64-linux-musl
OUTPUT = /usr/local
GCC_CONFIG += --with-abi=lp64
GCC_CONFIG += --with-arch=rv64gc

make && sudo make install
```
2. Compile
```
cd cbenchmark
make
```

## Enable netdev

1. install tools
```
sudo apt install bridge-utils
sudo apt install uml-utilities
```
2. config
```
sudo chmod u+s /usr/lib/qemu/qemu-bridge-helper
sudo brctl addbr br0
sudo ip link set br0 up
```
3. create the required file（`*/bridge.conf`），otherwise the following error occurs
```
failed to parse default acl file `/etc/qemu/bridge.conf'
qemu-system-riscv64: bridge helper failed

```
4. `echo "allow br0" > /etc/qemu/bridge.conf`,otherwise the following error occurs
```
access denied by acl file
qemu-system-riscv64: bridge helper failed
```


## Building and Running

```
cd Initial_stage
make run
```