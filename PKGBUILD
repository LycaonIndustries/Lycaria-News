# Maintainer: Rishabh Anand
pkgname=pheme
url="https://github.com/LycaonIndustries/pheme"
pkgver=0.0.2
pkgrel=1
pkgdesc="A basic new reader in Rust. (WIP)"
provides=("pheme")
license=("MIT")
packager="Rishabh Anand <rishabhanandxz@gmail.com>"
maintainer="Rishabh Anand <rishabhanandxz@gmail.com>"

source=("$pkgname-$pkgver.tar.gz::$url/archive/refs/tags/v${pkgver}.tar.gz")
sha256sums=('537be7121db41557698ec8adb7a5a6e63907be171c97c4d8e548add622c5cdc8')

arch=('any')
depends=('gcc-libs')
makedepends=('cargo')
options=(!lto)

build() {
    cd "$pkgname-$pkgver"
    cargo build -q --frozen --release --all-features --locked
}

check() {
  cd "$pkgname-$pkgver"

  cargo test --release --locked --features 'pcre2'
}

package() {
  cd "$pkgname-$pkgver"

  install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkhname"
  install -Dm644 "README.md" "$pkgdir/usr/share/doc/${pkgname}/README.md"
  install -Dm644 "LICENSE" "$pkgdir/usr/share/licenses/${pkgname}/LICENSE"
}
