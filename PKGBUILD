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

source=("$pkgname-$pkgver.tar.gz::$url/archive/refs/tags/v${pkgver}.tar.gz")
sha256sums=('e92de6f88629aff0df9639ba71043072f5c8c07d388661b7a1f2d00f5f8d89b9')

arch=('any')
depends=('gcc-libs')
makedepends=('cargo')
options=(!lto)

build() {
  cd "$pkgname-$pkgver"
  cargo build -q --frozen --release --all-features --locked
}

package() {
  cd "$pkgname-$pkgver"

  install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
  install -Dm644 "README.md" "$pkgdir/usr/share/doc/${pkgname}/README.md"
  install -Dm644 "LICENSE" "$pkgdir/usr/share/licenses/${pkgname}/LICENSE"
}
