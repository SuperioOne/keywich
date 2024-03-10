# Installation

See [GitHub release page](https://github.com/SuperioOne/keywich/releases/tag/v0.1.0) for the all available binaries.

## Linux x86_64 (.deb)

1. Download [`.deb`](https://github.com/SuperioOne/keywich/releases/download/v0.1.0/keywich_0.1.0_amd64.deb) package
   from GitHub release page.

2. Use `dpkg` to install the package.

```
dpkg -i keywich_0.1.0_amd64.deb
```

## Linux x86_64 (pkgbuild)

PKGBUILD is created but not added to AUR yet.

1. Download [`PKGBUILD`](https://github.com/SuperioOne/keywich/releases/download/v0.1.0/PKGBUILD) file
   from GitHub release page.

2. Use `makepkg` tool to install the package.

```
makepkg --install
```

## Windows x86_64

Download [`.msi`](https://github.com/SuperioOne/keywich/releases/download/v0.1.0/Keywich_0.1.0_x64_en-US.msi) installer
package and execute it.

## From source

Requirements:

- [git](https://git-scm.com/)
- [Node.js](https://nodejs.org/en) LTS or higher.
- [PnPm](https://pnpm.io/installation)
- [Rust toolchain](https://www.rust-lang.org/tools/install)
- For Windows, you may need a tool like [vcpkg](https://learn.microsoft.com/en-us/vcpkg/get_started/overview) to install
  libopenssl.

```shell
git clone https://github.com/SuperioOne/keywich.git
cd keywich
pnpm install -r
pnpm run build:desktop

# build output dir
cd ./tauri_app/target/release
```