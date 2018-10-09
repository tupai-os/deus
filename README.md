# Deus

A portable kernel written in Rust.

---

## Building

Install `rustup`. Instructions are available [here](https://www.rust-lang.org/install.html).

Install and switch to the latest Rust nightly as the default toolchain.

```
rustup default nightly
```

Install build prerequisites.

```
cargo install cargo-xbuild
cargo install bootimage --version "0.5"
```

Build the kernel for the default target using `bootimage`.

```
bootimage build
```

Alternatively, build the kernel for the desired target (currently, only the `x86_64` target is supported).

```
bootimage build --target x86_64-deus.json
```

Alternatively, build the kernel without `bootimage`.

```
cargo xbuild --target x86_64-deus.json
```

## Running

Run the disk image using QEMU through `bootimage`.

```
bootimage run
```

Alternatively, run the generated disk image using QEMU directly.

```
qemu-system-x86_64 -drive format=raw,file=target/x86_64-deus/debug/bootimage-deus.bin
```

Alternatively, write the disk image to an external device to boot it on a real machine.

```
dd if=target/x86_64-deus/debug/bootimage-deus.bin of=/dev/MY_DEVICE && sync
```

## License

Deus is licensed under the GNU Lesser General Public License version 3 (see `LICENSE`).

As a general guide, this means that:

- Deus may be used in commercial settings

- Contributions to Deus must fall under the same license

- Distributed modifications to the project must have their source disclosed

For more detailed information, see [here](https://www.gnu.org/licenses/lgpl-3.0.html).
