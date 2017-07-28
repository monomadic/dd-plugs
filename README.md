# dd-plugs
A set of minimalist VST plugins written in rust.

- **dd-subsynth** a polyphonic subtractive synth, good for sub basses at the moment.
- **dd-overdrive** a very simple overdrive / distortion.
- **dd-sampler** a simple sample playback plugin.

At the moment, these plugins are gui-less (though I'm building a vst gui lib to write gui based plugins which may end up here in future).

I was largely inspired by @ferrisstreamsstuff and his approach to self-built tools - he largely works for demoscene projects but I wish to create a series of basic workhorse plugins for myself.

Most plugins focus on flashy guis and silly unnecessary bloat, and coming with an army of presets so you don't have to do any thinking for yourself or understand how the plugin works. This suite is not for that kind of person. If you'd like to contribute your own plugins, please do so! Make a PR and lets learn DSP programming in rust together.

## Compiling
```shell
cargo build --release --all
```

On MacOS, you will need to package these up. A provided script will help you here.
```shell
chmod 777 ./contrib/mac-install.sh
./contrib/mac-install.sh
```

Alternatively I may release binary versions in the future.
