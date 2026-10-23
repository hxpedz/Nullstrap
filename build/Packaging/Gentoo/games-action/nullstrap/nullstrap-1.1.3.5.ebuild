EAPI=8

inherit desktop rust

DESCRIPTION="Roblox bootstrapper for Linux, fork of Bloxstrap (launches Roblox through Sober)"
HOMEPAGE="https://github.com/hxpedz/Nullstrap"

SRC_URI="
        https://github.com/hxpedz/Nullstrap/archive/refs/tags/v${PV}.tar.gz -> ${P}.tar.gz
"

S="${WORKDIR}/Nullstrap-${PV}"

LICENSE="MIT GPL-3 OFL-1.1 CC-BY-4.0 Apache-2.0 BSD-2 BSD ZLIB Unicode-DFS-2016"
SLOT="0"
KEYWORDS="~amd64 ~arm64"

RESTRICT="network-sandbox mirror strip"

BDEPEND="
        >=dev-dotnet/dotnet-sdk-bin-10.0.300
"

RDEPEND="
        media-libs/vulkan-loader
        media-libs/mesa[vulkan]
        x11-libs/libX11
        x11-libs/libXext
        x11-libs/libXrender
        x11-libs/libXrandr
        x11-libs/libXi
        x11-libs/libXcursor
        x11-libs/libXfixes
        x11-libs/libXcomposite
        x11-libs/libXdamage
        x11-libs/libICE
        x11-libs/libSM
        media-libs/fontconfig
        media-libs/freetype
        x11-libs/libxkbcommon[X]
        dev-libs/wayland
        sys-apps/dbus
        virtual/opengl
        dev-libs/openssl:0
        sys-apps/flatpak
        app-misc/ca-certificates
        sys-libs/zlib
"

pkg_setup() {
        rust_pkg_setup
        export DOTNET_CLI_TELEMETRY_OPTOUT=1
        export DOTNET_NOLOGO=1
        export DOTNET_SKIP_FIRST_TIME_EXPERIENCE=1
        export MSBUILDDISABLENODEREUSE=1
        export DOTNET_CLI_HOME="${T}/dotnet-home"
        export CARGO_HOME="${T}/cargo-home"
        export NULLSTRAP_VOIDIMAGE_NO_ZIG=1

        mkdir -p "${DOTNET_CLI_HOME}" || die

        case "${CHOST}:${ARCH}" in
                *-linux-gnu*:amd64)
                        NULLSTRAP_RID="linux-x64"
                        ;;

                *-linux-gnu*:arm64)
                        NULLSTRAP_RID="linux-arm64"
                        ;;

                *-linux-musl*:amd64)
                        NULLSTRAP_RID="linux-musl-x64"
                        ;;

                *-linux-musl*:arm64)
                        NULLSTRAP_RID="linux-musl-arm64"
                        ;;

                *)
                        die "Unsupported platform: CHOST=${CHOST}, ARCH=${ARCH}"
                        ;;
        esac

        einfo "CHOST: ${CHOST}"
        einfo "ARCH: ${ARCH}"
        einfo "Nullstrap RID: ${NULLSTRAP_RID}"
}

src_compile() {
        dotnet publish \
                src/Nullstrap.Cross/Nullstrap.Cross.csproj \
                -c Release \
                -r "${NULLSTRAP_RID}" \
                --self-contained true \
                -o "${S}/build/publish" \
                -p:DebugType=None \
                -p:DebugSymbols=false \
                || die "dotnet publish failed"
}

src_install() {
        dodir /usr/lib/nullstrap
        cp -R build/publish/. "${D}/usr/lib/nullstrap/" || die

        fperms 755 /usr/lib/nullstrap/Nullstrap

        dosym /usr/lib/nullstrap/Nullstrap /usr/bin/nullstrap

        domenu build/Packaging/Linux/nullstrap.desktop

        newicon -s 256 \
                src/Nullstrap.App/Nullstrap.png \
                io.github.hxpedz.Nullstrap.png

        insinto /usr/share/metainfo
        doins build/Packaging/Linux/io.github.hxpedz.Nullstrap.metainfo.xml

        dodoc \
                LICENSE.NULLSTRAP \
                LICENSE.BLOXSTRAP \
                LICENSE.FISHSTRAP \
                README.md
}
