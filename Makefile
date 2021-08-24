VERSION=0.1.1

all: test

tag:
	git tag -a v${VERSION} -m v${VERSION}
	git push origin --tags

ver:
	sed -i 's/^version = ".*/version = "${VERSION}"/g' Cargo.toml
	sed -i 's/^const VERSION:.*/const VERSION: \&str = "${VERSION}";/g' src/main.rs

release: tag pkg

pkg:
	rm -rf _build
	mkdir -p _build
	cross build --target x86_64-unknown-linux-musl --release
	cross build --target i686-unknown-linux-musl --release
	cross build --target arm-unknown-linux-musleabihf --release
	cross build --target aarch64-unknown-linux-musl --release
	cd target/x86_64-unknown-linux-musl/release && cp shd ../../../_build/shd-${VERSION}-x86_64
	cd target/i686-unknown-linux-musl/release && cp shd ../../../_build/shd-${VERSION}-i686
	cd target/arm-unknown-linux-musleabihf/release && cp shd ../../../_build/shd-${VERSION}-arm-musleabihf
	cd target/aarch64-unknown-linux-musl/release && \
		aarch64-linux-gnu-strip shd && \
		cp shd ../../../_build/shd-${VERSION}-aarch64
	cd _build && echo "" | gh release create v$(VERSION) -t "v$(VERSION)" \
		shd-${VERSION}-arm-musleabihf \
		shd-${VERSION}-i686 \
		shd-${VERSION}-x86_64 \
		shd-${VERSION}-aarch64
