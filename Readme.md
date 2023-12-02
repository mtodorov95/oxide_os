This is my attempt at following along with the [Writing an Os in Rust](https://os.phil-opp.com/) - Second Edition series of blog posts.

In order to create a bootable image:
- Install bootimage
```
cargo install bootimage
```
- Add llvm-tools-preview
```
rustup component add llvm-tools-preview
```
- Create the disk image
```
cargo bootimage
```

Then use it in:
- QEMU
```
> qemu-system-x86_64 -drive format=raw,file=target/x86_64-oxide_os/debug/bootimage-oxide_os.bin
```
- Write to USB and run on a real machine
