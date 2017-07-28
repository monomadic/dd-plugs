#!/bin/bash

cargo build --release
rm -rf ~/Library/Audio/Plug-Ins/VST/DDSampler.vst

DYLIB_FILE=libdd_sampler.dylib
VST_NAME=DDSampler
rm -rf ~/Library/Audio/Plug-Ins/VST/$VST_NAME.vst
vst-bundler $VST_NAME ../target/release/$DYLIB_FILE &&
mv -v ./$VST_NAME.vst ~/Library/Audio/Plug-Ins/VST/

du -sh ~/Library/Audio/Plug-Ins/VST/DDSampler.vst