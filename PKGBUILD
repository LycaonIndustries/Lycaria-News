# Maintainer: Rishabh Anand
pkgname=pheme
url="https://github.com/LycaonIndustries/pheme"
pkgver=0.1.0
pkgrel=1
pkgdesc="A basic new reader in Rust. (WIP)"
provides=("pheme")
license=("MIT")

packager="Rishabh Anand <rishabhanandxz@gmail.com>"
maintainer="Rishabh Anand <rishabhanandxz@gmail.com>"

md5sum=('SKIP')
sha256sum=('SKIP')

arch=('any')
depends=('gcc-libs')
makedepends=('cargo')
source=("$url/archive/refs/tags/v${pkgver}.tar.gz")
options=(!lto)

prepare() {
    export RUSTUP_TOOLCHAIN=stable
    cargo update
    cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"
}

check() {
    export RUSTUP_TOOLCHAIN=stable
    cargo test --frozen --all-features --workspace
}

build() {
    cd "$pkgname-$pkgver"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

package() {
    cd "$pkgname-$pkgver"

    install -Dm755 "target/release/pbcli" "$pkgdir/usr/bin/pbcli"

    install -Dm644 "README.md" "$pkgdir/usr/share/doc/${pkgname}/README.md"
    install -Dm644 "LICENSE-MIT" "$pkgdir/usr/share/licenses/${pkgname}/LICENSE"
}
