# https://jake-shadle.github.io/xwin/

FROM docker.io/library/rust:1.87-slim-bookworm

ENV KEYRINGS=/usr/local/share/keyrings

# 1. Install clang/lld/llvm from the official LLVM apt repo
RUN set -eux; \
    mkdir -p $KEYRINGS; \
    apt-get update && apt-get install -y --no-install-recommends gpg curl ca-certificates; \
    curl --fail https://apt.llvm.org/llvm-snapshot.gpg.key \
        | gpg --dearmor > $KEYRINGS/llvm.gpg; \
    echo "deb [signed-by=$KEYRINGS/llvm.gpg] http://apt.llvm.org/bookworm/ llvm-toolchain-bookworm-19 main" \
        > /etc/apt/sources.list.d/llvm.list; \
    apt-get update && apt-get install --no-install-recommends -y \
        clang-19 \
        llvm-19 \
        lld-19 \
        tar; \
    ln -s clang-19    /usr/bin/clang; \
    ln -s clang       /usr/bin/clang++; \
    ln -s lld-19      /usr/bin/ld.lld; \
    ln -s clang-19    /usr/bin/clang-cl; \
    ln -s llvm-ar-19  /usr/bin/llvm-lib; \
    ln -s lld-link-19 /usr/bin/lld-link; \
    clang++ -v; \
    ld.lld -v; \
    llvm-lib -v; \
    clang-cl -v; \
    lld-link --version; \
    update-alternatives --install /usr/bin/cc  cc  /usr/bin/clang   100; \
    update-alternatives --install /usr/bin/c++ c++ /usr/bin/clang++ 100; \
    apt-get remove -y --auto-remove gpg; \
    rm -rf /var/lib/apt/lists/*

# 2. Add the Windows MSVC Rust std lib
RUN rustup target add x86_64-pc-windows-msvc

# 3. Download the MSVCRT + Windows SDK via xwin
RUN set -eux; \
    xwin_version="0.6.7"; \
    xwin_prefix="xwin-${xwin_version}-x86_64-unknown-linux-musl"; \
    curl --fail -L \
        "https://github.com/Jake-Shadle/xwin/releases/download/${xwin_version}/${xwin_prefix}.tar.gz" \
        | tar -xzv -C /usr/local/cargo/bin --strip-components=1 "${xwin_prefix}/xwin"; \
    xwin --accept-license splat --output /xwin; \
    rm -rf .xwin-cache /usr/local/cargo/bin/xwin

# 4. Tell cc-rs and cargo how to compile + link for the MSVC target
ENV CC_x86_64_pc_windows_msvc="clang-cl" \
    CXX_x86_64_pc_windows_msvc="clang-cl" \
    AR_x86_64_pc_windows_msvc="llvm-lib" \
    CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER="lld-link" \
    CL_FLAGS="-Wno-unused-command-line-argument -fuse-ld=lld-link /vctoolsdir /xwin/crt /winsdkdir /xwin/sdk" \
    CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_RUSTFLAGS="-Lnative=/xwin/crt/lib/x86_64 -Lnative=/xwin/sdk/lib/um/x86_64 -Lnative=/xwin/sdk/lib/ucrt/x86_64"

ENV CFLAGS_x86_64_pc_windows_msvc="$CL_FLAGS" \
    CXXFLAGS_x86_64_pc_windows_msvc="$CL_FLAGS"

# 5. Build
# WORKDIR /app
# COPY . .

# RUN cargo build --release --target x86_64-pc-windows-msvc
# The .exe lands at: target/x86_64-pc-windows-msvc/release/<your_crate>.exe