/*
   ┏━━━┓╱╱╱╱╱┏┓╱╱┏┓╱╱╱╱╱┏┓╱╱
   ┃┏━┓┃╱╱╱╱╱┃┃╱╱┃┃╱╱╱╱╱┃┃╱╱╱
   ┃┗━┛┣┓┏┳━━┫┃┏┓┃┃╱╱┏━━┫┗━┳━━┓
   ┃┏━━┫┃┃┃┏┓┃┗┛┃┃┃╱┏┫┏┓┃┏┓┃━━┫
   ┃┃╱╱┃┗┛┃┃┃┃┏┓┃┃┗━┛┃┏┓┃┗┛┣━━┃
   ┗┛╱╱┗━━┻┛┗┻┛┗┛┗━━━┻┛┗┻━━┻━━┛
    ━━━━━━━━━━━━━━━━━━━━━━━━━━

    Copyright (c) 2024 Punk Labs LLC

    This section is part of OneTrick KEYS

    OneTrick KEYS is free software: you can redistribute it and/or modify it
    under the terms of the GNU General Public License as published by the Free
    Software Foundation, either version 3 of the License, or (at your option)
    any later version.

    OneTrick KEYS is distributed in the hope that it will be useful, but WITHOUT
    ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
    FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for
    more details.

    You should have received a copy of the GNU General Public License along with
    OneTrick KEYS.  If not, see <http://www.gnu.org/licenses/>.
*/

declare name "OneTrick KEYS DSP";
declare copyright "Copyright (c) 2023 Punk Labs LLC";
declare license "GPLv3 (or later)";

import("stdfaust.lib");
import("onetrick.lib");
import("params.lib");


addBodyReverb = result
    with {
        t60 = it.interpolate_linear(bodySize, 0.3, 0.6);
		result = ot.reverbZitaStereo(0, t60, bodyDamp) : ot.stereoSwap;
    };

addGlobalReverb = result with {
	t60 = it.interpolate_linear(reverbSize, 0.3, 4.5);
	rev = ot.reverbZitaStereo(20, t60, reverbDamp) : ot.stereo(fi.highpass(1, reverbCutoff));
	result = si.bus(2) <: si.bus(4) : (si.bus(2), (si.bus(2) : rev : ot.stereo(*(reverbAmount)) )) :> si.bus(2);
};

addMixSaturation = ot.saturationUrchin(mixSaturation * 0.75); // Tone it down a bit...
samplerHighpassAdjusted = ot.interpolate_freq(ot.input.modWheel : ot.smoothParam, samplerHighpass, 880);
samplerLowpassAdjusted = ot.interpolate_freq(ot.input.modWheel : ot.smoothParam, samplerLowpass, 1760);
addSamplerFilters = fi.highpass(2, samplerHighpassAdjusted) : fi.lowpass(4, samplerLowpassAdjusted);
mediaNoise = ot.interpolate_linear_stereo(mediaNoiseType, ot.stereoVinylNoise, ot.stereoTapeNoise) : ot.stereo(*(mediaNoiseAmount));
addMediaNoise = ot.addStereoNoiseGated(mediaNoise, -20+5, -80, ot.input.hold);

addMediaFlutter = ot.flutterStereo(mediaSpeed/60, mediaShape, mediaFlutter);
addMonoEffect = ot.monoizeBlend(mediaMono) : ot.stereo(*(1+mediaMono*0.25)); // Compensate some for loss of volume

addSamplerBitcrusher = ot.bitcrusher(samplerBits, samplerSamplerate);

// lowEQ = fi.lowshelf(1, 4, 300);
// midEQ = fi.peak_eq(-8, 590, 200);
// highEQ = fi.highshelf(3, 4, 10000);
// addEQ = lowEQ : midEQ : highEQ;
addEQ = _;

// process(left, right) = (left, right) : addBodyReverb : ot.stereo(addEQ) : addGlobalReverb : addMediaFlutter : addMediaNoise : ot.stereo(addSamplerFilters : addMixSaturation : addSamplerBitcrusher : *(mixGain)) : ot.stereoPanner(mixPan);
process(mono) = mono <: addBodyReverb : ot.stereo(addEQ) : addGlobalReverb : addMediaFlutter : addMediaNoise : ot.stereo(addSamplerFilters : addMixSaturation : addSamplerBitcrusher : *(mixGain)) : ot.stereoPanner(mixPan) : addMonoEffect;

// Test for recreating filters
// process(mono) = mono <: si.bus(2);

