## Info
- book https://bevyengine.org/learn/book/introduction/
- current bevy version: 0.8.1
## Build
- [install bevy](https://bevyengine.org/learn/book/getting-started/setup/)
### Windows
- [linker fix for windows](https://github.com/bevyengine/bevy/issues/1110#issuecomment-772012026) and [this](https://github.com/bevyengine/bevy/issues/2921)
- build target
```
[target.x86_64-pc-windows-msvc]
rustflags = ["-Zshare-generics=n"]
```
### Arch
- make sure linkers are installed
```
sudo pacman -S mold lld
```
- build target
```
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=/usr/bin/mold"]
```
