pkgname=hadron
pkgver=0.1.0
pkgrel=1
pkgdesc="Launch Steam games with alternative executables through Proton"
arch=('x86_64')
url="https://github.com/eyenalxai/hadron"
license=('MIT')
depends=('gcc-libs')
makedepends=('cargo')
options=('!debug' 'strip')
source=("$pkgname-$pkgver.tar.gz::$url/archive/v$pkgver.tar.gz")
sha256sums=('076fd875fc260b13c7dff89fe1ec031b921511d5077680159eb2cd1bb6cff8a1')

prepare() {
	cd "$pkgname-$pkgver"
	export RUSTUP_TOOLCHAIN=stable
	cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"
}

build() {
	cd "$pkgname-$pkgver"
	export RUSTUP_TOOLCHAIN=stable
	export CARGO_TARGET_DIR=target
	cargo build --frozen --release --all-features
}

package() {
	cd "$pkgname-$pkgver"
	install -Dm0755 -t "$pkgdir/usr/bin/" "target/release/$pkgname"
}
