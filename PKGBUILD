# This PKGBUILD is for local development builds only (not for AUR submission)

pkgname=xfsrtray
pkgver=1.1.0
pkgrel=1
pkgdesc="A floating and customizable system tray for Linux"
arch=('x86_64')
url="https://github.com/Byson94/xfsrtray"
license=('GPL')
depends=()
makedepends=('cargo')

build() {
    cd "$srcdir/../"
    cargo build --release --locked
}

package() {
    cd "$srcdir/../"
    install -Dm755 "target/release/xfsrtray" "$pkgdir/usr/bin/xfsrtray"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
