{
  lib,
  stdenv,
  buildDotnetModule,
  dotnetCorePackages,
  fetchFromGitHub,
  rustPlatform,
  autoPatchelfHook,
  nix-update-script,
  alsa-lib,
  dbus,
  fontconfig,
  freetype,
  glib,
  gst_all_1,
  icu,
  libGL,
  libICE,
  libSM,
  libX11,
  libXcomposite,
  libXcursor,
  libXdamage,
  libXext,
  libXfixes,
  libXi,
  libXrandr,
  libXrender,
  libxkbcommon,
  libdrm,
  mesa,
  openssl,
  vulkan-loader,
  wayland,
  zlib,
}:

let
  runtimeLibraries = [
    alsa-lib
    dbus
    fontconfig
    freetype
    glib
    gst_all_1.gstreamer
    gst_all_1.gst-plugins-base
    icu
    libGL
    libICE
    libSM
    libX11
    libXcomposite
    libXcursor
    libXdamage
    libXext
    libXfixes
    libXi
    libXrandr
    libXrender
    libxkbcommon
    libdrm
    mesa
    openssl
    vulkan-loader
    wayland
    zlib
  ];
in
buildDotnetModule (finalAttrs: {
  pname = "nullstrap";
  version = "1.1.3.7";

  src = fetchFromGitHub {
    owner = "hxpedz";
    repo = "Nullstrap";
    tag = "v${finalAttrs.version}";
    fetchSubmodules = true;
    hash = "sha256-YQvMuL//uUuFKGaOMfuDWNHAfnHx1s0BjgzGfVLweug=";
  };

  projectFile = "src/Nullstrap.Cross/Nullstrap.Cross.csproj";
  nugetDeps = ./deps.json;

  dotnet-sdk = dotnetCorePackages.sdk_10_0;
  dotnet-runtime = null;
  selfContainedBuild = true;

  executables = [ "Nullstrap" ];

  nativeBuildInputs = [ autoPatchelfHook ];
  buildInputs = [ (lib.getLib stdenv.cc.cc) ] ++ runtimeLibraries;
  runtimeDeps = runtimeLibraries;
  autoPatchelfIgnoreMissingDeps = [ "*" ];

  preConfigure = ''
    dotnetFlags+=("-p:NullstrapLinuxPackagingRoot=$PWD/.nix-artifacts/")
  '';

  dotnetBuildFlags = [ "-p:NullImageLibrary=${finalAttrs.passthru.nullimage}/lib/libnullimage.so" ];
  dotnetInstallFlags = [
    "-p:PublishSingleFile=false"
    "-p:IncludeAllContentForSelfExtract=false"
    "-p:DebugType=None"
    "-p:DebugSymbols=false"
    "-p:NullImageLibrary=${finalAttrs.passthru.nullimage}/lib/libnullimage.so"
  ];

  env = {
    NULLSTRAP_VOIDIMAGE_NO_ZIG = "1";
    NULLSTRAP_EDITOR_COMPILER = "${stdenv.cc}/bin/cc";
  };

  postInstall = ''
    for library in uc i18n data; do
      if [ -e $out/lib/nullstrap/libicu$library.so.72.1.0.3 ]; then
        ln -s libicu$library.so.72.1.0.3 $out/lib/nullstrap/libicu$library.so.72
      fi
    done
    install -Dm644 build/Packaging/Linux/nullstrap.desktop \
      $out/share/applications/io.github.hxpedz.Nullstrap.desktop
    substituteInPlace $out/share/applications/io.github.hxpedz.Nullstrap.desktop \
      --replace-fail "Exec=nullstrap" "Exec=$out/bin/nullstrap"
    install -Dm644 src/Nullstrap.App/Nullstrap.png \
      $out/share/icons/hicolor/256x256/apps/io.github.hxpedz.Nullstrap.png
    install -Dm644 build/Packaging/Linux/io.github.hxpedz.Nullstrap.metainfo.xml \
      $out/share/metainfo/io.github.hxpedz.Nullstrap.metainfo.xml
  '';

  postFixup = ''
    ln -s Nullstrap $out/bin/nullstrap
  '';

  passthru = {
    nullimage = rustPlatform.buildRustPackage {
      pname = "nullimage";
      inherit (finalAttrs) version src;
      cargoRoot = "native/nullimage";
      buildAndTestSubdir = "native/nullimage";
      cargoHash = "sha256-25ly3BBUjnx7nzhCbZi0GkHVuWYEtj+PG1Y1EefzZqY=";
      doCheck = false;
    };
    updateScript = nix-update-script { };
  };

  meta = {
    description = "Roblox bootstrapper that launches Roblox through Sober on Linux";
    homepage = "https://github.com/hxpedz/Nullstrap";
    changelog = "https://github.com/hxpedz/Nullstrap/releases/tag/v${finalAttrs.version}";
    license = lib.licenses.mit;
    mainProgram = "nullstrap";
    platforms = [
      "x86_64-linux"
      "aarch64-linux"
    ];
  };
})
