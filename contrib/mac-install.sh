#!/bin/bash

if [ ! -f "./contrib/vst-bundler.sh" ]; then
    echo "please run this script from the dd-plugs directory by typing ./contrib/mac-install.sh"
    exit 1
fi

cargo build --release --all

INSTALL_DIR="$HOME/Library/Audio/Plug-Ins/VST/"
plugins=$(find target/release/*.dylib -type f -exec basename {} \;)

for plugin in $plugins; do
    DYLIB_FILE="target/release/$plugin"
    TMP_VST_NAME=${plugin%.dylib}
    VST_NAME=${TMP_VST_NAME#lib}

    TARGET="$INSTALL_DIR$VST_NAME.vst"

    # remove the file if it exists in the target directory.
    [ -d "$TARGET" ] && rm -rf "$TARGET"

    bash ./contrib/vst-bundler.sh $VST_NAME $DYLIB_FILE &&
    mv -v ./$VST_NAME.vst $INSTALL_DIR
done 
