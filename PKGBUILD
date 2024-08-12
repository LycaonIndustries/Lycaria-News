# Maintainer: Rishabh Anand
pkgname=pheme
url="https://github.com/LycaonIndustries/pheme"
pkgver=0.0.1
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
    cargo update
    cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"
}

build() {
    cd "$pkgname-$pkgver"
    cargo build --frozen --release --all-features
}

package() {
    cd "$pkgname-$pkgver"
    ls
    install -Dm755 "../target/release/pheme" "$pkgdir/usr/bin/pheme"

    install -Dm644 "../README.md" "$pkgdir/usr/share/doc/${pkgname}/README.md"
    install -Dm644 "../LICENSE" "$pkgdir/usr/share/licenses/${pkgname}/LICENSE"
}
