┏━━━┓╱╱╱╱╱┏┓╱╱┏┓╱╱╱╱╱┏┓╱╱   
┃┏━┓┃╱╱╱╱╱┃┃╱╱┃┃╱╱╱╱╱┃┃╱╱╱  
┃┗━┛┣┓┏┳━━┫┃┏┓┃┃╱╱┏━━┫┗━┳━━┓
┃┏━━┫┃┃┃┏┓┃┗┛┃┃┃╱┏┫┏┓┃┏┓┃━━┫
┃┃╱╱┃┗┛┃┃┃┃┏┓┃┃┗━┛┃┏┓┃┗┛┣━━┃
┗┛╱╱┗━━┻┛┗┻┛┗┛┗━━━┻┛┗┻━━┻━━┛
━━━━━━━━━━━━━━━━━━━━━━━━━━ 

Copyright (c) 2024 Punk Labs LLC

License: GPL-3.0-or-later
Website: https://punklabs.com
Contact: contact@punklabs.com

Description:
	OneTrick KEYS is an open-source cross-platform piano synthesizer audio plugin.
	It is free as in rights, not as in beer.
	Please consider purchasing a copy to support our development efforts.
	
Build Instructions:
	Building the DSP (optional):
		Note: Using Faust 2.72.14
		1. Get a copy of the Faust compiler
		    https://faust.grame.fr/
		2. Run the following commands:
			# Build Output
			faust --check-table 0 --timeout 0 --process-name process -lang rust --architecture-dir onetrick/src/faust --import-dir onetrick/src/dsp -a arch.rs -o src/dsp/generated/dsp_output.rs --class-name DSP_Output -dlt 65536 src/dsp/output.dsp
            # Build Piano
            faust --check-table 0 --timeout 0 --process-name process -lang rust --architecture-dir onetrick/src/faust --import-dir onetrick/src/dsp -a arch.rs -o src/dsp/generated/dsp_piano.rs --class-name DSP_Piano -dlt 65536 src/dsp/piano.dsp
        3. Manually fix an issue with Faust 2.72.14 output:
           Faust 2.72.14 generated code has issues compiling on Windows
           To fix this find any instance of:
               #[link(name = "m")]
           And replace it with:
               #[cfg_attr(not(target_os="windows"), link(name="m"))]
        NOTE: This shouldn't be an issue with other versions of Faust.

	Building the Plugin:
		Note: Using rustc 1.79.0
		1. Install Rust
		    https://www.rust-lang.org/tools/install
		2. Build the plugin
			Debug Example:
				cargo xtask bundle onetrick_keys
			Release Example:
				cargo xtask bundle onetrick_keys --release
	Building the Audio Unit:
		Note: Using vst3sdk 3.7.11
		1. Download the Steinberg VST 3 SDK
		    https://steinbergmedia.github.io/vst3_dev_portal/pages/Getting+Started/Links.html#getting-vst-3-sdk
		2. Download the Apple CoreAudio SDK
		    https://developer.apple.com/library/archive/samplecode/CoreAudioUtilityClasses/Introduction/Intro.html
		3. Build auwrapper
		    https://steinbergmedia.github.io/vst3_dev_portal/pages/What+is+the+VST+3+SDK/Wrappers/AUv2+Wrapper.html
			Note: you can use the au-info.plist in OneTrick KEYS auwrapper folder.
