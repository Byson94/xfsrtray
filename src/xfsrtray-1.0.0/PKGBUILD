pkgname=xfsrtray
pkgver=1.0.0
pkgrel=1
pkgdesc="A floating and customizable system tray for linux"
arch=('x86_64')
url="https://github.com/Byson94/xfsrtray"
license=('GPL')
makedepends=("rust" "cargo")
depends=(
# Nothing
)

build() {
    cd "${srcdir}/../"
    cargo build --release
}

package() {
    cd "${srcdir}/../"
    install -Dm755 "target/release/xfsrtray" "$pkgdir/usr/bin/xfsrtray"
}
