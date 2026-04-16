/* ------------------------------------------------------------
copyright: "Copyright (c) 2023 Punk Labs LLC"
license: "GPLv3 (or later)"
name: "OneTrick KEYS DSP"
Code generated with Faust 2.72.14 (https://faust.grame.fr)
Compilation options: -a arch.rs -lang rust -ct 0 -cn DSP_Output -dtl 65536 -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
------------------------------------------------------------ */
/*
   ┏━━━┓╱╱╱╱╱┏┓╱╱┏┓╱╱╱╱╱┏┓╱╱
   ┃┏━┓┃╱╱╱╱╱┃┃╱╱┃┃╱╱╱╱╱┃┃╱╱╱
   ┃┗━┛┣┓┏┳━━┫┃┏┓┃┃╱╱┏━━┫┗━┳━━┓
   ┃┏━━┫┃┃┃┏┓┃┗┛┃┃┃╱┏┫┏┓┃┏┓┃━━┫
   ┃┃╱╱┃┗┛┃┃┃┃┏┓┃┃┗━┛┃┏┓┃┗┛┣━━┃
   ┗┛╱╱┗━━┻┛┗┻┛┗┛┗━━━┻┛┗┻━━┻━━┛
    ━━━━━━━━━━━━━━━━━━━━━━━━━━

    Copyright (c) 2024 Punk Labs LLC

    This section is part of OneTrick

    OneTrick is free software: you can redistribute it and/or modify it
    under the terms of the GNU General Public License as published by the Free
    Software Foundation, either version 3 of the License, or (at your option)
    any later version.

    OneTrick is distributed in the hope that it will be useful, but WITHOUT
    ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
    FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for
    more details.

    You should have received a copy of the GNU General Public License along with
    OneTrick.  If not, see <http://www.gnu.org/licenses/>.
*/

use faust_types::*;




pub struct DSP_OutputSIG0 {
	iVec2: [i32;2],
	iRec18: [i32;2],
}

impl DSP_OutputSIG0 {
	
	fn get_num_inputsDSP_OutputSIG0(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsDSP_OutputSIG0(&self) -> i32 {
		return 1;
	}
	
	fn instance_initDSP_OutputSIG0(&mut self, sample_rate: i32) {
		for l5 in 0..2 {
			self.iVec2[l5 as usize] = 0;
		}
		for l6 in 0..2 {
			self.iRec18[l6 as usize] = 0;
		}
	}
	
	fn fillDSP_OutputSIG0(&mut self, count: i32, table: &mut[F32]) {
		for i1 in 0..count {
			self.iVec2[0] = 1;
			self.iRec18[0] = (i32::wrapping_add(self.iVec2[1], self.iRec18[1])) % 65536;
			table[i1 as usize] = F32::sin(9.58738e-05 * (self.iRec18[0]) as F32);
			self.iVec2[1] = self.iVec2[0];
			self.iRec18[1] = self.iRec18[0];
		}
	}

}


pub fn newDSP_OutputSIG0() -> DSP_OutputSIG0 { 
	DSP_OutputSIG0 {
		iVec2: [0;2],
		iRec18: [0;2],
	}
}

pub struct DSP_OutputSIG1 {
	iRec156: [i32;2],
}

impl DSP_OutputSIG1 {
	
	fn get_num_inputsDSP_OutputSIG1(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsDSP_OutputSIG1(&self) -> i32 {
		return 1;
	}
	
	fn instance_initDSP_OutputSIG1(&mut self, sample_rate: i32) {
		for l177 in 0..2 {
			self.iRec156[l177 as usize] = 0;
		}
	}
	
	fn fillDSP_OutputSIG1(&mut self, count: i32, table: &mut[F32]) {
		for i2 in 0..count {
			self.iRec156[0] = i32::wrapping_add(self.iRec156[1], 1);
			table[i2 as usize] = F32::sin(0.0012568884 * (i32::wrapping_add(self.iRec156[0], -1)) as F32);
			self.iRec156[1] = self.iRec156[0];
		}
	}

}


pub fn newDSP_OutputSIG1() -> DSP_OutputSIG1 { 
	DSP_OutputSIG1 {
		iRec156: [0;2],
	}
}
fn DSP_Output_faustpower2_f(value: F32) -> F32 {
	return value * value;
}
static mut ftbl0DSP_OutputSIG0: [F32;65536] = [0.0;65536];
static mut ftbl1DSP_OutputSIG1: [F32;5000] = [0.0;5000];
mod ffi {
	use std::os::raw::{c_float};
	#[cfg_attr(not(target_os="windows"), link(name="m"))]
	extern {
		pub fn remainderf(from: c_float, to: c_float) -> c_float;
		pub fn rintf(val: c_float) -> c_float;
	}
}
fn remainder_f32(from: f32, to: f32) -> f32 {
	unsafe { ffi::remainderf(from, to) }
}
fn rint_f32(val: f32) -> f32 {
	unsafe { ffi::rintf(val) }
}

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[repr(C)]
pub struct DSP_Output {
	iVec0: [i32;2],
	fButton0: F32,
	fVec1: [F32;2],
	fSampleRate: i32,
	fConst0: F32,
	fConst1: F32,
	fConst2: F32,
	fHslider0: F32,
	fRec0: [F32;2],
	fEntry0: F32,
	fRec1: [F32;2],
	fConst3: F32,
	fHslider1: F32,
	fRec4: [F32;2],
	fConst4: F32,
	fConst5: F32,
	fConst6: F32,
	fConst7: F32,
	fConst8: F32,
	fConst9: F32,
	fConst10: F32,
	fConst11: F32,
	fConst12: F32,
	fConst13: F32,
	fConst14: F32,
	fConst15: F32,
	fConst16: F32,
	fConst17: F32,
	fConst18: F32,
	fConst19: F32,
	fConst20: F32,
	fConst21: F32,
	fConst22: F32,
	fConst23: F32,
	fConst24: F32,
	fConst25: F32,
	fConst26: F32,
	fConst27: F32,
	fConst28: F32,
	fConst29: F32,
	fConst30: F32,
	fConst31: F32,
	fRec19: [F32;2],
	fConst32: F32,
	fRec20: [F32;2],
	fConst33: F32,
	fRec21: [F32;2],
	fConst34: F32,
	fConst35: F32,
	fConst36: F32,
	fConst37: F32,
	fConst38: F32,
	fConst39: F32,
	fConst40: F32,
	fConst41: F32,
	fConst42: F32,
	fConst43: F32,
	fConst44: F32,
	fConst45: F32,
	fConst46: F32,
	fConst47: F32,
	fConst48: F32,
	fConst49: F32,
	fConst50: F32,
	fConst51: F32,
	fConst52: F32,
	iConst53: i32,
	iRec26: [i32;2],
	fConst54: F32,
	fConst55: F32,
	iConst56: i32,
	iRec30: [i32;2],
	fVec3: [F32;2],
	fRec29: [F32;2],
	fRec27: [F32;2],
	fRec28: [F32;2],
	fRec25: [F32;2],
	fConst57: F32,
	fConst58: F32,
	fRec34: [F32;2],
	fVec4: [F32;2],
	fRec33: [F32;2],
	fConst59: F32,
	fRec32: [F32;2],
	fRec35: [F32;2],
	iVec5: [i32;2],
	iRec31: [i32;2],
	iRec39: [i32;2],
	fRec38: [F32;2],
	fRec36: [F32;2],
	fRec37: [F32;2],
	fRec24: [F32;3],
	fConst60: F32,
	fConst61: F32,
	fRec47: [F32;2],
	fVec6: [F32;2],
	fRec46: [F32;2],
	fRec45: [F32;2],
	fRec48: [F32;2],
	iVec7: [i32;2],
	iRec44: [i32;2],
	fRec43: [F32;3],
	fConst62: F32,
	fRec51: [F32;2],
	fRec49: [F32;2],
	fRec50: [F32;2],
	fVec8: [F32;2],
	fConst63: F32,
	fRec23: [F32;2],
	fConst64: F32,
	fRec52: [F32;2],
	fVec9: [F32;2],
	fConst65: F32,
	fConst66: F32,
	fConst67: F32,
	fRec53: [F32;2],
	fRec22: [F32;3],
	fRec17: [F32;3],
	fRec16: [F32;3],
	fRec15: [F32;3],
	fRec14: [F32;3],
	fRec13: [F32;3],
	fRec12: [F32;3],
	fRec11: [F32;3],
	fRec10: [F32;3],
	fRec9: [F32;3],
	fRec8: [F32;3],
	fRec7: [F32;3],
	fRec6: [F32;3],
	fConst68: F32,
	fConst69: F32,
	fConst70: F32,
	fConst71: F32,
	fConst72: F32,
	fConst73: F32,
	fConst74: F32,
	fConst75: F32,
	fConst76: F32,
	fConst77: F32,
	fConst78: F32,
	fConst79: F32,
	fConst80: F32,
	fConst81: F32,
	fConst82: F32,
	fConst83: F32,
	fConst84: F32,
	fConst85: F32,
	fConst86: F32,
	fConst87: F32,
	fConst88: F32,
	fConst89: F32,
	fConst90: F32,
	fConst91: F32,
	fConst92: F32,
	fConst93: F32,
	fConst94: F32,
	fConst95: F32,
	iRec70: [i32;2],
	fRec69: [F32;2],
	fRec67: [F32;2],
	fRec68: [F32;2],
	fRec66: [F32;2],
	fRec72: [F32;2],
	fRec65: [F32;3],
	fRec64: [F32;3],
	fRec63: [F32;3],
	fRec62: [F32;3],
	fRec61: [F32;3],
	fRec60: [F32;3],
	fRec59: [F32;3],
	fRec58: [F32;3],
	fRec57: [F32;3],
	fRec56: [F32;3],
	fRec55: [F32;3],
	fRec54: [F32;3],
	fConst96: F32,
	fHslider2: F32,
	fConst97: F32,
	fButton1: F32,
	fHslider3: F32,
	fConst98: F32,
	fHslider4: F32,
	fHslider5: F32,
	fConst99: F32,
	fConst100: F32,
	fConst101: F32,
	fConst102: F32,
	fConst103: F32,
	fConst104: F32,
	fRec87: [F32;2],
	fRec86: [F32;2],
	IOTA0: i32,
	fVec10: [F32;65536],
	fConst105: F32,
	iConst106: i32,
	fHslider6: F32,
	fHslider7: F32,
	fRec99: [F32;2],
	fRec98: [F32;2],
	fVec11: [F32;65536],
	fVec12: [F32;8192],
	iConst107: i32,
	fRec96: [F32;2],
	fConst108: F32,
	fConst109: F32,
	fRec103: [F32;2],
	fRec102: [F32;2],
	fVec13: [F32;32768],
	fConst110: F32,
	iConst111: i32,
	fVec14: [F32;4096],
	iConst112: i32,
	fRec100: [F32;2],
	fConst113: F32,
	fConst114: F32,
	fRec107: [F32;2],
	fRec106: [F32;2],
	fVec15: [F32;32768],
	fConst115: F32,
	iConst116: i32,
	fVec16: [F32;8192],
	iConst117: i32,
	fRec104: [F32;2],
	fConst118: F32,
	fConst119: F32,
	fRec111: [F32;2],
	fRec110: [F32;2],
	fVec17: [F32;32768],
	fConst120: F32,
	iConst121: i32,
	fVec18: [F32;4096],
	iConst122: i32,
	fRec108: [F32;2],
	fConst123: F32,
	fConst124: F32,
	fRec115: [F32;2],
	fRec114: [F32;2],
	fVec19: [F32;65536],
	fConst125: F32,
	iConst126: i32,
	fVec20: [F32;8192],
	iConst127: i32,
	fRec112: [F32;2],
	fConst128: F32,
	fConst129: F32,
	fRec119: [F32;2],
	fRec118: [F32;2],
	fVec21: [F32;65536],
	fConst130: F32,
	iConst131: i32,
	fVec22: [F32;8192],
	iConst132: i32,
	fRec116: [F32;2],
	fConst133: F32,
	fConst134: F32,
	fRec123: [F32;2],
	fRec122: [F32;2],
	fVec23: [F32;65536],
	fConst135: F32,
	iConst136: i32,
	fVec24: [F32;8192],
	iConst137: i32,
	fRec120: [F32;2],
	fConst138: F32,
	fConst139: F32,
	fRec127: [F32;2],
	fRec126: [F32;2],
	fVec25: [F32;65536],
	fConst140: F32,
	iConst141: i32,
	fVec26: [F32;4096],
	iConst142: i32,
	fRec124: [F32;2],
	fRec88: [F32;3],
	fRec89: [F32;3],
	fRec90: [F32;3],
	fRec91: [F32;3],
	fRec92: [F32;3],
	fRec93: [F32;3],
	fRec94: [F32;3],
	fRec95: [F32;3],
	fVec27: [F32;4096],
	iConst143: i32,
	fVec28: [F32;8192],
	fRec84: [F32;2],
	fRec131: [F32;2],
	fRec130: [F32;2],
	fVec29: [F32;32768],
	fVec30: [F32;4096],
	fRec128: [F32;2],
	fRec135: [F32;2],
	fRec134: [F32;2],
	fVec31: [F32;32768],
	fVec32: [F32;8192],
	fRec132: [F32;2],
	fRec139: [F32;2],
	fRec138: [F32;2],
	fVec33: [F32;32768],
	fVec34: [F32;4096],
	fRec136: [F32;2],
	fRec143: [F32;2],
	fRec142: [F32;2],
	fVec35: [F32;65536],
	fVec36: [F32;4096],
	fVec37: [F32;8192],
	fRec140: [F32;2],
	fRec147: [F32;2],
	fRec146: [F32;2],
	fVec38: [F32;65536],
	fVec39: [F32;8192],
	fRec144: [F32;2],
	fRec151: [F32;2],
	fRec150: [F32;2],
	fVec40: [F32;65536],
	fVec41: [F32;8192],
	fRec148: [F32;2],
	fRec155: [F32;2],
	fRec154: [F32;2],
	fVec42: [F32;65536],
	fVec43: [F32;4096],
	fRec152: [F32;2],
	fRec76: [F32;3],
	fRec77: [F32;3],
	fRec78: [F32;3],
	fRec79: [F32;3],
	fRec80: [F32;3],
	fRec81: [F32;3],
	fRec82: [F32;3],
	fRec83: [F32;3],
	fVec44: [F32;2],
	fRec75: [F32;2],
	fHslider8: F32,
	fVec45: [F32;32768],
	fConst144: F32,
	fHslider9: F32,
	fConst145: F32,
	fRec157: [F32;2],
	fHslider10: F32,
	fHslider11: F32,
	fRec158: [F32;2],
	fConst146: F32,
	fConst147: F32,
	fRec74: [F32;2],
	fConst148: F32,
	fRec73: [F32;2],
	fVec46: [F32;2],
	fRec161: [F32;2],
	fVec47: [F32;32768],
	fRec160: [F32;2],
	fRec159: [F32;2],
	fHslider12: F32,
	fRec5: [F32;3],
	fRec3: [F32;3],
	fRec2: [F32;3],
	fHslider13: F32,
	fHslider14: F32,
	fHslider15: F32,
	fRec164: [F32;2],
	fRec162: [F32;2],
	fRec163: [F32;2],
	fHslider16: F32,
	fRec165: [F32;2],
	fRec186: [F32;2],
	fRec184: [F32;2],
	fRec185: [F32;2],
	fRec183: [F32;3],
	fRec187: [F32;3],
	fRec190: [F32;2],
	fRec188: [F32;2],
	fRec189: [F32;2],
	fVec48: [F32;2],
	fRec182: [F32;2],
	fRec191: [F32;2],
	fVec49: [F32;2],
	fRec192: [F32;2],
	fRec181: [F32;3],
	fRec180: [F32;3],
	fRec179: [F32;3],
	fRec178: [F32;3],
	fRec177: [F32;3],
	fRec176: [F32;3],
	fRec175: [F32;3],
	fRec174: [F32;3],
	fRec173: [F32;3],
	fRec172: [F32;3],
	fRec171: [F32;3],
	fRec170: [F32;3],
	fRec169: [F32;3],
	fRec208: [F32;2],
	fRec206: [F32;2],
	fRec207: [F32;2],
	fRec205: [F32;2],
	fRec209: [F32;2],
	fRec204: [F32;3],
	fRec203: [F32;3],
	fRec202: [F32;3],
	fRec201: [F32;3],
	fRec200: [F32;3],
	fRec199: [F32;3],
	fRec198: [F32;3],
	fRec197: [F32;3],
	fRec196: [F32;3],
	fRec195: [F32;3],
	fRec194: [F32;3],
	fRec193: [F32;3],
	fRec168: [F32;3],
	fRec167: [F32;3],
	fRec166: [F32;3],
	fRec212: [F32;2],
	fRec210: [F32;2],
	fRec211: [F32;2],
	fHslider17: F32,
	fHslider18: F32,
	fRec213: [F32;2],
}

impl FaustDsp for DSP_Output {
	type T = F32;
		
	fn new() -> DSP_Output { 
		DSP_Output {
			iVec0: [0;2],
			fButton0: 0.0,
			fVec1: [0.0;2],
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			fConst2: 0.0,
			fHslider0: 0.0,
			fRec0: [0.0;2],
			fEntry0: 0.0,
			fRec1: [0.0;2],
			fConst3: 0.0,
			fHslider1: 0.0,
			fRec4: [0.0;2],
			fConst4: 0.0,
			fConst5: 0.0,
			fConst6: 0.0,
			fConst7: 0.0,
			fConst8: 0.0,
			fConst9: 0.0,
			fConst10: 0.0,
			fConst11: 0.0,
			fConst12: 0.0,
			fConst13: 0.0,
			fConst14: 0.0,
			fConst15: 0.0,
			fConst16: 0.0,
			fConst17: 0.0,
			fConst18: 0.0,
			fConst19: 0.0,
			fConst20: 0.0,
			fConst21: 0.0,
			fConst22: 0.0,
			fConst23: 0.0,
			fConst24: 0.0,
			fConst25: 0.0,
			fConst26: 0.0,
			fConst27: 0.0,
			fConst28: 0.0,
			fConst29: 0.0,
			fConst30: 0.0,
			fConst31: 0.0,
			fRec19: [0.0;2],
			fConst32: 0.0,
			fRec20: [0.0;2],
			fConst33: 0.0,
			fRec21: [0.0;2],
			fConst34: 0.0,
			fConst35: 0.0,
			fConst36: 0.0,
			fConst37: 0.0,
			fConst38: 0.0,
			fConst39: 0.0,
			fConst40: 0.0,
			fConst41: 0.0,
			fConst42: 0.0,
			fConst43: 0.0,
			fConst44: 0.0,
			fConst45: 0.0,
			fConst46: 0.0,
			fConst47: 0.0,
			fConst48: 0.0,
			fConst49: 0.0,
			fConst50: 0.0,
			fConst51: 0.0,
			fConst52: 0.0,
			iConst53: 0,
			iRec26: [0;2],
			fConst54: 0.0,
			fConst55: 0.0,
			iConst56: 0,
			iRec30: [0;2],
			fVec3: [0.0;2],
			fRec29: [0.0;2],
			fRec27: [0.0;2],
			fRec28: [0.0;2],
			fRec25: [0.0;2],
			fConst57: 0.0,
			fConst58: 0.0,
			fRec34: [0.0;2],
			fVec4: [0.0;2],
			fRec33: [0.0;2],
			fConst59: 0.0,
			fRec32: [0.0;2],
			fRec35: [0.0;2],
			iVec5: [0;2],
			iRec31: [0;2],
			iRec39: [0;2],
			fRec38: [0.0;2],
			fRec36: [0.0;2],
			fRec37: [0.0;2],
			fRec24: [0.0;3],
			fConst60: 0.0,
			fConst61: 0.0,
			fRec47: [0.0;2],
			fVec6: [0.0;2],
			fRec46: [0.0;2],
			fRec45: [0.0;2],
			fRec48: [0.0;2],
			iVec7: [0;2],
			iRec44: [0;2],
			fRec43: [0.0;3],
			fConst62: 0.0,
			fRec51: [0.0;2],
			fRec49: [0.0;2],
			fRec50: [0.0;2],
			fVec8: [0.0;2],
			fConst63: 0.0,
			fRec23: [0.0;2],
			fConst64: 0.0,
			fRec52: [0.0;2],
			fVec9: [0.0;2],
			fConst65: 0.0,
			fConst66: 0.0,
			fConst67: 0.0,
			fRec53: [0.0;2],
			fRec22: [0.0;3],
			fRec17: [0.0;3],
			fRec16: [0.0;3],
			fRec15: [0.0;3],
			fRec14: [0.0;3],
			fRec13: [0.0;3],
			fRec12: [0.0;3],
			fRec11: [0.0;3],
			fRec10: [0.0;3],
			fRec9: [0.0;3],
			fRec8: [0.0;3],
			fRec7: [0.0;3],
			fRec6: [0.0;3],
			fConst68: 0.0,
			fConst69: 0.0,
			fConst70: 0.0,
			fConst71: 0.0,
			fConst72: 0.0,
			fConst73: 0.0,
			fConst74: 0.0,
			fConst75: 0.0,
			fConst76: 0.0,
			fConst77: 0.0,
			fConst78: 0.0,
			fConst79: 0.0,
			fConst80: 0.0,
			fConst81: 0.0,
			fConst82: 0.0,
			fConst83: 0.0,
			fConst84: 0.0,
			fConst85: 0.0,
			fConst86: 0.0,
			fConst87: 0.0,
			fConst88: 0.0,
			fConst89: 0.0,
			fConst90: 0.0,
			fConst91: 0.0,
			fConst92: 0.0,
			fConst93: 0.0,
			fConst94: 0.0,
			fConst95: 0.0,
			iRec70: [0;2],
			fRec69: [0.0;2],
			fRec67: [0.0;2],
			fRec68: [0.0;2],
			fRec66: [0.0;2],
			fRec72: [0.0;2],
			fRec65: [0.0;3],
			fRec64: [0.0;3],
			fRec63: [0.0;3],
			fRec62: [0.0;3],
			fRec61: [0.0;3],
			fRec60: [0.0;3],
			fRec59: [0.0;3],
			fRec58: [0.0;3],
			fRec57: [0.0;3],
			fRec56: [0.0;3],
			fRec55: [0.0;3],
			fRec54: [0.0;3],
			fConst96: 0.0,
			fHslider2: 0.0,
			fConst97: 0.0,
			fButton1: 0.0,
			fHslider3: 0.0,
			fConst98: 0.0,
			fHslider4: 0.0,
			fHslider5: 0.0,
			fConst99: 0.0,
			fConst100: 0.0,
			fConst101: 0.0,
			fConst102: 0.0,
			fConst103: 0.0,
			fConst104: 0.0,
			fRec87: [0.0;2],
			fRec86: [0.0;2],
			IOTA0: 0,
			fVec10: [0.0;65536],
			fConst105: 0.0,
			iConst106: 0,
			fHslider6: 0.0,
			fHslider7: 0.0,
			fRec99: [0.0;2],
			fRec98: [0.0;2],
			fVec11: [0.0;65536],
			fVec12: [0.0;8192],
			iConst107: 0,
			fRec96: [0.0;2],
			fConst108: 0.0,
			fConst109: 0.0,
			fRec103: [0.0;2],
			fRec102: [0.0;2],
			fVec13: [0.0;32768],
			fConst110: 0.0,
			iConst111: 0,
			fVec14: [0.0;4096],
			iConst112: 0,
			fRec100: [0.0;2],
			fConst113: 0.0,
			fConst114: 0.0,
			fRec107: [0.0;2],
			fRec106: [0.0;2],
			fVec15: [0.0;32768],
			fConst115: 0.0,
			iConst116: 0,
			fVec16: [0.0;8192],
			iConst117: 0,
			fRec104: [0.0;2],
			fConst118: 0.0,
			fConst119: 0.0,
			fRec111: [0.0;2],
			fRec110: [0.0;2],
			fVec17: [0.0;32768],
			fConst120: 0.0,
			iConst121: 0,
			fVec18: [0.0;4096],
			iConst122: 0,
			fRec108: [0.0;2],
			fConst123: 0.0,
			fConst124: 0.0,
			fRec115: [0.0;2],
			fRec114: [0.0;2],
			fVec19: [0.0;65536],
			fConst125: 0.0,
			iConst126: 0,
			fVec20: [0.0;8192],
			iConst127: 0,
			fRec112: [0.0;2],
			fConst128: 0.0,
			fConst129: 0.0,
			fRec119: [0.0;2],
			fRec118: [0.0;2],
			fVec21: [0.0;65536],
			fConst130: 0.0,
			iConst131: 0,
			fVec22: [0.0;8192],
			iConst132: 0,
			fRec116: [0.0;2],
			fConst133: 0.0,
			fConst134: 0.0,
			fRec123: [0.0;2],
			fRec122: [0.0;2],
			fVec23: [0.0;65536],
			fConst135: 0.0,
			iConst136: 0,
			fVec24: [0.0;8192],
			iConst137: 0,
			fRec120: [0.0;2],
			fConst138: 0.0,
			fConst139: 0.0,
			fRec127: [0.0;2],
			fRec126: [0.0;2],
			fVec25: [0.0;65536],
			fConst140: 0.0,
			iConst141: 0,
			fVec26: [0.0;4096],
			iConst142: 0,
			fRec124: [0.0;2],
			fRec88: [0.0;3],
			fRec89: [0.0;3],
			fRec90: [0.0;3],
			fRec91: [0.0;3],
			fRec92: [0.0;3],
			fRec93: [0.0;3],
			fRec94: [0.0;3],
			fRec95: [0.0;3],
			fVec27: [0.0;4096],
			iConst143: 0,
			fVec28: [0.0;8192],
			fRec84: [0.0;2],
			fRec131: [0.0;2],
			fRec130: [0.0;2],
			fVec29: [0.0;32768],
			fVec30: [0.0;4096],
			fRec128: [0.0;2],
			fRec135: [0.0;2],
			fRec134: [0.0;2],
			fVec31: [0.0;32768],
			fVec32: [0.0;8192],
			fRec132: [0.0;2],
			fRec139: [0.0;2],
			fRec138: [0.0;2],
			fVec33: [0.0;32768],
			fVec34: [0.0;4096],
			fRec136: [0.0;2],
			fRec143: [0.0;2],
			fRec142: [0.0;2],
			fVec35: [0.0;65536],
			fVec36: [0.0;4096],
			fVec37: [0.0;8192],
			fRec140: [0.0;2],
			fRec147: [0.0;2],
			fRec146: [0.0;2],
			fVec38: [0.0;65536],
			fVec39: [0.0;8192],
			fRec144: [0.0;2],
			fRec151: [0.0;2],
			fRec150: [0.0;2],
			fVec40: [0.0;65536],
			fVec41: [0.0;8192],
			fRec148: [0.0;2],
			fRec155: [0.0;2],
			fRec154: [0.0;2],
			fVec42: [0.0;65536],
			fVec43: [0.0;4096],
			fRec152: [0.0;2],
			fRec76: [0.0;3],
			fRec77: [0.0;3],
			fRec78: [0.0;3],
			fRec79: [0.0;3],
			fRec80: [0.0;3],
			fRec81: [0.0;3],
			fRec82: [0.0;3],
			fRec83: [0.0;3],
			fVec44: [0.0;2],
			fRec75: [0.0;2],
			fHslider8: 0.0,
			fVec45: [0.0;32768],
			fConst144: 0.0,
			fHslider9: 0.0,
			fConst145: 0.0,
			fRec157: [0.0;2],
			fHslider10: 0.0,
			fHslider11: 0.0,
			fRec158: [0.0;2],
			fConst146: 0.0,
			fConst147: 0.0,
			fRec74: [0.0;2],
			fConst148: 0.0,
			fRec73: [0.0;2],
			fVec46: [0.0;2],
			fRec161: [0.0;2],
			fVec47: [0.0;32768],
			fRec160: [0.0;2],
			fRec159: [0.0;2],
			fHslider12: 0.0,
			fRec5: [0.0;3],
			fRec3: [0.0;3],
			fRec2: [0.0;3],
			fHslider13: 0.0,
			fHslider14: 0.0,
			fHslider15: 0.0,
			fRec164: [0.0;2],
			fRec162: [0.0;2],
			fRec163: [0.0;2],
			fHslider16: 0.0,
			fRec165: [0.0;2],
			fRec186: [0.0;2],
			fRec184: [0.0;2],
			fRec185: [0.0;2],
			fRec183: [0.0;3],
			fRec187: [0.0;3],
			fRec190: [0.0;2],
			fRec188: [0.0;2],
			fRec189: [0.0;2],
			fVec48: [0.0;2],
			fRec182: [0.0;2],
			fRec191: [0.0;2],
			fVec49: [0.0;2],
			fRec192: [0.0;2],
			fRec181: [0.0;3],
			fRec180: [0.0;3],
			fRec179: [0.0;3],
			fRec178: [0.0;3],
			fRec177: [0.0;3],
			fRec176: [0.0;3],
			fRec175: [0.0;3],
			fRec174: [0.0;3],
			fRec173: [0.0;3],
			fRec172: [0.0;3],
			fRec171: [0.0;3],
			fRec170: [0.0;3],
			fRec169: [0.0;3],
			fRec208: [0.0;2],
			fRec206: [0.0;2],
			fRec207: [0.0;2],
			fRec205: [0.0;2],
			fRec209: [0.0;2],
			fRec204: [0.0;3],
			fRec203: [0.0;3],
			fRec202: [0.0;3],
			fRec201: [0.0;3],
			fRec200: [0.0;3],
			fRec199: [0.0;3],
			fRec198: [0.0;3],
			fRec197: [0.0;3],
			fRec196: [0.0;3],
			fRec195: [0.0;3],
			fRec194: [0.0;3],
			fRec193: [0.0;3],
			fRec168: [0.0;3],
			fRec167: [0.0;3],
			fRec166: [0.0;3],
			fRec212: [0.0;2],
			fRec210: [0.0;2],
			fRec211: [0.0;2],
			fHslider17: 0.0,
			fHslider18: 0.0,
			fRec213: [0.0;2],
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("analyzers.lib/name", r"Faust Analyzer Library");
		m.declare("analyzers.lib/version", r"1.2.0");
		m.declare("basics.lib/bitcrusher:author", r"Julius O. Smith III, revised by Stephane Letz");
		m.declare("basics.lib/downSample:author", r"Romain Michon");
		m.declare("basics.lib/name", r"Faust Basic Element Library");
		m.declare("basics.lib/sAndH:author", r"Romain Michon");
		m.declare("basics.lib/tabulate:author", r"Stephane Letz");
		m.declare("basics.lib/tabulateNd", r"Copyright (C) 2023 Bart Brouns <bart@magnetophon.nl>");
		m.declare("basics.lib/version", r"1.15.0");
		m.declare("compile_options", r"-a arch.rs -lang rust -ct 0 -cn DSP_Output -dtl 65536 -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
		m.declare("copyright", r"Copyright (c) 2023 Punk Labs LLC");
		m.declare("delays.lib/name", r"Faust Delay Library");
		m.declare("delays.lib/version", r"1.1.0");
		m.declare("envelopes.lib/ar:author", r"Yann Orlarey, Stéphane Letz");
		m.declare("envelopes.lib/author", r"GRAME");
		m.declare("envelopes.lib/copyright", r"GRAME");
		m.declare("envelopes.lib/license", r"LGPL with exception");
		m.declare("envelopes.lib/name", r"Faust Envelope Library");
		m.declare("envelopes.lib/version", r"1.3.0");
		m.declare("filename", r"output.dsp");
		m.declare("filters.lib/allpass_comb:author", r"Julius O. Smith III");
		m.declare("filters.lib/allpass_comb:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/allpass_comb:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/bandpass0_bandstop1:author", r"Julius O. Smith III");
		m.declare("filters.lib/bandpass0_bandstop1:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/bandpass0_bandstop1:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/bandpass:author", r"Julius O. Smith III");
		m.declare("filters.lib/bandpass:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/bandpass:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/filterbank:author", r"Julius O. Smith III");
		m.declare("filters.lib/filterbank:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/filterbank:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/fir:author", r"Julius O. Smith III");
		m.declare("filters.lib/fir:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/fir:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/highpass:author", r"Julius O. Smith III");
		m.declare("filters.lib/highpass:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/iir:author", r"Julius O. Smith III");
		m.declare("filters.lib/iir:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/iir:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/lowpass0_highpass1", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/lowpass0_highpass1:author", r"Julius O. Smith III");
		m.declare("filters.lib/lowpass:author", r"Julius O. Smith III");
		m.declare("filters.lib/lowpass:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/lowpass:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/lowshelf:author", r"Julius O. Smith III");
		m.declare("filters.lib/lowshelf:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/lowshelf:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/name", r"Faust Filters Library");
		m.declare("filters.lib/peak_eq:author", r"Julius O. Smith III");
		m.declare("filters.lib/peak_eq:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/peak_eq:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/tf1:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf1:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf1:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/tf1s:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf1s:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf1s:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/tf1sb:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf1sb:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf1sb:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/tf2:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf2:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf2:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/tf2s:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf2s:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf2s:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/version", r"1.3.0");
		m.declare("interpolators.lib/interpolate_linear:author", r"Stéphane Letz");
		m.declare("interpolators.lib/interpolate_linear:licence", r"MIT");
		m.declare("interpolators.lib/name", r"Faust Interpolator Library");
		m.declare("interpolators.lib/remap:author", r"David Braun");
		m.declare("interpolators.lib/version", r"1.3.1");
		m.declare("license", r"GPLv3 (or later)");
		m.declare("maths.lib/author", r"GRAME");
		m.declare("maths.lib/copyright", r"GRAME");
		m.declare("maths.lib/license", r"LGPL with exception");
		m.declare("maths.lib/name", r"Faust Math Library");
		m.declare("maths.lib/version", r"2.8.0");
		m.declare("misceffects.lib/dryWetMixer:author", r"David Braun, revised by Stéphane Letz");
		m.declare("misceffects.lib/name", r"Misc Effects Library");
		m.declare("misceffects.lib/version", r"2.4.0");
		m.declare("name", r"OneTrick KEYS DSP");
		m.declare("noises.lib/name", r"Faust Noise Generator Library");
		m.declare("noises.lib/version", r"1.4.1");
		m.declare("onetrick.lib/copyright", r"Copyright (c) 2023 Punk Labs LLC");
		m.declare("onetrick.lib/license", r"GPLv3 (or later)");
		m.declare("onetrick.lib/name", r"OneTrick DSP Library");
		m.declare("oscillators.lib/lf_sawpos:author", r"Bart Brouns, revised by Stéphane Letz");
		m.declare("oscillators.lib/lf_sawpos:licence", r"STK-4.3");
		m.declare("oscillators.lib/name", r"Faust Oscillator Library");
		m.declare("oscillators.lib/version", r"1.5.1");
		m.declare("platform.lib/name", r"Generic Platform Library");
		m.declare("platform.lib/version", r"1.3.0");
		m.declare("reverbs.lib/name", r"Faust Reverb Library");
		m.declare("reverbs.lib/version", r"1.2.1");
		m.declare("routes.lib/hadamard:author", r"Remy Muller, revised by Romain Michon");
		m.declare("routes.lib/name", r"Faust Signal Routing Library");
		m.declare("routes.lib/version", r"1.2.0");
		m.declare("signals.lib/name", r"Faust Signal Routing Library");
		m.declare("signals.lib/version", r"1.5.0");
	}

	fn get_sample_rate(&self) -> i32 {
		return self.fSampleRate;
	}
	fn get_num_inputs(&self) -> i32 {
		return 1;
	}
	fn get_num_outputs(&self) -> i32 {
		return 2;
	}
	
	fn class_init(sample_rate: i32) {
		let mut sig0: DSP_OutputSIG0 = newDSP_OutputSIG0();
		sig0.instance_initDSP_OutputSIG0(sample_rate);
		sig0.fillDSP_OutputSIG0(65536, unsafe { &mut ftbl0DSP_OutputSIG0 });
		let mut sig1: DSP_OutputSIG1 = newDSP_OutputSIG1();
		sig1.instance_initDSP_OutputSIG1(sample_rate);
		sig1.fillDSP_OutputSIG1(5000, unsafe { &mut ftbl1DSP_OutputSIG1 });
	}
	fn instance_reset_params(&mut self) {
		self.fButton0 = 0.0;
		self.fHslider0 = 2e+01;
		self.fEntry0 = 0.0;
		self.fHslider1 = 2e+01;
		self.fHslider2 = 0.0;
		self.fButton1 = 0.0;
		self.fHslider3 = 2e+02;
		self.fHslider4 = 25.0;
		self.fHslider5 = 15.0;
		self.fHslider6 = 15.0;
		self.fHslider7 = 5e+01;
		self.fHslider8 = 5e+01;
		self.fHslider9 = 78.0;
		self.fHslider10 = 75.0;
		self.fHslider11 = 0.0;
		self.fHslider12 = 0.0;
		self.fHslider13 = 0.0;
		self.fHslider14 = 16.0;
		self.fHslider15 = 44.1;
		self.fHslider16 = 0.0;
		self.fHslider17 = 0.0;
		self.fHslider18 = 0.0;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.iVec0[l0 as usize] = 0;
		}
		for l1 in 0..2 {
			self.fVec1[l1 as usize] = 0.0;
		}
		for l2 in 0..2 {
			self.fRec0[l2 as usize] = 0.0;
		}
		for l3 in 0..2 {
			self.fRec1[l3 as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.fRec4[l4 as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec19[l7 as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fRec20[l8 as usize] = 0.0;
		}
		for l9 in 0..2 {
			self.fRec21[l9 as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.iRec26[l10 as usize] = 0;
		}
		for l11 in 0..2 {
			self.iRec30[l11 as usize] = 0;
		}
		for l12 in 0..2 {
			self.fVec3[l12 as usize] = 0.0;
		}
		for l13 in 0..2 {
			self.fRec29[l13 as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fRec27[l14 as usize] = 0.0;
		}
		for l15 in 0..2 {
			self.fRec28[l15 as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.fRec25[l16 as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.fRec34[l17 as usize] = 0.0;
		}
		for l18 in 0..2 {
			self.fVec4[l18 as usize] = 0.0;
		}
		for l19 in 0..2 {
			self.fRec33[l19 as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fRec32[l20 as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec35[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.iVec5[l22 as usize] = 0;
		}
		for l23 in 0..2 {
			self.iRec31[l23 as usize] = 0;
		}
		for l24 in 0..2 {
			self.iRec39[l24 as usize] = 0;
		}
		for l25 in 0..2 {
			self.fRec38[l25 as usize] = 0.0;
		}
		for l26 in 0..2 {
			self.fRec36[l26 as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fRec37[l27 as usize] = 0.0;
		}
		for l28 in 0..3 {
			self.fRec24[l28 as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fRec47[l29 as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fVec6[l30 as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fRec46[l31 as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec45[l32 as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fRec48[l33 as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.iVec7[l34 as usize] = 0;
		}
		for l35 in 0..2 {
			self.iRec44[l35 as usize] = 0;
		}
		for l36 in 0..3 {
			self.fRec43[l36 as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fRec51[l37 as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fRec49[l38 as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fRec50[l39 as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fVec8[l40 as usize] = 0.0;
		}
		for l41 in 0..2 {
			self.fRec23[l41 as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.fRec52[l42 as usize] = 0.0;
		}
		for l43 in 0..2 {
			self.fVec9[l43 as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fRec53[l44 as usize] = 0.0;
		}
		for l45 in 0..3 {
			self.fRec22[l45 as usize] = 0.0;
		}
		for l46 in 0..3 {
			self.fRec17[l46 as usize] = 0.0;
		}
		for l47 in 0..3 {
			self.fRec16[l47 as usize] = 0.0;
		}
		for l48 in 0..3 {
			self.fRec15[l48 as usize] = 0.0;
		}
		for l49 in 0..3 {
			self.fRec14[l49 as usize] = 0.0;
		}
		for l50 in 0..3 {
			self.fRec13[l50 as usize] = 0.0;
		}
		for l51 in 0..3 {
			self.fRec12[l51 as usize] = 0.0;
		}
		for l52 in 0..3 {
			self.fRec11[l52 as usize] = 0.0;
		}
		for l53 in 0..3 {
			self.fRec10[l53 as usize] = 0.0;
		}
		for l54 in 0..3 {
			self.fRec9[l54 as usize] = 0.0;
		}
		for l55 in 0..3 {
			self.fRec8[l55 as usize] = 0.0;
		}
		for l56 in 0..3 {
			self.fRec7[l56 as usize] = 0.0;
		}
		for l57 in 0..3 {
			self.fRec6[l57 as usize] = 0.0;
		}
		for l58 in 0..2 {
			self.iRec70[l58 as usize] = 0;
		}
		for l59 in 0..2 {
			self.fRec69[l59 as usize] = 0.0;
		}
		for l60 in 0..2 {
			self.fRec67[l60 as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.fRec68[l61 as usize] = 0.0;
		}
		for l62 in 0..2 {
			self.fRec66[l62 as usize] = 0.0;
		}
		for l63 in 0..2 {
			self.fRec72[l63 as usize] = 0.0;
		}
		for l64 in 0..3 {
			self.fRec65[l64 as usize] = 0.0;
		}
		for l65 in 0..3 {
			self.fRec64[l65 as usize] = 0.0;
		}
		for l66 in 0..3 {
			self.fRec63[l66 as usize] = 0.0;
		}
		for l67 in 0..3 {
			self.fRec62[l67 as usize] = 0.0;
		}
		for l68 in 0..3 {
			self.fRec61[l68 as usize] = 0.0;
		}
		for l69 in 0..3 {
			self.fRec60[l69 as usize] = 0.0;
		}
		for l70 in 0..3 {
			self.fRec59[l70 as usize] = 0.0;
		}
		for l71 in 0..3 {
			self.fRec58[l71 as usize] = 0.0;
		}
		for l72 in 0..3 {
			self.fRec57[l72 as usize] = 0.0;
		}
		for l73 in 0..3 {
			self.fRec56[l73 as usize] = 0.0;
		}
		for l74 in 0..3 {
			self.fRec55[l74 as usize] = 0.0;
		}
		for l75 in 0..3 {
			self.fRec54[l75 as usize] = 0.0;
		}
		for l76 in 0..2 {
			self.fRec87[l76 as usize] = 0.0;
		}
		for l77 in 0..2 {
			self.fRec86[l77 as usize] = 0.0;
		}
		self.IOTA0 = 0;
		for l78 in 0..65536 {
			self.fVec10[l78 as usize] = 0.0;
		}
		for l79 in 0..2 {
			self.fRec99[l79 as usize] = 0.0;
		}
		for l80 in 0..2 {
			self.fRec98[l80 as usize] = 0.0;
		}
		for l81 in 0..65536 {
			self.fVec11[l81 as usize] = 0.0;
		}
		for l82 in 0..8192 {
			self.fVec12[l82 as usize] = 0.0;
		}
		for l83 in 0..2 {
			self.fRec96[l83 as usize] = 0.0;
		}
		for l84 in 0..2 {
			self.fRec103[l84 as usize] = 0.0;
		}
		for l85 in 0..2 {
			self.fRec102[l85 as usize] = 0.0;
		}
		for l86 in 0..32768 {
			self.fVec13[l86 as usize] = 0.0;
		}
		for l87 in 0..4096 {
			self.fVec14[l87 as usize] = 0.0;
		}
		for l88 in 0..2 {
			self.fRec100[l88 as usize] = 0.0;
		}
		for l89 in 0..2 {
			self.fRec107[l89 as usize] = 0.0;
		}
		for l90 in 0..2 {
			self.fRec106[l90 as usize] = 0.0;
		}
		for l91 in 0..32768 {
			self.fVec15[l91 as usize] = 0.0;
		}
		for l92 in 0..8192 {
			self.fVec16[l92 as usize] = 0.0;
		}
		for l93 in 0..2 {
			self.fRec104[l93 as usize] = 0.0;
		}
		for l94 in 0..2 {
			self.fRec111[l94 as usize] = 0.0;
		}
		for l95 in 0..2 {
			self.fRec110[l95 as usize] = 0.0;
		}
		for l96 in 0..32768 {
			self.fVec17[l96 as usize] = 0.0;
		}
		for l97 in 0..4096 {
			self.fVec18[l97 as usize] = 0.0;
		}
		for l98 in 0..2 {
			self.fRec108[l98 as usize] = 0.0;
		}
		for l99 in 0..2 {
			self.fRec115[l99 as usize] = 0.0;
		}
		for l100 in 0..2 {
			self.fRec114[l100 as usize] = 0.0;
		}
		for l101 in 0..65536 {
			self.fVec19[l101 as usize] = 0.0;
		}
		for l102 in 0..8192 {
			self.fVec20[l102 as usize] = 0.0;
		}
		for l103 in 0..2 {
			self.fRec112[l103 as usize] = 0.0;
		}
		for l104 in 0..2 {
			self.fRec119[l104 as usize] = 0.0;
		}
		for l105 in 0..2 {
			self.fRec118[l105 as usize] = 0.0;
		}
		for l106 in 0..65536 {
			self.fVec21[l106 as usize] = 0.0;
		}
		for l107 in 0..8192 {
			self.fVec22[l107 as usize] = 0.0;
		}
		for l108 in 0..2 {
			self.fRec116[l108 as usize] = 0.0;
		}
		for l109 in 0..2 {
			self.fRec123[l109 as usize] = 0.0;
		}
		for l110 in 0..2 {
			self.fRec122[l110 as usize] = 0.0;
		}
		for l111 in 0..65536 {
			self.fVec23[l111 as usize] = 0.0;
		}
		for l112 in 0..8192 {
			self.fVec24[l112 as usize] = 0.0;
		}
		for l113 in 0..2 {
			self.fRec120[l113 as usize] = 0.0;
		}
		for l114 in 0..2 {
			self.fRec127[l114 as usize] = 0.0;
		}
		for l115 in 0..2 {
			self.fRec126[l115 as usize] = 0.0;
		}
		for l116 in 0..65536 {
			self.fVec25[l116 as usize] = 0.0;
		}
		for l117 in 0..4096 {
			self.fVec26[l117 as usize] = 0.0;
		}
		for l118 in 0..2 {
			self.fRec124[l118 as usize] = 0.0;
		}
		for l119 in 0..3 {
			self.fRec88[l119 as usize] = 0.0;
		}
		for l120 in 0..3 {
			self.fRec89[l120 as usize] = 0.0;
		}
		for l121 in 0..3 {
			self.fRec90[l121 as usize] = 0.0;
		}
		for l122 in 0..3 {
			self.fRec91[l122 as usize] = 0.0;
		}
		for l123 in 0..3 {
			self.fRec92[l123 as usize] = 0.0;
		}
		for l124 in 0..3 {
			self.fRec93[l124 as usize] = 0.0;
		}
		for l125 in 0..3 {
			self.fRec94[l125 as usize] = 0.0;
		}
		for l126 in 0..3 {
			self.fRec95[l126 as usize] = 0.0;
		}
		for l127 in 0..4096 {
			self.fVec27[l127 as usize] = 0.0;
		}
		for l128 in 0..8192 {
			self.fVec28[l128 as usize] = 0.0;
		}
		for l129 in 0..2 {
			self.fRec84[l129 as usize] = 0.0;
		}
		for l130 in 0..2 {
			self.fRec131[l130 as usize] = 0.0;
		}
		for l131 in 0..2 {
			self.fRec130[l131 as usize] = 0.0;
		}
		for l132 in 0..32768 {
			self.fVec29[l132 as usize] = 0.0;
		}
		for l133 in 0..4096 {
			self.fVec30[l133 as usize] = 0.0;
		}
		for l134 in 0..2 {
			self.fRec128[l134 as usize] = 0.0;
		}
		for l135 in 0..2 {
			self.fRec135[l135 as usize] = 0.0;
		}
		for l136 in 0..2 {
			self.fRec134[l136 as usize] = 0.0;
		}
		for l137 in 0..32768 {
			self.fVec31[l137 as usize] = 0.0;
		}
		for l138 in 0..8192 {
			self.fVec32[l138 as usize] = 0.0;
		}
		for l139 in 0..2 {
			self.fRec132[l139 as usize] = 0.0;
		}
		for l140 in 0..2 {
			self.fRec139[l140 as usize] = 0.0;
		}
		for l141 in 0..2 {
			self.fRec138[l141 as usize] = 0.0;
		}
		for l142 in 0..32768 {
			self.fVec33[l142 as usize] = 0.0;
		}
		for l143 in 0..4096 {
			self.fVec34[l143 as usize] = 0.0;
		}
		for l144 in 0..2 {
			self.fRec136[l144 as usize] = 0.0;
		}
		for l145 in 0..2 {
			self.fRec143[l145 as usize] = 0.0;
		}
		for l146 in 0..2 {
			self.fRec142[l146 as usize] = 0.0;
		}
		for l147 in 0..65536 {
			self.fVec35[l147 as usize] = 0.0;
		}
		for l148 in 0..4096 {
			self.fVec36[l148 as usize] = 0.0;
		}
		for l149 in 0..8192 {
			self.fVec37[l149 as usize] = 0.0;
		}
		for l150 in 0..2 {
			self.fRec140[l150 as usize] = 0.0;
		}
		for l151 in 0..2 {
			self.fRec147[l151 as usize] = 0.0;
		}
		for l152 in 0..2 {
			self.fRec146[l152 as usize] = 0.0;
		}
		for l153 in 0..65536 {
			self.fVec38[l153 as usize] = 0.0;
		}
		for l154 in 0..8192 {
			self.fVec39[l154 as usize] = 0.0;
		}
		for l155 in 0..2 {
			self.fRec144[l155 as usize] = 0.0;
		}
		for l156 in 0..2 {
			self.fRec151[l156 as usize] = 0.0;
		}
		for l157 in 0..2 {
			self.fRec150[l157 as usize] = 0.0;
		}
		for l158 in 0..65536 {
			self.fVec40[l158 as usize] = 0.0;
		}
		for l159 in 0..8192 {
			self.fVec41[l159 as usize] = 0.0;
		}
		for l160 in 0..2 {
			self.fRec148[l160 as usize] = 0.0;
		}
		for l161 in 0..2 {
			self.fRec155[l161 as usize] = 0.0;
		}
		for l162 in 0..2 {
			self.fRec154[l162 as usize] = 0.0;
		}
		for l163 in 0..65536 {
			self.fVec42[l163 as usize] = 0.0;
		}
		for l164 in 0..4096 {
			self.fVec43[l164 as usize] = 0.0;
		}
		for l165 in 0..2 {
			self.fRec152[l165 as usize] = 0.0;
		}
		for l166 in 0..3 {
			self.fRec76[l166 as usize] = 0.0;
		}
		for l167 in 0..3 {
			self.fRec77[l167 as usize] = 0.0;
		}
		for l168 in 0..3 {
			self.fRec78[l168 as usize] = 0.0;
		}
		for l169 in 0..3 {
			self.fRec79[l169 as usize] = 0.0;
		}
		for l170 in 0..3 {
			self.fRec80[l170 as usize] = 0.0;
		}
		for l171 in 0..3 {
			self.fRec81[l171 as usize] = 0.0;
		}
		for l172 in 0..3 {
			self.fRec82[l172 as usize] = 0.0;
		}
		for l173 in 0..3 {
			self.fRec83[l173 as usize] = 0.0;
		}
		for l174 in 0..2 {
			self.fVec44[l174 as usize] = 0.0;
		}
		for l175 in 0..2 {
			self.fRec75[l175 as usize] = 0.0;
		}
		for l176 in 0..32768 {
			self.fVec45[l176 as usize] = 0.0;
		}
		for l178 in 0..2 {
			self.fRec157[l178 as usize] = 0.0;
		}
		for l179 in 0..2 {
			self.fRec158[l179 as usize] = 0.0;
		}
		for l180 in 0..2 {
			self.fRec74[l180 as usize] = 0.0;
		}
		for l181 in 0..2 {
			self.fRec73[l181 as usize] = 0.0;
		}
		for l182 in 0..2 {
			self.fVec46[l182 as usize] = 0.0;
		}
		for l183 in 0..2 {
			self.fRec161[l183 as usize] = 0.0;
		}
		for l184 in 0..32768 {
			self.fVec47[l184 as usize] = 0.0;
		}
		for l185 in 0..2 {
			self.fRec160[l185 as usize] = 0.0;
		}
		for l186 in 0..2 {
			self.fRec159[l186 as usize] = 0.0;
		}
		for l187 in 0..3 {
			self.fRec5[l187 as usize] = 0.0;
		}
		for l188 in 0..3 {
			self.fRec3[l188 as usize] = 0.0;
		}
		for l189 in 0..3 {
			self.fRec2[l189 as usize] = 0.0;
		}
		for l190 in 0..2 {
			self.fRec164[l190 as usize] = 0.0;
		}
		for l191 in 0..2 {
			self.fRec162[l191 as usize] = 0.0;
		}
		for l192 in 0..2 {
			self.fRec163[l192 as usize] = 0.0;
		}
		for l193 in 0..2 {
			self.fRec165[l193 as usize] = 0.0;
		}
		for l194 in 0..2 {
			self.fRec186[l194 as usize] = 0.0;
		}
		for l195 in 0..2 {
			self.fRec184[l195 as usize] = 0.0;
		}
		for l196 in 0..2 {
			self.fRec185[l196 as usize] = 0.0;
		}
		for l197 in 0..3 {
			self.fRec183[l197 as usize] = 0.0;
		}
		for l198 in 0..3 {
			self.fRec187[l198 as usize] = 0.0;
		}
		for l199 in 0..2 {
			self.fRec190[l199 as usize] = 0.0;
		}
		for l200 in 0..2 {
			self.fRec188[l200 as usize] = 0.0;
		}
		for l201 in 0..2 {
			self.fRec189[l201 as usize] = 0.0;
		}
		for l202 in 0..2 {
			self.fVec48[l202 as usize] = 0.0;
		}
		for l203 in 0..2 {
			self.fRec182[l203 as usize] = 0.0;
		}
		for l204 in 0..2 {
			self.fRec191[l204 as usize] = 0.0;
		}
		for l205 in 0..2 {
			self.fVec49[l205 as usize] = 0.0;
		}
		for l206 in 0..2 {
			self.fRec192[l206 as usize] = 0.0;
		}
		for l207 in 0..3 {
			self.fRec181[l207 as usize] = 0.0;
		}
		for l208 in 0..3 {
			self.fRec180[l208 as usize] = 0.0;
		}
		for l209 in 0..3 {
			self.fRec179[l209 as usize] = 0.0;
		}
		for l210 in 0..3 {
			self.fRec178[l210 as usize] = 0.0;
		}
		for l211 in 0..3 {
			self.fRec177[l211 as usize] = 0.0;
		}
		for l212 in 0..3 {
			self.fRec176[l212 as usize] = 0.0;
		}
		for l213 in 0..3 {
			self.fRec175[l213 as usize] = 0.0;
		}
		for l214 in 0..3 {
			self.fRec174[l214 as usize] = 0.0;
		}
		for l215 in 0..3 {
			self.fRec173[l215 as usize] = 0.0;
		}
		for l216 in 0..3 {
			self.fRec172[l216 as usize] = 0.0;
		}
		for l217 in 0..3 {
			self.fRec171[l217 as usize] = 0.0;
		}
		for l218 in 0..3 {
			self.fRec170[l218 as usize] = 0.0;
		}
		for l219 in 0..3 {
			self.fRec169[l219 as usize] = 0.0;
		}
		for l220 in 0..2 {
			self.fRec208[l220 as usize] = 0.0;
		}
		for l221 in 0..2 {
			self.fRec206[l221 as usize] = 0.0;
		}
		for l222 in 0..2 {
			self.fRec207[l222 as usize] = 0.0;
		}
		for l223 in 0..2 {
			self.fRec205[l223 as usize] = 0.0;
		}
		for l224 in 0..2 {
			self.fRec209[l224 as usize] = 0.0;
		}
		for l225 in 0..3 {
			self.fRec204[l225 as usize] = 0.0;
		}
		for l226 in 0..3 {
			self.fRec203[l226 as usize] = 0.0;
		}
		for l227 in 0..3 {
			self.fRec202[l227 as usize] = 0.0;
		}
		for l228 in 0..3 {
			self.fRec201[l228 as usize] = 0.0;
		}
		for l229 in 0..3 {
			self.fRec200[l229 as usize] = 0.0;
		}
		for l230 in 0..3 {
			self.fRec199[l230 as usize] = 0.0;
		}
		for l231 in 0..3 {
			self.fRec198[l231 as usize] = 0.0;
		}
		for l232 in 0..3 {
			self.fRec197[l232 as usize] = 0.0;
		}
		for l233 in 0..3 {
			self.fRec196[l233 as usize] = 0.0;
		}
		for l234 in 0..3 {
			self.fRec195[l234 as usize] = 0.0;
		}
		for l235 in 0..3 {
			self.fRec194[l235 as usize] = 0.0;
		}
		for l236 in 0..3 {
			self.fRec193[l236 as usize] = 0.0;
		}
		for l237 in 0..3 {
			self.fRec168[l237 as usize] = 0.0;
		}
		for l238 in 0..3 {
			self.fRec167[l238 as usize] = 0.0;
		}
		for l239 in 0..3 {
			self.fRec166[l239 as usize] = 0.0;
		}
		for l240 in 0..2 {
			self.fRec212[l240 as usize] = 0.0;
		}
		for l241 in 0..2 {
			self.fRec210[l241 as usize] = 0.0;
		}
		for l242 in 0..2 {
			self.fRec211[l242 as usize] = 0.0;
		}
		for l243 in 0..2 {
			self.fRec213[l243 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		self.fConst1 = F32::exp(-(1e+02 / self.fConst0));
		self.fConst2 = 0.475 * self.fConst0;
		self.fConst3 = 1382.3008 / self.fConst0;
		self.fConst4 = F32::tan(3.1415927 * (F32::min(self.fConst2, 8e+03) / self.fConst0));
		self.fConst5 = 2.0 * (1.0 - 1.0 / DSP_Output_faustpower2_f(self.fConst4));
		self.fConst6 = 1.0 / self.fConst4;
		self.fConst7 = (self.fConst6 + -0.13080625) / self.fConst4 + 1.0;
		self.fConst8 = 1.0 / ((self.fConst6 + 0.13080625) / self.fConst4 + 1.0);
		self.fConst9 = (self.fConst6 + -0.39018065) / self.fConst4 + 1.0;
		self.fConst10 = 1.0 / ((self.fConst6 + 0.39018065) / self.fConst4 + 1.0);
		self.fConst11 = (self.fConst6 + -0.64287895) / self.fConst4 + 1.0;
		self.fConst12 = 1.0 / ((self.fConst6 + 0.64287895) / self.fConst4 + 1.0);
		self.fConst13 = (self.fConst6 + -0.8845774) / self.fConst4 + 1.0;
		self.fConst14 = 1.0 / ((self.fConst6 + 0.8845774) / self.fConst4 + 1.0);
		self.fConst15 = (self.fConst6 + -1.1111405) / self.fConst4 + 1.0;
		self.fConst16 = 1.0 / ((self.fConst6 + 1.1111405) / self.fConst4 + 1.0);
		self.fConst17 = (self.fConst6 + -1.3186916) / self.fConst4 + 1.0;
		self.fConst18 = 1.0 / ((self.fConst6 + 1.3186916) / self.fConst4 + 1.0);
		self.fConst19 = (self.fConst6 + -1.5036796) / self.fConst4 + 1.0;
		self.fConst20 = 1.0 / ((self.fConst6 + 1.5036796) / self.fConst4 + 1.0);
		self.fConst21 = (self.fConst6 + -1.6629392) / self.fConst4 + 1.0;
		self.fConst22 = 1.0 / ((self.fConst6 + 1.6629392) / self.fConst4 + 1.0);
		self.fConst23 = (self.fConst6 + -1.7937455) / self.fConst4 + 1.0;
		self.fConst24 = 1.0 / ((self.fConst6 + 1.7937455) / self.fConst4 + 1.0);
		self.fConst25 = (self.fConst6 + -1.8938602) / self.fConst4 + 1.0;
		self.fConst26 = 1.0 / ((self.fConst6 + 1.8938602) / self.fConst4 + 1.0);
		self.fConst27 = (self.fConst6 + -1.9615705) / self.fConst4 + 1.0;
		self.fConst28 = 1.0 / ((self.fConst6 + 1.9615705) / self.fConst4 + 1.0);
		self.fConst29 = (self.fConst6 + -1.9957179) / self.fConst4 + 1.0;
		self.fConst30 = 1.0 / ((self.fConst6 + 1.9957179) / self.fConst4 + 1.0);
		self.fConst31 = 5.2 / self.fConst0;
		self.fConst32 = 2.6 / self.fConst0;
		self.fConst33 = 1.3 / self.fConst0;
		self.fConst34 = 10995.574 / (self.fConst0 * F32::sin(9424.778 / self.fConst0));
		self.fConst35 = F32::tan(4712.389 / self.fConst0);
		self.fConst36 = 1.0 / self.fConst35;
		self.fConst37 = 2.0 * (1.0 - 1.0 / DSP_Output_faustpower2_f(self.fConst35));
		self.fConst38 = F32::tan(314.15927 / self.fConst0);
		self.fConst39 = 1.0 / self.fConst38;
		self.fConst40 = 1.0 - self.fConst39;
		self.fConst41 = F32::tan(12566.371 / self.fConst0);
		self.fConst42 = self.fConst0 * self.fConst41;
		self.fConst43 = F32::tan(1570.7964 / self.fConst0);
		self.fConst44 = DSP_Output_faustpower2_f(F32::sqrt(4.0 * DSP_Output_faustpower2_f(self.fConst0) * self.fConst43 * self.fConst41));
		self.fConst45 = 2.0 * self.fConst42 - 0.5 * (self.fConst44 / self.fConst42);
		self.fConst46 = 2.0 * (self.fConst45 / self.fConst0);
		self.fConst47 = DSP_Output_faustpower2_f(1.0 / self.fConst0) * self.fConst44;
		self.fConst48 = self.fConst47 + (4.0 - self.fConst46);
		self.fConst49 = 2.0 * self.fConst47 + -8.0;
		self.fConst50 = self.fConst47 + self.fConst46 + 4.0;
		self.fConst51 = 1.0 / self.fConst50;
		self.fConst52 = 0.1 * self.fConst0;
		self.iConst53 = (self.fConst52) as i32;
		self.fConst54 = F32::min(4.8e+04, self.fConst0);
		self.fConst55 = self.fConst0 / F32::min(self.fConst54, self.fConst0);
		self.iConst56 = (self.fConst55) as i32;
		self.fConst57 = 0.00025 * self.fConst0;
		self.fConst58 = 1e+01 / self.fConst0;
		self.fConst59 = F32::round(self.fConst0 / self.fConst54);
		self.fConst60 = 0.001 * self.fConst0;
		self.fConst61 = 1.5 / self.fConst0;
		self.fConst62 = self.fConst45 / (self.fConst0 * self.fConst50);
		self.fConst63 = 1.0 / (self.fConst39 + 1.0);
		self.fConst64 = 0.0891251 / self.fConst38;
		self.fConst65 = 1.0 / self.fConst43;
		self.fConst66 = 1.0 - self.fConst65;
		self.fConst67 = 1.0 / (self.fConst65 + 1.0);
		self.fConst68 = F32::tan(3.1415927 * (F32::min(self.fConst2, 1.65e+04) / self.fConst0));
		self.fConst69 = 2.0 * (1.0 - 1.0 / DSP_Output_faustpower2_f(self.fConst68));
		self.fConst70 = 1.0 / self.fConst68;
		self.fConst71 = (self.fConst70 + -0.13080625) / self.fConst68 + 1.0;
		self.fConst72 = (self.fConst70 + 0.13080625) / self.fConst68 + 1.0;
		self.fConst73 = 1.0 / self.fConst72;
		self.fConst74 = (self.fConst70 + -0.39018065) / self.fConst68 + 1.0;
		self.fConst75 = 1.0 / ((self.fConst70 + 0.39018065) / self.fConst68 + 1.0);
		self.fConst76 = (self.fConst70 + -0.64287895) / self.fConst68 + 1.0;
		self.fConst77 = 1.0 / ((self.fConst70 + 0.64287895) / self.fConst68 + 1.0);
		self.fConst78 = (self.fConst70 + -0.8845774) / self.fConst68 + 1.0;
		self.fConst79 = 1.0 / ((self.fConst70 + 0.8845774) / self.fConst68 + 1.0);
		self.fConst80 = (self.fConst70 + -1.1111405) / self.fConst68 + 1.0;
		self.fConst81 = 1.0 / ((self.fConst70 + 1.1111405) / self.fConst68 + 1.0);
		self.fConst82 = (self.fConst70 + -1.3186916) / self.fConst68 + 1.0;
		self.fConst83 = 1.0 / ((self.fConst70 + 1.3186916) / self.fConst68 + 1.0);
		self.fConst84 = (self.fConst70 + -1.5036796) / self.fConst68 + 1.0;
		self.fConst85 = 1.0 / ((self.fConst70 + 1.5036796) / self.fConst68 + 1.0);
		self.fConst86 = (self.fConst70 + -1.6629392) / self.fConst68 + 1.0;
		self.fConst87 = 1.0 / ((self.fConst70 + 1.6629392) / self.fConst68 + 1.0);
		self.fConst88 = (self.fConst70 + -1.7937455) / self.fConst68 + 1.0;
		self.fConst89 = 1.0 / ((self.fConst70 + 1.7937455) / self.fConst68 + 1.0);
		self.fConst90 = (self.fConst70 + -1.8938602) / self.fConst68 + 1.0;
		self.fConst91 = 1.0 / ((self.fConst70 + 1.8938602) / self.fConst68 + 1.0);
		self.fConst92 = (self.fConst70 + -1.9615705) / self.fConst68 + 1.0;
		self.fConst93 = 1.0 / ((self.fConst70 + 1.9615705) / self.fConst68 + 1.0);
		self.fConst94 = (self.fConst70 + -1.9957179) / self.fConst68 + 1.0;
		self.fConst95 = 1.0 / ((self.fConst70 + 1.9957179) / self.fConst68 + 1.0);
		self.fConst96 = 0.06309573 / self.fConst72;
		self.fConst97 = F32::exp(-(1e+04 / self.fConst0));
		self.fConst98 = 3.1415927 / self.fConst0;
		self.fConst99 = F32::floor(0.174713 * self.fConst0 + 0.5);
		self.fConst100 = 6.9077554 * (self.fConst99 / self.fConst0);
		self.fConst101 = F32::cos(37699.113 / self.fConst0);
		self.fConst102 = 1.0 / F32::tan(628.31854 / self.fConst0);
		self.fConst103 = 1.0 - self.fConst102;
		self.fConst104 = 1.0 / (self.fConst102 + 1.0);
		self.fConst105 = F32::floor(0.022904 * self.fConst0 + 0.5);
		self.iConst106 = (F32::min(32768.0, F32::max(0.0, self.fConst99 - self.fConst105))) as i32;
		self.iConst107 = (F32::min(8192.0, F32::max(0.0, self.fConst105 + -1.0))) as i32;
		self.fConst108 = F32::floor(0.153129 * self.fConst0 + 0.5);
		self.fConst109 = 6.9077554 * (self.fConst108 / self.fConst0);
		self.fConst110 = F32::floor(0.020346 * self.fConst0 + 0.5);
		self.iConst111 = (F32::min(32768.0, F32::max(0.0, self.fConst108 - self.fConst110))) as i32;
		self.iConst112 = (F32::min(4096.0, F32::max(0.0, self.fConst110 + -1.0))) as i32;
		self.fConst113 = F32::floor(0.127837 * self.fConst0 + 0.5);
		self.fConst114 = 6.9077554 * (self.fConst113 / self.fConst0);
		self.fConst115 = F32::floor(0.031604 * self.fConst0 + 0.5);
		self.iConst116 = (F32::min(32768.0, F32::max(0.0, self.fConst113 - self.fConst115))) as i32;
		self.iConst117 = (F32::min(8192.0, F32::max(0.0, self.fConst115 + -1.0))) as i32;
		self.fConst118 = F32::floor(0.125 * self.fConst0 + 0.5);
		self.fConst119 = 6.9077554 * (self.fConst118 / self.fConst0);
		self.fConst120 = F32::floor(0.013458 * self.fConst0 + 0.5);
		self.iConst121 = (F32::min(32768.0, F32::max(0.0, self.fConst118 - self.fConst120))) as i32;
		self.iConst122 = (F32::min(4096.0, F32::max(0.0, self.fConst120 + -1.0))) as i32;
		self.fConst123 = F32::floor(0.210389 * self.fConst0 + 0.5);
		self.fConst124 = 6.9077554 * (self.fConst123 / self.fConst0);
		self.fConst125 = F32::floor(0.024421 * self.fConst0 + 0.5);
		self.iConst126 = (F32::min(65536.0, F32::max(0.0, self.fConst123 - self.fConst125))) as i32;
		self.iConst127 = (F32::min(8192.0, F32::max(0.0, self.fConst125 + -1.0))) as i32;
		self.fConst128 = F32::floor(0.192303 * self.fConst0 + 0.5);
		self.fConst129 = 6.9077554 * (self.fConst128 / self.fConst0);
		self.fConst130 = F32::floor(0.029291 * self.fConst0 + 0.5);
		self.iConst131 = (F32::min(32768.0, F32::max(0.0, self.fConst128 - self.fConst130))) as i32;
		self.iConst132 = (F32::min(8192.0, F32::max(0.0, self.fConst130 + -1.0))) as i32;
		self.fConst133 = F32::floor(0.256891 * self.fConst0 + 0.5);
		self.fConst134 = 6.9077554 * (self.fConst133 / self.fConst0);
		self.fConst135 = F32::floor(0.027333 * self.fConst0 + 0.5);
		self.iConst136 = (F32::min(65536.0, F32::max(0.0, self.fConst133 - self.fConst135))) as i32;
		self.iConst137 = (F32::min(8192.0, F32::max(0.0, self.fConst135 + -1.0))) as i32;
		self.fConst138 = F32::floor(0.219991 * self.fConst0 + 0.5);
		self.fConst139 = 6.9077554 * (self.fConst138 / self.fConst0);
		self.fConst140 = F32::floor(0.019123 * self.fConst0 + 0.5);
		self.iConst141 = (F32::min(65536.0, F32::max(0.0, self.fConst138 - self.fConst140))) as i32;
		self.iConst142 = (F32::min(4096.0, F32::max(0.0, self.fConst140 + -1.0))) as i32;
		self.iConst143 = (F32::min(8192.0, F32::max(0.0, 0.02 * self.fConst0))) as i32;
		self.fConst144 = self.fConst52 + 1.0;
		self.fConst145 = 0.016666668 / self.fConst0;
		self.fConst146 = F32::exp(-(33.333332 / self.fConst0));
		self.fConst147 = 1.0 - self.fConst146;
		self.fConst148 = 1.0 - self.fConst97;
	}
	fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	fn init(&mut self, sample_rate: i32) {
		DSP_Output::class_init(sample_rate);
		self.instance_init(sample_rate);
	}
	
	fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
		Self::build_user_interface_static(ui_interface);
	}
	
	fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) {
		ui_interface.open_vertical_box("OneTrick KEYS DSP");
		ui_interface.add_button("Hold", ParamIndex(0));
		ui_interface.add_num_entry("ModWheel", ParamIndex(1), 0.0, 0.0, 1.0, 0.001);
		ui_interface.add_button("WakeUp", ParamIndex(2));
		ui_interface.declare(Some(ParamIndex(3)), "110", "");
		ui_interface.declare(Some(ParamIndex(3)), "export", "Gain");
		ui_interface.declare(Some(ParamIndex(3)), "group", "Mix");
		ui_interface.declare(Some(ParamIndex(3)), "unit", "dB");
		ui_interface.add_horizontal_slider("Mix_Gain", ParamIndex(3), 0.0, -1e+02, 6.0, 0.1);
		ui_interface.declare(Some(ParamIndex(4)), "120", "");
		ui_interface.declare(Some(ParamIndex(4)), "export", "Pan");
		ui_interface.declare(Some(ParamIndex(4)), "group", "Mix");
		ui_interface.declare(Some(ParamIndex(4)), "unit", "%");
		ui_interface.add_horizontal_slider("Mix_Pan", ParamIndex(4), 0.0, -1e+02, 1e+02, 0.01);
		ui_interface.declare(Some(ParamIndex(5)), "130", "");
		ui_interface.declare(Some(ParamIndex(5)), "export", "Saturation");
		ui_interface.declare(Some(ParamIndex(5)), "group", "Mix");
		ui_interface.declare(Some(ParamIndex(5)), "unit", "%");
		ui_interface.add_horizontal_slider("Mix_Saturation", ParamIndex(5), 0.0, 0.0, 1e+02, 0.01);
		ui_interface.declare(Some(ParamIndex(6)), "210", "");
		ui_interface.declare(Some(ParamIndex(6)), "export", "Size");
		ui_interface.declare(Some(ParamIndex(6)), "group", "Body");
		ui_interface.declare(Some(ParamIndex(6)), "unit", "%");
		ui_interface.add_horizontal_slider("Body_Size", ParamIndex(6), 5e+01, 0.0, 1e+02, 0.01);
		ui_interface.declare(Some(ParamIndex(7)), "220", "");
		ui_interface.declare(Some(ParamIndex(7)), "export", "Damp");
		ui_interface.declare(Some(ParamIndex(7)), "group", "Body");
		ui_interface.declare(Some(ParamIndex(7)), "unit", "%");
		ui_interface.add_horizontal_slider("Body_Damp", ParamIndex(7), 15.0, 0.0, 1e+02, 0.01);
		ui_interface.declare(Some(ParamIndex(8)), "310", "");
		ui_interface.declare(Some(ParamIndex(8)), "export", "Amount");
		ui_interface.declare(Some(ParamIndex(8)), "group", "Reverb");
		ui_interface.declare(Some(ParamIndex(8)), "unit", "%");
		ui_interface.add_horizontal_slider("Reverb_Amount", ParamIndex(8), 5e+01, 0.0, 1e+02, 0.01);
		ui_interface.declare(Some(ParamIndex(9)), "320", "");
		ui_interface.declare(Some(ParamIndex(9)), "export", "Size");
		ui_interface.declare(Some(ParamIndex(9)), "group", "Reverb");
		ui_interface.declare(Some(ParamIndex(9)), "unit", "%");
		ui_interface.add_horizontal_slider("Reverb_Size", ParamIndex(9), 15.0, 0.0, 1e+02, 0.01);
		ui_interface.declare(Some(ParamIndex(10)), "330", "");
		ui_interface.declare(Some(ParamIndex(10)), "export", "Damp");
		ui_interface.declare(Some(ParamIndex(10)), "group", "Reverb");
		ui_interface.declare(Some(ParamIndex(10)), "unit", "%");
		ui_interface.add_horizontal_slider("Reverb_Damp", ParamIndex(10), 25.0, 0.0, 1e+02, 0.01);
		ui_interface.declare(Some(ParamIndex(11)), "340", "");
		ui_interface.declare(Some(ParamIndex(11)), "export", "Highpass");
		ui_interface.declare(Some(ParamIndex(11)), "group", "Reverb");
		ui_interface.declare(Some(ParamIndex(11)), "unit", "Hz");
		ui_interface.add_horizontal_slider("Reverb_Highpass", ParamIndex(11), 2e+02, 1e+02, 1e+03, 0.01);
		ui_interface.declare(Some(ParamIndex(12)), "410", "");
		ui_interface.declare(Some(ParamIndex(12)), "export", "Lowpass");
		ui_interface.declare(Some(ParamIndex(12)), "group", "Sampler");
		ui_interface.declare(Some(ParamIndex(12)), "unit", "kHz");
		ui_interface.add_horizontal_slider("Sampler_Lowpass", ParamIndex(12), 2e+01, 1.0, 2e+01, 0.01);
		ui_interface.declare(Some(ParamIndex(13)), "420", "");
		ui_interface.declare(Some(ParamIndex(13)), "export", "Highpass");
		ui_interface.declare(Some(ParamIndex(13)), "group", "Sampler");
		ui_interface.declare(Some(ParamIndex(13)), "unit", "Hz");
		ui_interface.add_horizontal_slider("Sampler_Highpass", ParamIndex(13), 2e+01, 2e+01, 2e+02, 0.01);
		ui_interface.declare(Some(ParamIndex(14)), "430", "");
		ui_interface.declare(Some(ParamIndex(14)), "export", "Samplerate");
		ui_interface.declare(Some(ParamIndex(14)), "group", "Sampler");
		ui_interface.declare(Some(ParamIndex(14)), "unit", "kHz");
		ui_interface.add_horizontal_slider("Sampler_Samplerate", ParamIndex(14), 44.1, 22.05, 44.1, 0.01);
		ui_interface.declare(Some(ParamIndex(15)), "440", "");
		ui_interface.declare(Some(ParamIndex(15)), "export", "Bits");
		ui_interface.declare(Some(ParamIndex(15)), "group", "Sampler");
		ui_interface.declare(Some(ParamIndex(15)), "unit", "bit");
		ui_interface.add_horizontal_slider("Sampler_Bits", ParamIndex(15), 16.0, 8.0, 16.0, 1.0);
		ui_interface.declare(Some(ParamIndex(16)), "510", "");
		ui_interface.declare(Some(ParamIndex(16)), "enum", "Vinyl,Tape");
		ui_interface.declare(Some(ParamIndex(16)), "export", "Noise Type");
		ui_interface.declare(Some(ParamIndex(16)), "group", "Media");
		ui_interface.add_horizontal_slider("Media_Noise_Type", ParamIndex(16), 0.0, 0.0, 1.0, 1.0);
		ui_interface.declare(Some(ParamIndex(17)), "520", "");
		ui_interface.declare(Some(ParamIndex(17)), "export", "Noise Amount");
		ui_interface.declare(Some(ParamIndex(17)), "group", "Media");
		ui_interface.declare(Some(ParamIndex(17)), "unit", "%");
		ui_interface.add_horizontal_slider("Media_Noise_Amount", ParamIndex(17), 0.0, 0.0, 1e+02, 0.01);
		ui_interface.declare(Some(ParamIndex(18)), "530", "");
		ui_interface.declare(Some(ParamIndex(18)), "export", "Flutter");
		ui_interface.declare(Some(ParamIndex(18)), "group", "Media");
		ui_interface.declare(Some(ParamIndex(18)), "unit", "%");
		ui_interface.add_horizontal_slider("Media_Flutter", ParamIndex(18), 0.0, 0.0, 1e+02, 0.01);
		ui_interface.declare(Some(ParamIndex(19)), "540", "");
		ui_interface.declare(Some(ParamIndex(19)), "export", "Speed");
		ui_interface.declare(Some(ParamIndex(19)), "group", "Media");
		ui_interface.declare(Some(ParamIndex(19)), "unit", "rpm");
		ui_interface.add_horizontal_slider("Media_Speed", ParamIndex(19), 78.0, 33.0, 5e+02, 0.01);
		ui_interface.declare(Some(ParamIndex(20)), "550", "");
		ui_interface.declare(Some(ParamIndex(20)), "export", "Shape");
		ui_interface.declare(Some(ParamIndex(20)), "group", "Media");
		ui_interface.declare(Some(ParamIndex(20)), "unit", "%");
		ui_interface.add_horizontal_slider("Media_Shape", ParamIndex(20), 75.0, 1.0, 1e+02, 0.01);
		ui_interface.declare(Some(ParamIndex(21)), "560", "");
		ui_interface.declare(Some(ParamIndex(21)), "export", "Mono");
		ui_interface.declare(Some(ParamIndex(21)), "group", "Media");
		ui_interface.declare(Some(ParamIndex(21)), "unit", "%");
		ui_interface.add_horizontal_slider("Media_Mono", ParamIndex(21), 0.0, 0.0, 1e+02, 0.01);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			2 => Some(self.fButton0),
			0 => Some(self.fButton1),
			1 => Some(self.fEntry0),
			12 => Some(self.fHslider0),
			13 => Some(self.fHslider1),
			20 => Some(self.fHslider10),
			18 => Some(self.fHslider11),
			17 => Some(self.fHslider12),
			5 => Some(self.fHslider13),
			15 => Some(self.fHslider14),
			14 => Some(self.fHslider15),
			4 => Some(self.fHslider16),
			21 => Some(self.fHslider17),
			3 => Some(self.fHslider18),
			16 => Some(self.fHslider2),
			11 => Some(self.fHslider3),
			10 => Some(self.fHslider4),
			9 => Some(self.fHslider5),
			7 => Some(self.fHslider6),
			6 => Some(self.fHslider7),
			8 => Some(self.fHslider8),
			19 => Some(self.fHslider9),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			2 => { self.fButton0 = value }
			0 => { self.fButton1 = value }
			1 => { self.fEntry0 = value }
			12 => { self.fHslider0 = value }
			13 => { self.fHslider1 = value }
			20 => { self.fHslider10 = value }
			18 => { self.fHslider11 = value }
			17 => { self.fHslider12 = value }
			5 => { self.fHslider13 = value }
			15 => { self.fHslider14 = value }
			14 => { self.fHslider15 = value }
			4 => { self.fHslider16 = value }
			21 => { self.fHslider17 = value }
			3 => { self.fHslider18 = value }
			16 => { self.fHslider2 = value }
			11 => { self.fHslider3 = value }
			10 => { self.fHslider4 = value }
			9 => { self.fHslider5 = value }
			7 => { self.fHslider6 = value }
			6 => { self.fHslider7 = value }
			8 => { self.fHslider8 = value }
			19 => { self.fHslider9 = value }
			_ => {}
		}
	}
	
	fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut[&mut[Self::T]]) {
		let (inputs0) = if let [inputs0, ..] = inputs {
			let inputs0 = inputs0[..count as usize].iter();
			(inputs0)
		} else {
			panic!("wrong number of inputs");
		};
		let (outputs0, outputs1) = if let [outputs0, outputs1, ..] = outputs {
			let outputs0 = outputs0[..count as usize].iter_mut();
			let outputs1 = outputs1[..count as usize].iter_mut();
			(outputs0, outputs1)
		} else {
			panic!("wrong number of outputs");
		};
		let mut fSlow0: F32 = self.fButton0;
		let mut fSlow1: F32 = F32::min(self.fConst2, 1e+03 * self.fHslider0);
		let mut fSlow2: F32 = self.fEntry0;
		let mut fSlow3: F32 = self.fHslider1;
		let mut fSlow4: F32 = self.fHslider2;
		let mut iSlow5: i32 = (self.fButton1) as i32;
		let mut fSlow6: F32 = F32::tan(self.fConst98 * self.fHslider3);
		let mut fSlow7: F32 = 1.0 / fSlow6;
		let mut fSlow8: F32 = 1.0 - fSlow7;
		let mut fSlow9: F32 = 0.042 * self.fHslider5 + 0.3;
		let mut fSlow10: F32 = fSlow9 * (1.0 - 0.00855 * self.fHslider4);
		let mut fSlow11: F32 = F32::exp(-(self.fConst100 / fSlow10));
		let mut fSlow12: F32 = DSP_Output_faustpower2_f(fSlow11);
		let mut fSlow13: F32 = 1.0 - fSlow12;
		let mut fSlow14: F32 = 1.0 - self.fConst101 * fSlow12;
		let mut fSlow15: F32 = F32::sqrt(F32::max(0.0, DSP_Output_faustpower2_f(fSlow14) / DSP_Output_faustpower2_f(fSlow13) + -1.0));
		let mut fSlow16: F32 = fSlow14 / fSlow13;
		let mut fSlow17: F32 = fSlow16 - fSlow15;
		let mut fSlow18: F32 = F32::exp(-(self.fConst100 / fSlow9)) / fSlow11 + -1.0;
		let mut fSlow19: F32 = fSlow11 * (fSlow15 + (1.0 - fSlow16));
		let mut fSlow20: F32 = 0.003 * self.fHslider7 + 0.3;
		let mut fSlow21: F32 = fSlow20 * (1.0 - 0.00855 * self.fHslider6);
		let mut fSlow22: F32 = F32::exp(-(self.fConst100 / fSlow21));
		let mut fSlow23: F32 = DSP_Output_faustpower2_f(fSlow22);
		let mut fSlow24: F32 = 1.0 - fSlow23;
		let mut fSlow25: F32 = 1.0 - self.fConst101 * fSlow23;
		let mut fSlow26: F32 = F32::sqrt(F32::max(0.0, DSP_Output_faustpower2_f(fSlow25) / DSP_Output_faustpower2_f(fSlow24) + -1.0));
		let mut fSlow27: F32 = fSlow25 / fSlow24;
		let mut fSlow28: F32 = fSlow27 - fSlow26;
		let mut fSlow29: F32 = F32::exp(-(self.fConst100 / fSlow20)) / fSlow22 + -1.0;
		let mut fSlow30: F32 = fSlow22 * (fSlow26 + (1.0 - fSlow27));
		let mut fSlow31: F32 = F32::exp(-(self.fConst109 / fSlow21));
		let mut fSlow32: F32 = DSP_Output_faustpower2_f(fSlow31);
		let mut fSlow33: F32 = 1.0 - fSlow32;
		let mut fSlow34: F32 = 1.0 - self.fConst101 * fSlow32;
		let mut fSlow35: F32 = F32::sqrt(F32::max(0.0, DSP_Output_faustpower2_f(fSlow34) / DSP_Output_faustpower2_f(fSlow33) + -1.0));
		let mut fSlow36: F32 = fSlow34 / fSlow33;
		let mut fSlow37: F32 = fSlow36 - fSlow35;
		let mut fSlow38: F32 = F32::exp(-(self.fConst109 / fSlow20)) / fSlow31 + -1.0;
		let mut fSlow39: F32 = fSlow31 * (fSlow35 + (1.0 - fSlow36));
		let mut fSlow40: F32 = F32::exp(-(self.fConst114 / fSlow21));
		let mut fSlow41: F32 = DSP_Output_faustpower2_f(fSlow40);
		let mut fSlow42: F32 = 1.0 - fSlow41;
		let mut fSlow43: F32 = 1.0 - self.fConst101 * fSlow41;
		let mut fSlow44: F32 = F32::sqrt(F32::max(0.0, DSP_Output_faustpower2_f(fSlow43) / DSP_Output_faustpower2_f(fSlow42) + -1.0));
		let mut fSlow45: F32 = fSlow43 / fSlow42;
		let mut fSlow46: F32 = fSlow45 - fSlow44;
		let mut fSlow47: F32 = F32::exp(-(self.fConst114 / fSlow20)) / fSlow40 + -1.0;
		let mut fSlow48: F32 = fSlow40 * (fSlow44 + (1.0 - fSlow45));
		let mut fSlow49: F32 = F32::exp(-(self.fConst119 / fSlow21));
		let mut fSlow50: F32 = DSP_Output_faustpower2_f(fSlow49);
		let mut fSlow51: F32 = 1.0 - fSlow50;
		let mut fSlow52: F32 = 1.0 - self.fConst101 * fSlow50;
		let mut fSlow53: F32 = F32::sqrt(F32::max(0.0, DSP_Output_faustpower2_f(fSlow52) / DSP_Output_faustpower2_f(fSlow51) + -1.0));
		let mut fSlow54: F32 = fSlow52 / fSlow51;
		let mut fSlow55: F32 = fSlow54 - fSlow53;
		let mut fSlow56: F32 = F32::exp(-(self.fConst119 / fSlow20)) / fSlow49 + -1.0;
		let mut fSlow57: F32 = fSlow49 * (fSlow53 + (1.0 - fSlow54));
		let mut fSlow58: F32 = F32::exp(-(self.fConst124 / fSlow21));
		let mut fSlow59: F32 = DSP_Output_faustpower2_f(fSlow58);
		let mut fSlow60: F32 = 1.0 - fSlow59;
		let mut fSlow61: F32 = 1.0 - self.fConst101 * fSlow59;
		let mut fSlow62: F32 = F32::sqrt(F32::max(0.0, DSP_Output_faustpower2_f(fSlow61) / DSP_Output_faustpower2_f(fSlow60) + -1.0));
		let mut fSlow63: F32 = fSlow61 / fSlow60;
		let mut fSlow64: F32 = fSlow63 - fSlow62;
		let mut fSlow65: F32 = F32::exp(-(self.fConst124 / fSlow20)) / fSlow58 + -1.0;
		let mut fSlow66: F32 = fSlow58 * (fSlow62 + (1.0 - fSlow63));
		let mut fSlow67: F32 = F32::exp(-(self.fConst129 / fSlow21));
		let mut fSlow68: F32 = DSP_Output_faustpower2_f(fSlow67);
		let mut fSlow69: F32 = 1.0 - fSlow68;
		let mut fSlow70: F32 = 1.0 - self.fConst101 * fSlow68;
		let mut fSlow71: F32 = F32::sqrt(F32::max(0.0, DSP_Output_faustpower2_f(fSlow70) / DSP_Output_faustpower2_f(fSlow69) + -1.0));
		let mut fSlow72: F32 = fSlow70 / fSlow69;
		let mut fSlow73: F32 = fSlow72 - fSlow71;
		let mut fSlow74: F32 = F32::exp(-(self.fConst129 / fSlow20)) / fSlow67 + -1.0;
		let mut fSlow75: F32 = fSlow67 * (fSlow71 + (1.0 - fSlow72));
		let mut fSlow76: F32 = F32::exp(-(self.fConst134 / fSlow21));
		let mut fSlow77: F32 = DSP_Output_faustpower2_f(fSlow76);
		let mut fSlow78: F32 = 1.0 - fSlow77;
		let mut fSlow79: F32 = 1.0 - self.fConst101 * fSlow77;
		let mut fSlow80: F32 = F32::sqrt(F32::max(0.0, DSP_Output_faustpower2_f(fSlow79) / DSP_Output_faustpower2_f(fSlow78) + -1.0));
		let mut fSlow81: F32 = fSlow79 / fSlow78;
		let mut fSlow82: F32 = fSlow81 - fSlow80;
		let mut fSlow83: F32 = F32::exp(-(self.fConst134 / fSlow20)) / fSlow76 + -1.0;
		let mut fSlow84: F32 = fSlow76 * (fSlow80 + (1.0 - fSlow81));
		let mut fSlow85: F32 = F32::exp(-(self.fConst139 / fSlow21));
		let mut fSlow86: F32 = DSP_Output_faustpower2_f(fSlow85);
		let mut fSlow87: F32 = 1.0 - fSlow86;
		let mut fSlow88: F32 = 1.0 - self.fConst101 * fSlow86;
		let mut fSlow89: F32 = F32::sqrt(F32::max(0.0, DSP_Output_faustpower2_f(fSlow88) / DSP_Output_faustpower2_f(fSlow87) + -1.0));
		let mut fSlow90: F32 = fSlow88 / fSlow87;
		let mut fSlow91: F32 = fSlow90 - fSlow89;
		let mut fSlow92: F32 = F32::exp(-(self.fConst139 / fSlow20)) / fSlow85 + -1.0;
		let mut fSlow93: F32 = fSlow85 * (fSlow89 + (1.0 - fSlow90));
		let mut fSlow94: F32 = F32::exp(-(self.fConst109 / fSlow10));
		let mut fSlow95: F32 = DSP_Output_faustpower2_f(fSlow94);
		let mut fSlow96: F32 = 1.0 - fSlow95;
		let mut fSlow97: F32 = 1.0 - self.fConst101 * fSlow95;
		let mut fSlow98: F32 = F32::sqrt(F32::max(0.0, DSP_Output_faustpower2_f(fSlow97) / DSP_Output_faustpower2_f(fSlow96) + -1.0));
		let mut fSlow99: F32 = fSlow97 / fSlow96;
		let mut fSlow100: F32 = fSlow99 - fSlow98;
		let mut fSlow101: F32 = F32::exp(-(self.fConst109 / fSlow9)) / fSlow94 + -1.0;
		let mut fSlow102: F32 = fSlow94 * (fSlow98 + (1.0 - fSlow99));
		let mut fSlow103: F32 = F32::exp(-(self.fConst114 / fSlow10));
		let mut fSlow104: F32 = DSP_Output_faustpower2_f(fSlow103);
		let mut fSlow105: F32 = 1.0 - fSlow104;
		let mut fSlow106: F32 = 1.0 - self.fConst101 * fSlow104;
		let mut fSlow107: F32 = F32::sqrt(F32::max(0.0, DSP_Output_faustpower2_f(fSlow106) / DSP_Output_faustpower2_f(fSlow105) + -1.0));
		let mut fSlow108: F32 = fSlow106 / fSlow105;
		let mut fSlow109: F32 = fSlow108 - fSlow107;
		let mut fSlow110: F32 = F32::exp(-(self.fConst114 / fSlow9)) / fSlow103 + -1.0;
		let mut fSlow111: F32 = fSlow103 * (fSlow107 + (1.0 - fSlow108));
		let mut fSlow112: F32 = F32::exp(-(self.fConst119 / fSlow10));
		let mut fSlow113: F32 = DSP_Output_faustpower2_f(fSlow112);
		let mut fSlow114: F32 = 1.0 - fSlow113;
		let mut fSlow115: F32 = 1.0 - self.fConst101 * fSlow113;
		let mut fSlow116: F32 = F32::sqrt(F32::max(0.0, DSP_Output_faustpower2_f(fSlow115) / DSP_Output_faustpower2_f(fSlow114) + -1.0));
		let mut fSlow117: F32 = fSlow115 / fSlow114;
		let mut fSlow118: F32 = fSlow117 - fSlow116;
		let mut fSlow119: F32 = F32::exp(-(self.fConst119 / fSlow9)) / fSlow112 + -1.0;
		let mut fSlow120: F32 = fSlow112 * (fSlow116 + (1.0 - fSlow117));
		let mut fSlow121: F32 = F32::exp(-(self.fConst124 / fSlow10));
		let mut fSlow122: F32 = DSP_Output_faustpower2_f(fSlow121);
		let mut fSlow123: F32 = 1.0 - fSlow122;
		let mut fSlow124: F32 = 1.0 - self.fConst101 * fSlow122;
		let mut fSlow125: F32 = F32::sqrt(F32::max(0.0, DSP_Output_faustpower2_f(fSlow124) / DSP_Output_faustpower2_f(fSlow123) + -1.0));
		let mut fSlow126: F32 = fSlow124 / fSlow123;
		let mut fSlow127: F32 = fSlow126 - fSlow125;
		let mut fSlow128: F32 = F32::exp(-(self.fConst124 / fSlow9)) / fSlow121 + -1.0;
		let mut fSlow129: F32 = fSlow121 * (fSlow125 + (1.0 - fSlow126));
		let mut fSlow130: F32 = F32::exp(-(self.fConst129 / fSlow10));
		let mut fSlow131: F32 = DSP_Output_faustpower2_f(fSlow130);
		let mut fSlow132: F32 = 1.0 - fSlow131;
		let mut fSlow133: F32 = 1.0 - self.fConst101 * fSlow131;
		let mut fSlow134: F32 = F32::sqrt(F32::max(0.0, DSP_Output_faustpower2_f(fSlow133) / DSP_Output_faustpower2_f(fSlow132) + -1.0));
		let mut fSlow135: F32 = fSlow133 / fSlow132;
		let mut fSlow136: F32 = fSlow135 - fSlow134;
		let mut fSlow137: F32 = F32::exp(-(self.fConst129 / fSlow9)) / fSlow130 + -1.0;
		let mut fSlow138: F32 = fSlow130 * (fSlow134 + (1.0 - fSlow135));
		let mut fSlow139: F32 = F32::exp(-(self.fConst134 / fSlow10));
		let mut fSlow140: F32 = DSP_Output_faustpower2_f(fSlow139);
		let mut fSlow141: F32 = 1.0 - fSlow140;
		let mut fSlow142: F32 = 1.0 - self.fConst101 * fSlow140;
		let mut fSlow143: F32 = F32::sqrt(F32::max(0.0, DSP_Output_faustpower2_f(fSlow142) / DSP_Output_faustpower2_f(fSlow141) + -1.0));
		let mut fSlow144: F32 = fSlow142 / fSlow141;
		let mut fSlow145: F32 = fSlow144 - fSlow143;
		let mut fSlow146: F32 = F32::exp(-(self.fConst134 / fSlow9)) / fSlow139 + -1.0;
		let mut fSlow147: F32 = fSlow139 * (fSlow143 + (1.0 - fSlow144));
		let mut fSlow148: F32 = F32::exp(-(self.fConst139 / fSlow10));
		let mut fSlow149: F32 = DSP_Output_faustpower2_f(fSlow148);
		let mut fSlow150: F32 = 1.0 - fSlow149;
		let mut fSlow151: F32 = 1.0 - self.fConst101 * fSlow149;
		let mut fSlow152: F32 = F32::sqrt(F32::max(0.0, DSP_Output_faustpower2_f(fSlow151) / DSP_Output_faustpower2_f(fSlow150) + -1.0));
		let mut fSlow153: F32 = fSlow151 / fSlow150;
		let mut fSlow154: F32 = fSlow153 - fSlow152;
		let mut fSlow155: F32 = F32::exp(-(self.fConst139 / fSlow9)) / fSlow148 + -1.0;
		let mut fSlow156: F32 = fSlow148 * (fSlow152 + (1.0 - fSlow153));
		let mut fSlow157: F32 = 0.37 / fSlow6;
		let mut fSlow158: F32 = 1.0 / (fSlow7 + 1.0);
		let mut fSlow159: F32 = 0.01 * self.fHslider8;
		let mut fSlow160: F32 = self.fConst145 * self.fHslider9;
		let mut fSlow161: F32 = 1.0 / F32::max(0.01 * self.fHslider10, 0.001);
		let mut fSlow162: F32 = 0.0001 * self.fHslider11;
		let mut fSlow163: F32 = 0.0017782794 * self.fHslider12;
		let mut fSlow164: F32 = self.fHslider13;
		let mut fSlow165: F32 = 1.0 / F32::min(F32::max(0.003 * fSlow164 + 0.5, 0.001), 0.999) + -2.0;
		let mut fSlow166: F32 = 1.0 - 0.00525 * fSlow164;
		let mut fSlow167: F32 = self.fHslider14;
		let mut fSlow168: F32 = ((fSlow167 < 15.9) as i32) as u32 as F32;
		let mut fSlow169: F32 = (1.0 - fSlow168) * fSlow166;
		let mut fSlow170: F32 = F32::powf(2.0, fSlow167) + -1.0;
		let mut fSlow171: F32 = fSlow166 * fSlow170;
		let mut fSlow172: F32 = fSlow168 / fSlow170;
		let mut fSlow173: F32 = F32::min(self.fConst0, 1e+03 * self.fHslider15);
		let mut fSlow174: F32 = ((fSlow173 < 44099.9) as i32) as u32 as F32;
		let mut fSlow175: F32 = 1.0 - fSlow174;
		let mut fSlow176: F32 = self.fConst0 / F32::min(fSlow173, self.fConst0);
		let mut iSlow177: i32 = (fSlow176) as i32;
		let mut fSlow178: F32 = 0.01 * self.fHslider16;
		let mut fSlow179: F32 = self.fHslider17;
		let mut fSlow180: F32 = 0.005 * fSlow179;
		let mut fSlow181: F32 = F32::max(-1e+02, self.fHslider18);
		let mut fSlow182: F32 = 0.0025 * fSlow179 + 1.0;
		let zipped_iterators = inputs0.zip(outputs0).zip(outputs1);
		for ((input0, output0), output1) in zipped_iterators {
			self.iVec0[0] = 1;
			self.fVec1[0] = fSlow0;
			let mut fTemp0: F32 = ((fSlow0 <= self.fVec1[1]) as i32) as u32 as F32;
			let mut fTemp1: F32 = 1.0 - self.fConst1 * fTemp0;
			self.fRec0[0] = fSlow1 * fTemp1 + self.fConst1 * fTemp0 * self.fRec0[1];
			let mut fTemp2: F32 = 17.31234 * F32::log(0.0022727272 * self.fRec0[0], std::f32::consts::E);
			self.fRec1[0] = fSlow2 * fTemp1 + self.fConst1 * fTemp0 * self.fRec1[1];
			let mut fTemp3: F32 = F32::tan(self.fConst3 * F32::powf(2.0, 0.083333336 * (fTemp2 + self.fRec1[0] * (24.0 - fTemp2))));
			let mut fTemp4: F32 = 1.0 / fTemp3;
			let mut fTemp5: F32 = (fTemp4 + 0.76536685) / fTemp3 + 1.0;
			let mut fTemp6: F32 = 1.0 - 1.0 / DSP_Output_faustpower2_f(fTemp3);
			let mut fTemp7: F32 = (fTemp4 + -0.76536685) / fTemp3 + 1.0;
			let mut fTemp8: F32 = (fTemp4 + 1.847759) / fTemp3 + 1.0;
			let mut fTemp9: F32 = (fTemp4 + -1.847759) / fTemp3 + 1.0;
			self.fRec4[0] = fSlow3 * fTemp1 + self.fConst1 * fTemp0 * self.fRec4[1];
			let mut fTemp10: F32 = 17.31234 * F32::log(0.0022727272 * self.fRec4[0], std::f32::consts::E);
			let mut fTemp11: F32 = F32::tan(self.fConst3 * F32::powf(2.0, 0.083333336 * (fTemp10 + self.fRec1[0] * (12.0 - fTemp10))));
			let mut fTemp12: F32 = 1.0 / fTemp11;
			let mut fTemp13: F32 = (fTemp12 + 1.4142135) / fTemp11 + 1.0;
			let mut fTemp14: F32 = DSP_Output_faustpower2_f(fTemp11);
			let mut fTemp15: F32 = fTemp14 * fTemp13;
			let mut fTemp16: F32 = 1.0 - 1.0 / fTemp14;
			let mut fTemp17: F32 = (fTemp12 + -1.4142135) / fTemp11 + 1.0;
			let mut iTemp18: i32 = i32::wrapping_sub(1, self.iVec0[1]);
			let mut fTemp19: F32 = if iTemp18 != 0 {0.0} else {self.fConst31 + self.fRec19[1]};
			self.fRec19[0] = fTemp19 - F32::floor(fTemp19);
			let mut fTemp20: F32 = if iTemp18 != 0 {0.0} else {self.fConst32 + self.fRec20[1]};
			self.fRec20[0] = fTemp20 - F32::floor(fTemp20);
			let mut fTemp21: F32 = if iTemp18 != 0 {0.0} else {self.fConst33 + self.fRec21[1]};
			self.fRec21[0] = fTemp21 - F32::floor(fTemp21);
			let mut fTemp22: F32 = 0.025 * (unsafe { ftbl0DSP_OutputSIG0[((65536.0 * self.fRec21[0]) as i32) as usize] } + 1.0) * (unsafe { ftbl0DSP_OutputSIG0[((65536.0 * self.fRec20[0]) as i32) as usize] } + 1.0) * (unsafe { ftbl0DSP_OutputSIG0[((65536.0 * self.fRec19[0]) as i32) as usize] } + 1.0) + 0.8;
			let mut fTemp23: F32 = 1e+01 * fTemp22;
			let mut iTemp24: i32 = (fTemp23 > 0.0) as i32;
			let mut fTemp25: F32 = self.fConst34 * F32::powf(1e+01, 0.05 * F32::abs(fTemp23));
			let mut fTemp26: F32 = if iTemp24 != 0 {self.fConst34} else {fTemp25};
			let mut fTemp27: F32 = self.fConst36 * (self.fConst36 + fTemp26) + 1.0;
			let mut fTemp28: F32 = self.fConst37 * self.fRec22[1];
			let mut fTemp29: F32 = self.fConst36 * (self.fConst36 - fTemp26) + 1.0;
			self.iRec26[0] = i32::wrapping_add(self.iRec26[1], 1);
			let mut iTemp30: i32 = i32::wrapping_add(self.iRec26[0], -1);
			let mut iTemp31: i32 = ((iTemp30 % self.iConst56) == 0) as i32;
			self.iRec30[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec30[1]), 12345);
			let mut fTemp32: F32 = (self.iRec30[0]) as F32;
			self.fVec3[0] = fTemp32;
			self.fRec29[0] = if iTemp31 != 0 {4.656613e-10 * fTemp32} else {self.fRec29[1]};
			let mut fTemp33: F32 = if (self.fRec29[0] != self.fRec29[1]) as i32 != 0 {self.fConst55} else {self.fRec27[1] + -1.0};
			self.fRec27[0] = fTemp33;
			self.fRec28[0] = if (fTemp33 > 0.0) as i32 != 0 {self.fRec28[1] + (self.fRec29[0] - self.fRec28[1]) / fTemp33} else {self.fRec29[0]};
			self.fRec25[0] = if ((iTemp30 % self.iConst53) == 0) as i32 != 0 {3.5 * (self.fRec28[0] + 1.0) + 1.0} else {self.fRec25[1]};
			let mut fTemp34: F32 = if iTemp18 != 0 {0.0} else {self.fRec34[1] + self.fConst58 * fTemp22};
			self.fRec34[0] = fTemp34 - F32::floor(fTemp34);
			let mut fTemp35: F32 = self.fRec34[0] - self.fRec34[1];
			self.fVec4[0] = fTemp35;
			let mut iTemp36: i32 = ((self.fVec4[1] <= 0.0) as i32) & ((fTemp35 > 0.0) as i32);
			self.fRec33[0] = self.fRec33[1] * (i32::wrapping_sub(1, iTemp36)) as F32 + 4.656613e-10 * fTemp32 * (iTemp36) as F32;
			let mut fTemp37: F32 = 0.5 * (self.fRec33[0] + 1.0);
			let mut fTemp38: F32 = 4.656613e-10 * self.fVec3[1] * (((self.fRec34[0] >= fTemp37) as i32) * ((self.fRec34[1] < fTemp37) as i32)) as F32;
			let mut iTemp39: i32 = i32::abs(((fTemp38 > 0.0) as i32) - ((fTemp38 < 0.0) as i32));
			self.fRec32[0] = if (iTemp39 > 0) as i32 != 0 {self.fConst59} else {F32::max(0.0, self.fRec32[1] + -1.0)};
			self.fRec35[0] = if iTemp39 != 0 {fTemp38} else {self.fRec35[1]};
			let mut fTemp40: F32 = self.fRec35[0] * ((self.fRec32[0] > 0.0) as i32) as u32 as F32;
			let mut iTemp41: i32 = ((fTemp40 > 0.0) as i32) - ((fTemp40 < 0.0) as i32);
			self.iVec5[0] = iTemp41;
			self.iRec31[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec31[1], (self.iRec31[1] > 0) as i32), (iTemp41 <= self.iVec5[1]) as i32), (iTemp41 > self.iVec5[1]) as i32);
			let mut fTemp42: F32 = (self.iRec31[0]) as F32 / F32::max(1.0, self.fConst57 * self.fRec25[0]);
			let mut fTemp43: F32 = F32::max(0.0, F32::min(fTemp42, 2.0 - fTemp42));
			let mut iTemp44: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(self.iRec39[1], 12345));
			let mut iTemp45: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(iTemp44, 12345));
			let mut iTemp46: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(iTemp45, 12345));
			self.iRec39[0] = i32::wrapping_mul(1103515245, i32::wrapping_add(iTemp46, 12345));
			let mut iRec40: i32 = iTemp46;
			let mut iRec41: i32 = iTemp45;
			let mut iRec42: i32 = iTemp44;
			self.fRec38[0] = if iTemp31 != 0 {4.656613e-10 * (iRec42) as F32} else {self.fRec38[1]};
			let mut fTemp47: F32 = if (self.fRec38[0] != self.fRec38[1]) as i32 != 0 {self.fConst55} else {self.fRec36[1] + -1.0};
			self.fRec36[0] = fTemp47;
			self.fRec37[0] = if (fTemp47 > 0.0) as i32 != 0 {self.fRec37[1] + (self.fRec38[0] - self.fRec37[1]) / fTemp47} else {self.fRec38[0]};
			self.fRec24[0] = self.fRec37[0] * fTemp43 - self.fConst51 * (self.fConst49 * self.fRec24[1] + self.fConst48 * self.fRec24[2]);
			let mut fTemp48: F32 = if iTemp18 != 0 {0.0} else {self.fConst61 + self.fRec47[1]};
			self.fRec47[0] = fTemp48 - F32::floor(fTemp48);
			let mut fTemp49: F32 = self.fRec47[0] - self.fRec47[1];
			self.fVec6[0] = fTemp49;
			let mut iTemp50: i32 = ((self.fVec6[1] <= 0.0) as i32) & ((fTemp49 > 0.0) as i32);
			self.fRec46[0] = self.fRec46[1] * (i32::wrapping_sub(1, iTemp50)) as F32 + 4.656613e-10 * fTemp32 * (iTemp50) as F32;
			let mut fTemp51: F32 = 0.5 * (self.fRec46[0] + 1.0);
			let mut fTemp52: F32 = 4.656613e-10 * self.fVec3[1] * (((self.fRec47[0] >= fTemp51) as i32) * ((self.fRec47[1] < fTemp51) as i32)) as F32;
			let mut iTemp53: i32 = i32::abs(((fTemp52 > 0.0) as i32) - ((fTemp52 < 0.0) as i32));
			self.fRec45[0] = if (iTemp53 > 0) as i32 != 0 {self.fConst59} else {F32::max(0.0, self.fRec45[1] + -1.0)};
			self.fRec48[0] = if iTemp53 != 0 {fTemp52} else {self.fRec48[1]};
			let mut fTemp54: F32 = self.fRec48[0] * ((self.fRec45[0] > 0.0) as i32) as u32 as F32;
			let mut iTemp55: i32 = ((fTemp54 > 0.0) as i32) - ((fTemp54 < 0.0) as i32);
			self.iVec7[0] = iTemp55;
			self.iRec44[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec44[1], (self.iRec44[1] > 0) as i32), (iTemp55 <= self.iVec7[1]) as i32), (iTemp55 > self.iVec7[1]) as i32);
			let mut fTemp56: F32 = (self.iRec44[0]) as F32 / F32::max(1.0, self.fConst60 * self.fRec25[0]);
			let mut fTemp57: F32 = F32::max(0.0, F32::min(fTemp56, 2.0 - fTemp56));
			self.fRec43[0] = self.fRec37[0] * fTemp57 - self.fConst51 * (self.fConst49 * self.fRec43[1] + self.fConst48 * self.fRec43[2]);
			self.fRec51[0] = if iTemp31 != 0 {4.656613e-10 * (iRec40) as F32} else {self.fRec51[1]};
			let mut fTemp58: F32 = if (self.fRec51[0] != self.fRec51[1]) as i32 != 0 {self.fConst55} else {self.fRec49[1] + -1.0};
			self.fRec49[0] = fTemp58;
			self.fRec50[0] = if (fTemp58 > 0.0) as i32 != 0 {self.fRec50[1] + (self.fRec51[0] - self.fRec50[1]) / fTemp58} else {self.fRec51[0]};
			let mut fTemp59: F32 = fTemp22 * (self.fRec50[0] + self.fConst62 * (3e+01 * (self.fRec43[0] - self.fRec43[2]) + 2e+01 * (self.fRec24[0] - self.fRec24[2])));
			self.fVec8[0] = fTemp59;
			self.fRec23[0] = self.fConst63 * (0.0891251 * (fTemp59 + self.fVec8[1]) - self.fConst40 * self.fRec23[1]);
			self.fRec52[0] = self.fConst63 * (self.fConst64 * (fTemp59 - self.fVec8[1]) - self.fConst40 * self.fRec52[1]);
			let mut fTemp60: F32 = self.fRec52[0] + 7.0794578 * self.fRec23[0];
			self.fVec9[0] = fTemp60;
			self.fRec53[0] = -(self.fConst67 * (self.fConst66 * self.fRec53[1] - (fTemp60 + self.fVec9[1])));
			self.fRec22[0] = 0.75 * self.fRec53[0] + 0.25 * fTemp60 - (fTemp29 * self.fRec22[2] + fTemp28) / fTemp27;
			let mut fTemp61: F32 = if iTemp24 != 0 {fTemp25} else {self.fConst34};
			let mut fTemp62: F32 = self.fConst36 * (self.fConst36 - fTemp61) + 1.0;
			let mut fTemp63: F32 = self.fConst36 * (self.fConst36 + fTemp61) + 1.0;
			self.fRec17[0] = (fTemp28 + self.fRec22[0] * fTemp63 + fTemp62 * self.fRec22[2]) / fTemp27 - self.fConst30 * (self.fConst29 * self.fRec17[2] + self.fConst5 * self.fRec17[1]);
			self.fRec16[0] = self.fConst30 * (self.fRec17[2] + self.fRec17[0] + 2.0 * self.fRec17[1]) - self.fConst28 * (self.fConst27 * self.fRec16[2] + self.fConst5 * self.fRec16[1]);
			self.fRec15[0] = self.fConst28 * (self.fRec16[2] + self.fRec16[0] + 2.0 * self.fRec16[1]) - self.fConst26 * (self.fConst25 * self.fRec15[2] + self.fConst5 * self.fRec15[1]);
			self.fRec14[0] = self.fConst26 * (self.fRec15[2] + self.fRec15[0] + 2.0 * self.fRec15[1]) - self.fConst24 * (self.fConst23 * self.fRec14[2] + self.fConst5 * self.fRec14[1]);
			self.fRec13[0] = self.fConst24 * (self.fRec14[2] + self.fRec14[0] + 2.0 * self.fRec14[1]) - self.fConst22 * (self.fConst21 * self.fRec13[2] + self.fConst5 * self.fRec13[1]);
			self.fRec12[0] = self.fConst22 * (self.fRec13[2] + self.fRec13[0] + 2.0 * self.fRec13[1]) - self.fConst20 * (self.fConst19 * self.fRec12[2] + self.fConst5 * self.fRec12[1]);
			self.fRec11[0] = self.fConst20 * (self.fRec12[2] + self.fRec12[0] + 2.0 * self.fRec12[1]) - self.fConst18 * (self.fConst17 * self.fRec11[2] + self.fConst5 * self.fRec11[1]);
			self.fRec10[0] = self.fConst18 * (self.fRec11[2] + self.fRec11[0] + 2.0 * self.fRec11[1]) - self.fConst16 * (self.fConst15 * self.fRec10[2] + self.fConst5 * self.fRec10[1]);
			self.fRec9[0] = self.fConst16 * (self.fRec10[2] + self.fRec10[0] + 2.0 * self.fRec10[1]) - self.fConst14 * (self.fConst13 * self.fRec9[2] + self.fConst5 * self.fRec9[1]);
			self.fRec8[0] = self.fConst14 * (self.fRec9[2] + self.fRec9[0] + 2.0 * self.fRec9[1]) - self.fConst12 * (self.fConst11 * self.fRec8[2] + self.fConst5 * self.fRec8[1]);
			self.fRec7[0] = self.fConst12 * (self.fRec8[2] + self.fRec8[0] + 2.0 * self.fRec8[1]) - self.fConst10 * (self.fConst9 * self.fRec7[2] + self.fConst5 * self.fRec7[1]);
			self.fRec6[0] = self.fConst10 * (self.fRec7[2] + self.fRec7[0] + 2.0 * self.fRec7[1]) - self.fConst8 * (self.fConst7 * self.fRec6[2] + self.fConst5 * self.fRec6[1]);
			let mut fTemp64: F32 = self.fConst8 * (self.fRec6[2] + self.fRec6[0] + 2.0 * self.fRec6[1]);
			let mut iTemp65: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(self.iRec70[1], 12345));
			self.iRec70[0] = i32::wrapping_mul(1103515245, i32::wrapping_add(iTemp65, 12345));
			let mut iRec71: i32 = iTemp65;
			self.fRec69[0] = if iTemp31 != 0 {4.656613e-10 * (iRec71) as F32} else {self.fRec69[1]};
			let mut fTemp66: F32 = if (self.fRec69[0] != self.fRec69[1]) as i32 != 0 {self.fConst55} else {self.fRec67[1] + -1.0};
			self.fRec67[0] = fTemp66;
			self.fRec68[0] = if (fTemp66 > 0.0) as i32 != 0 {self.fRec68[1] + (self.fRec69[0] - self.fRec68[1]) / fTemp66} else {self.fRec69[0]};
			self.fRec66[0] = -(self.fConst63 * (self.fConst40 * self.fRec66[1] - (self.fRec68[0] + self.fRec68[1])));
			self.fRec72[0] = -(self.fConst63 * (self.fConst40 * self.fRec72[1] - self.fConst39 * (self.fRec68[0] - self.fRec68[1])));
			self.fRec65[0] = self.fRec72[0] + 7.0794578 * self.fRec66[0] - self.fConst95 * (self.fConst94 * self.fRec65[2] + self.fConst69 * self.fRec65[1]);
			self.fRec64[0] = self.fConst95 * (self.fRec65[2] + self.fRec65[0] + 2.0 * self.fRec65[1]) - self.fConst93 * (self.fConst92 * self.fRec64[2] + self.fConst69 * self.fRec64[1]);
			self.fRec63[0] = self.fConst93 * (self.fRec64[2] + self.fRec64[0] + 2.0 * self.fRec64[1]) - self.fConst91 * (self.fConst90 * self.fRec63[2] + self.fConst69 * self.fRec63[1]);
			self.fRec62[0] = self.fConst91 * (self.fRec63[2] + self.fRec63[0] + 2.0 * self.fRec63[1]) - self.fConst89 * (self.fConst88 * self.fRec62[2] + self.fConst69 * self.fRec62[1]);
			self.fRec61[0] = self.fConst89 * (self.fRec62[2] + self.fRec62[0] + 2.0 * self.fRec62[1]) - self.fConst87 * (self.fConst86 * self.fRec61[2] + self.fConst69 * self.fRec61[1]);
			self.fRec60[0] = self.fConst87 * (self.fRec61[2] + self.fRec61[0] + 2.0 * self.fRec61[1]) - self.fConst85 * (self.fConst84 * self.fRec60[2] + self.fConst69 * self.fRec60[1]);
			self.fRec59[0] = self.fConst85 * (self.fRec60[2] + self.fRec60[0] + 2.0 * self.fRec60[1]) - self.fConst83 * (self.fConst82 * self.fRec59[2] + self.fConst69 * self.fRec59[1]);
			self.fRec58[0] = self.fConst83 * (self.fRec59[2] + self.fRec59[0] + 2.0 * self.fRec59[1]) - self.fConst81 * (self.fConst80 * self.fRec58[2] + self.fConst69 * self.fRec58[1]);
			self.fRec57[0] = self.fConst81 * (self.fRec58[2] + self.fRec58[0] + 2.0 * self.fRec58[1]) - self.fConst79 * (self.fConst78 * self.fRec57[2] + self.fConst69 * self.fRec57[1]);
			self.fRec56[0] = self.fConst79 * (self.fRec57[2] + self.fRec57[0] + 2.0 * self.fRec57[1]) - self.fConst77 * (self.fConst76 * self.fRec56[2] + self.fConst69 * self.fRec56[1]);
			self.fRec55[0] = self.fConst77 * (self.fRec56[2] + self.fRec56[0] + 2.0 * self.fRec56[1]) - self.fConst75 * (self.fConst74 * self.fRec55[2] + self.fConst69 * self.fRec55[1]);
			self.fRec54[0] = self.fConst75 * (self.fRec55[2] + self.fRec55[0] + 2.0 * self.fRec55[1]) - self.fConst73 * (self.fConst71 * self.fRec54[2] + self.fConst69 * self.fRec54[1]);
			self.fRec87[0] = -(self.fConst104 * (self.fConst103 * self.fRec87[1] - (self.fRec80[1] + self.fRec80[2])));
			self.fRec86[0] = fSlow19 * (self.fRec80[1] + fSlow18 * self.fRec87[0]) + fSlow17 * self.fRec86[1];
			self.fVec10[(self.IOTA0 & 65535) as usize] = 0.35355338 * self.fRec86[0] + 1e-20;
			self.fRec99[0] = -(self.fConst104 * (self.fConst103 * self.fRec99[1] - (self.fRec92[1] + self.fRec92[2])));
			self.fRec98[0] = fSlow30 * (self.fRec92[1] + fSlow29 * self.fRec99[0]) + fSlow28 * self.fRec98[1];
			self.fVec11[(self.IOTA0 & 65535) as usize] = 0.35355338 * self.fRec98[0] + 1e-20;
			let mut fTemp67: F32 = 0.3 * *input0;
			let mut fTemp68: F32 = fTemp67 + self.fVec11[((i32::wrapping_sub(self.IOTA0, self.iConst106)) & 65535) as usize] - 0.6 * self.fRec96[1];
			self.fVec12[(self.IOTA0 & 8191) as usize] = fTemp68;
			self.fRec96[0] = self.fVec12[((i32::wrapping_sub(self.IOTA0, self.iConst107)) & 8191) as usize];
			let mut fRec97: F32 = 0.6 * fTemp68;
			self.fRec103[0] = -(self.fConst104 * (self.fConst103 * self.fRec103[1] - (self.fRec88[1] + self.fRec88[2])));
			self.fRec102[0] = fSlow39 * (self.fRec88[1] + fSlow38 * self.fRec103[0]) + fSlow37 * self.fRec102[1];
			self.fVec13[(self.IOTA0 & 32767) as usize] = 0.35355338 * self.fRec102[0] + 1e-20;
			let mut fTemp69: F32 = self.fVec13[((i32::wrapping_sub(self.IOTA0, self.iConst111)) & 32767) as usize] + fTemp67 - 0.6 * self.fRec100[1];
			self.fVec14[(self.IOTA0 & 4095) as usize] = fTemp69;
			self.fRec100[0] = self.fVec14[((i32::wrapping_sub(self.IOTA0, self.iConst112)) & 4095) as usize];
			let mut fRec101: F32 = 0.6 * fTemp69;
			let mut fTemp70: F32 = fRec101 + fRec97;
			self.fRec107[0] = -(self.fConst104 * (self.fConst103 * self.fRec107[1] - (self.fRec90[1] + self.fRec90[2])));
			self.fRec106[0] = fSlow48 * (self.fRec90[1] + fSlow47 * self.fRec107[0]) + fSlow46 * self.fRec106[1];
			self.fVec15[(self.IOTA0 & 32767) as usize] = 0.35355338 * self.fRec106[0] + 1e-20;
			let mut fTemp71: F32 = self.fVec15[((i32::wrapping_sub(self.IOTA0, self.iConst116)) & 32767) as usize] - (fTemp67 + 0.6 * self.fRec104[1]);
			self.fVec16[(self.IOTA0 & 8191) as usize] = fTemp71;
			self.fRec104[0] = self.fVec16[((i32::wrapping_sub(self.IOTA0, self.iConst117)) & 8191) as usize];
			let mut fRec105: F32 = 0.6 * fTemp71;
			self.fRec111[0] = -(self.fConst104 * (self.fConst103 * self.fRec111[1] - (self.fRec94[1] + self.fRec94[2])));
			self.fRec110[0] = fSlow57 * (self.fRec94[1] + fSlow56 * self.fRec111[0]) + fSlow55 * self.fRec110[1];
			self.fVec17[(self.IOTA0 & 32767) as usize] = 0.35355338 * self.fRec110[0] + 1e-20;
			let mut fTemp72: F32 = self.fVec17[((i32::wrapping_sub(self.IOTA0, self.iConst121)) & 32767) as usize] - (fTemp67 + 0.6 * self.fRec108[1]);
			self.fVec18[(self.IOTA0 & 4095) as usize] = fTemp72;
			self.fRec108[0] = self.fVec18[((i32::wrapping_sub(self.IOTA0, self.iConst122)) & 4095) as usize];
			let mut fRec109: F32 = 0.6 * fTemp72;
			let mut fTemp73: F32 = fRec109 + fRec105 + fTemp70;
			self.fRec115[0] = -(self.fConst104 * (self.fConst103 * self.fRec115[1] - (self.fRec89[1] + self.fRec89[2])));
			self.fRec114[0] = fSlow66 * (self.fRec89[1] + fSlow65 * self.fRec115[0]) + fSlow64 * self.fRec114[1];
			self.fVec19[(self.IOTA0 & 65535) as usize] = 0.35355338 * self.fRec114[0] + 1e-20;
			let mut fTemp74: F32 = self.fVec19[((i32::wrapping_sub(self.IOTA0, self.iConst126)) & 65535) as usize] + fTemp67 + 0.6 * self.fRec112[1];
			self.fVec20[(self.IOTA0 & 8191) as usize] = fTemp74;
			self.fRec112[0] = self.fVec20[((i32::wrapping_sub(self.IOTA0, self.iConst127)) & 8191) as usize];
			let mut fRec113: F32 = -(0.6 * fTemp74);
			self.fRec119[0] = -(self.fConst104 * (self.fConst103 * self.fRec119[1] - (self.fRec93[1] + self.fRec93[2])));
			self.fRec118[0] = fSlow75 * (self.fRec93[1] + fSlow74 * self.fRec119[0]) + fSlow73 * self.fRec118[1];
			self.fVec21[(self.IOTA0 & 65535) as usize] = 0.35355338 * self.fRec118[0] + 1e-20;
			let mut fTemp75: F32 = self.fVec21[((i32::wrapping_sub(self.IOTA0, self.iConst131)) & 65535) as usize] + fTemp67 + 0.6 * self.fRec116[1];
			self.fVec22[(self.IOTA0 & 8191) as usize] = fTemp75;
			self.fRec116[0] = self.fVec22[((i32::wrapping_sub(self.IOTA0, self.iConst132)) & 8191) as usize];
			let mut fRec117: F32 = -(0.6 * fTemp75);
			self.fRec123[0] = -(self.fConst104 * (self.fConst103 * self.fRec123[1] - (self.fRec91[1] + self.fRec91[2])));
			self.fRec122[0] = fSlow84 * (self.fRec91[1] + fSlow83 * self.fRec123[0]) + fSlow82 * self.fRec122[1];
			self.fVec23[(self.IOTA0 & 65535) as usize] = 0.35355338 * self.fRec122[0] + 1e-20;
			let mut fTemp76: F32 = 0.6 * self.fRec120[1] + self.fVec23[((i32::wrapping_sub(self.IOTA0, self.iConst136)) & 65535) as usize];
			self.fVec24[(self.IOTA0 & 8191) as usize] = fTemp76 - fTemp67;
			self.fRec120[0] = self.fVec24[((i32::wrapping_sub(self.IOTA0, self.iConst137)) & 8191) as usize];
			let mut fRec121: F32 = 0.6 * (fTemp67 - fTemp76);
			self.fRec127[0] = -(self.fConst104 * (self.fConst103 * self.fRec127[1] - (self.fRec95[1] + self.fRec95[2])));
			self.fRec126[0] = fSlow93 * (self.fRec95[1] + fSlow92 * self.fRec127[0]) + fSlow91 * self.fRec126[1];
			self.fVec25[(self.IOTA0 & 65535) as usize] = 0.35355338 * self.fRec126[0] + 1e-20;
			let mut fTemp77: F32 = 0.6 * self.fRec124[1] + self.fVec25[((i32::wrapping_sub(self.IOTA0, self.iConst141)) & 65535) as usize];
			self.fVec26[(self.IOTA0 & 4095) as usize] = fTemp77 - fTemp67;
			self.fRec124[0] = self.fVec26[((i32::wrapping_sub(self.IOTA0, self.iConst142)) & 4095) as usize];
			let mut fRec125: F32 = 0.6 * (fTemp67 - fTemp77);
			self.fRec88[0] = self.fRec124[1] + self.fRec120[1] + self.fRec116[1] + self.fRec112[1] + self.fRec108[1] + self.fRec104[1] + self.fRec96[1] + self.fRec100[1] + fRec125 + fRec121 + fRec117 + fRec113 + fTemp73;
			self.fRec89[0] = self.fRec108[1] + self.fRec104[1] + self.fRec96[1] + self.fRec100[1] + fTemp73 - (self.fRec124[1] + self.fRec120[1] + self.fRec116[1] + self.fRec112[1] + fRec125 + fRec121 + fRec113 + fRec117);
			let mut fTemp78: F32 = fRec105 + fRec109;
			self.fRec90[0] = self.fRec116[1] + self.fRec112[1] + self.fRec96[1] + self.fRec100[1] + fRec117 + fRec113 + fTemp70 - (self.fRec124[1] + self.fRec120[1] + self.fRec108[1] + self.fRec104[1] + fRec125 + fRec121 + fTemp78);
			self.fRec91[0] = self.fRec124[1] + self.fRec120[1] + self.fRec96[1] + self.fRec100[1] + fRec125 + fRec121 + fTemp70 - (self.fRec116[1] + self.fRec112[1] + self.fRec108[1] + self.fRec104[1] + fRec117 + fRec113 + fTemp78);
			let mut fTemp79: F32 = fRec97 + fRec109;
			let mut fTemp80: F32 = fRec101 + fRec105;
			self.fRec92[0] = self.fRec120[1] + self.fRec112[1] + self.fRec104[1] + self.fRec100[1] + fRec121 + fRec113 + fTemp80 - (self.fRec124[1] + self.fRec116[1] + self.fRec108[1] + self.fRec96[1] + fRec125 + fRec117 + fTemp79);
			self.fRec93[0] = self.fRec124[1] + self.fRec116[1] + self.fRec104[1] + self.fRec100[1] + fRec125 + fRec117 + fTemp80 - (self.fRec120[1] + self.fRec112[1] + self.fRec108[1] + self.fRec96[1] + fRec121 + fRec113 + fTemp79);
			let mut fTemp81: F32 = fRec97 + fRec105;
			let mut fTemp82: F32 = fRec101 + fRec109;
			self.fRec94[0] = self.fRec124[1] + self.fRec112[1] + self.fRec108[1] + self.fRec100[1] + fRec125 + fRec113 + fTemp82 - (self.fRec120[1] + self.fRec116[1] + self.fRec104[1] + self.fRec96[1] + fRec121 + fRec117 + fTemp81);
			self.fRec95[0] = self.fRec120[1] + self.fRec116[1] + self.fRec108[1] + self.fRec100[1] + fRec121 + fRec117 + fTemp82 - (self.fRec124[1] + self.fRec112[1] + self.fRec104[1] + self.fRec96[1] + fRec125 + fRec113 + fTemp81);
			let mut fTemp83: F32 = self.fRec89[0] - self.fRec90[0];
			self.fVec27[(self.IOTA0 & 4095) as usize] = fTemp83;
			let mut fTemp84: F32 = 0.111 * self.fVec27[((i32::wrapping_sub(self.IOTA0, self.iConst143)) & 4095) as usize];
			let mut fTemp85: F32 = fTemp84 + self.fVec10[((i32::wrapping_sub(self.IOTA0, self.iConst106)) & 65535) as usize] - 0.6 * self.fRec84[1];
			self.fVec28[(self.IOTA0 & 8191) as usize] = fTemp85;
			self.fRec84[0] = self.fVec28[((i32::wrapping_sub(self.IOTA0, self.iConst107)) & 8191) as usize];
			let mut fRec85: F32 = 0.6 * fTemp85;
			self.fRec131[0] = -(self.fConst104 * (self.fConst103 * self.fRec131[1] - (self.fRec76[1] + self.fRec76[2])));
			self.fRec130[0] = fSlow102 * (self.fRec76[1] + fSlow101 * self.fRec131[0]) + fSlow100 * self.fRec130[1];
			self.fVec29[(self.IOTA0 & 32767) as usize] = 0.35355338 * self.fRec130[0] + 1e-20;
			let mut fTemp86: F32 = self.fVec29[((i32::wrapping_sub(self.IOTA0, self.iConst111)) & 32767) as usize] + fTemp84 - 0.6 * self.fRec128[1];
			self.fVec30[(self.IOTA0 & 4095) as usize] = fTemp86;
			self.fRec128[0] = self.fVec30[((i32::wrapping_sub(self.IOTA0, self.iConst112)) & 4095) as usize];
			let mut fRec129: F32 = 0.6 * fTemp86;
			let mut fTemp87: F32 = fRec129 + fRec85;
			self.fRec135[0] = -(self.fConst104 * (self.fConst103 * self.fRec135[1] - (self.fRec78[1] + self.fRec78[2])));
			self.fRec134[0] = fSlow111 * (self.fRec78[1] + fSlow110 * self.fRec135[0]) + fSlow109 * self.fRec134[1];
			self.fVec31[(self.IOTA0 & 32767) as usize] = 0.35355338 * self.fRec134[0] + 1e-20;
			let mut fTemp88: F32 = self.fVec31[((i32::wrapping_sub(self.IOTA0, self.iConst116)) & 32767) as usize] - (fTemp84 + 0.6 * self.fRec132[1]);
			self.fVec32[(self.IOTA0 & 8191) as usize] = fTemp88;
			self.fRec132[0] = self.fVec32[((i32::wrapping_sub(self.IOTA0, self.iConst117)) & 8191) as usize];
			let mut fRec133: F32 = 0.6 * fTemp88;
			self.fRec139[0] = -(self.fConst104 * (self.fConst103 * self.fRec139[1] - (self.fRec82[1] + self.fRec82[2])));
			self.fRec138[0] = fSlow120 * (self.fRec82[1] + fSlow119 * self.fRec139[0]) + fSlow118 * self.fRec138[1];
			self.fVec33[(self.IOTA0 & 32767) as usize] = 0.35355338 * self.fRec138[0] + 1e-20;
			let mut fTemp89: F32 = self.fVec33[((i32::wrapping_sub(self.IOTA0, self.iConst121)) & 32767) as usize] - (fTemp84 + 0.6 * self.fRec136[1]);
			self.fVec34[(self.IOTA0 & 4095) as usize] = fTemp89;
			self.fRec136[0] = self.fVec34[((i32::wrapping_sub(self.IOTA0, self.iConst122)) & 4095) as usize];
			let mut fRec137: F32 = 0.6 * fTemp89;
			let mut fTemp90: F32 = fRec137 + fRec133 + fTemp87;
			self.fRec143[0] = -(self.fConst104 * (self.fConst103 * self.fRec143[1] - (self.fRec77[1] + self.fRec77[2])));
			self.fRec142[0] = fSlow129 * (self.fRec77[1] + fSlow128 * self.fRec143[0]) + fSlow127 * self.fRec142[1];
			self.fVec35[(self.IOTA0 & 65535) as usize] = 0.35355338 * self.fRec142[0] + 1e-20;
			let mut fTemp91: F32 = self.fRec89[0] + self.fRec90[0];
			self.fVec36[(self.IOTA0 & 4095) as usize] = fTemp91;
			let mut fTemp92: F32 = 0.111 * self.fVec36[((i32::wrapping_sub(self.IOTA0, self.iConst143)) & 4095) as usize];
			let mut fTemp93: F32 = fTemp92 + 0.6 * self.fRec140[1] + self.fVec35[((i32::wrapping_sub(self.IOTA0, self.iConst126)) & 65535) as usize];
			self.fVec37[(self.IOTA0 & 8191) as usize] = fTemp93;
			self.fRec140[0] = self.fVec37[((i32::wrapping_sub(self.IOTA0, self.iConst127)) & 8191) as usize];
			let mut fRec141: F32 = -(0.6 * fTemp93);
			self.fRec147[0] = -(self.fConst104 * (self.fConst103 * self.fRec147[1] - (self.fRec81[1] + self.fRec81[2])));
			self.fRec146[0] = fSlow138 * (self.fRec81[1] + fSlow137 * self.fRec147[0]) + fSlow136 * self.fRec146[1];
			self.fVec38[(self.IOTA0 & 65535) as usize] = 0.35355338 * self.fRec146[0] + 1e-20;
			let mut fTemp94: F32 = self.fVec38[((i32::wrapping_sub(self.IOTA0, self.iConst131)) & 65535) as usize] + fTemp92 + 0.6 * self.fRec144[1];
			self.fVec39[(self.IOTA0 & 8191) as usize] = fTemp94;
			self.fRec144[0] = self.fVec39[((i32::wrapping_sub(self.IOTA0, self.iConst132)) & 8191) as usize];
			let mut fRec145: F32 = -(0.6 * fTemp94);
			self.fRec151[0] = -(self.fConst104 * (self.fConst103 * self.fRec151[1] - (self.fRec79[1] + self.fRec79[2])));
			self.fRec150[0] = fSlow147 * (self.fRec79[1] + fSlow146 * self.fRec151[0]) + fSlow145 * self.fRec150[1];
			self.fVec40[(self.IOTA0 & 65535) as usize] = 0.35355338 * self.fRec150[0] + 1e-20;
			let mut fTemp95: F32 = 0.6 * self.fRec148[1] + self.fVec40[((i32::wrapping_sub(self.IOTA0, self.iConst136)) & 65535) as usize];
			self.fVec41[(self.IOTA0 & 8191) as usize] = fTemp95 - fTemp92;
			self.fRec148[0] = self.fVec41[((i32::wrapping_sub(self.IOTA0, self.iConst137)) & 8191) as usize];
			let mut fRec149: F32 = 0.6 * (fTemp92 - fTemp95);
			self.fRec155[0] = -(self.fConst104 * (self.fConst103 * self.fRec155[1] - (self.fRec83[1] + self.fRec83[2])));
			self.fRec154[0] = fSlow156 * (self.fRec83[1] + fSlow155 * self.fRec155[0]) + fSlow154 * self.fRec154[1];
			self.fVec42[(self.IOTA0 & 65535) as usize] = 0.35355338 * self.fRec154[0] + 1e-20;
			let mut fTemp96: F32 = 0.6 * self.fRec152[1] + self.fVec42[((i32::wrapping_sub(self.IOTA0, self.iConst141)) & 65535) as usize];
			self.fVec43[(self.IOTA0 & 4095) as usize] = fTemp96 - fTemp92;
			self.fRec152[0] = self.fVec43[((i32::wrapping_sub(self.IOTA0, self.iConst142)) & 4095) as usize];
			let mut fRec153: F32 = 0.6 * (fTemp92 - fTemp96);
			self.fRec76[0] = self.fRec152[1] + self.fRec148[1] + self.fRec144[1] + self.fRec140[1] + self.fRec136[1] + self.fRec132[1] + self.fRec84[1] + self.fRec128[1] + fRec153 + fRec149 + fRec145 + fRec141 + fTemp90;
			self.fRec77[0] = self.fRec136[1] + self.fRec132[1] + self.fRec84[1] + self.fRec128[1] + fTemp90 - (self.fRec152[1] + self.fRec148[1] + self.fRec144[1] + self.fRec140[1] + fRec153 + fRec149 + fRec141 + fRec145);
			let mut fTemp97: F32 = fRec133 + fRec137;
			self.fRec78[0] = self.fRec144[1] + self.fRec140[1] + self.fRec84[1] + self.fRec128[1] + fRec145 + fRec141 + fTemp87 - (self.fRec152[1] + self.fRec148[1] + self.fRec136[1] + self.fRec132[1] + fRec153 + fRec149 + fTemp97);
			self.fRec79[0] = self.fRec152[1] + self.fRec148[1] + self.fRec84[1] + self.fRec128[1] + fRec153 + fRec149 + fTemp87 - (self.fRec144[1] + self.fRec140[1] + self.fRec136[1] + self.fRec132[1] + fRec145 + fRec141 + fTemp97);
			let mut fTemp98: F32 = fRec85 + fRec137;
			let mut fTemp99: F32 = fRec129 + fRec133;
			self.fRec80[0] = self.fRec148[1] + self.fRec140[1] + self.fRec132[1] + self.fRec128[1] + fRec149 + fRec141 + fTemp99 - (self.fRec152[1] + self.fRec144[1] + self.fRec136[1] + self.fRec84[1] + fRec153 + fRec145 + fTemp98);
			self.fRec81[0] = self.fRec152[1] + self.fRec144[1] + self.fRec132[1] + self.fRec128[1] + fRec153 + fRec145 + fTemp99 - (self.fRec148[1] + self.fRec140[1] + self.fRec136[1] + self.fRec84[1] + fRec149 + fRec141 + fTemp98);
			let mut fTemp100: F32 = fRec85 + fRec133;
			let mut fTemp101: F32 = fRec129 + fRec137;
			self.fRec82[0] = self.fRec152[1] + self.fRec140[1] + self.fRec136[1] + self.fRec128[1] + fRec153 + fRec141 + fTemp101 - (self.fRec148[1] + self.fRec144[1] + self.fRec132[1] + self.fRec84[1] + fRec149 + fRec145 + fTemp100);
			self.fRec83[0] = self.fRec148[1] + self.fRec144[1] + self.fRec136[1] + self.fRec128[1] + fRec149 + fRec145 + fTemp101 - (self.fRec152[1] + self.fRec140[1] + self.fRec132[1] + self.fRec84[1] + fRec153 + fRec141 + fTemp100);
			let mut fTemp102: F32 = self.fRec77[0] + self.fRec78[0];
			self.fVec44[0] = fTemp102;
			self.fRec75[0] = fSlow158 * (fSlow157 * (fTemp102 - self.fVec44[1]) - fSlow8 * self.fRec75[1]);
			let mut fTemp103: F32 = 0.37 * fTemp83 + fSlow159 * self.fRec75[0];
			self.fVec45[(self.IOTA0 & 32767) as usize] = fTemp103;
			let mut fTemp104: F32 = if iTemp18 != 0 {0.0} else {fSlow160 + self.fRec157[1]};
			self.fRec157[0] = fTemp104 - F32::floor(fTemp104);
			let mut fTemp105: F32 = F32::min(1.0, fSlow161 * self.fRec157[0]) + 0.75;
			let mut fTemp106: F32 = 6.2831855 * (fTemp105 - F32::floor(0.15915494 * F32::min(6.2831855 * fTemp105, 0.0)));
			let mut fTemp107: F32 = 795.61554 * if (fTemp106 < 0.0) as i32 != 0 {0.0} else {fTemp106 % 6.2831855};
			let mut iTemp108: i32 = (fTemp107) as i32;
			let mut fTemp109: F32 = unsafe { ftbl1DSP_OutputSIG1[iTemp108 as usize] };
			self.fRec158[0] = fSlow162 * fTemp1 + self.fConst1 * fTemp0 * self.fRec158[1];
			let mut fTemp110: F32 = self.fConst0 * self.fRec158[0] * F32::max(0.0, 0.5 * (fTemp109 + (fTemp107 - (iTemp108) as F32) * (unsafe { ftbl1DSP_OutputSIG1[(i32::wrapping_add(iTemp108, 1)) as usize] } - fTemp109) + 1.0));
			let mut iTemp111: i32 = (fTemp110) as i32;
			let mut iTemp112: i32 = (F32::min(self.fConst144, (std::cmp::max(0, i32::wrapping_add(iTemp111, 1))) as F32)) as i32;
			let mut fTemp113: F32 = F32::floor(fTemp110);
			let mut fTemp114: F32 = fTemp110 - fTemp113;
			let mut fTemp115: F32 = fTemp113 + (1.0 - fTemp110);
			let mut iTemp116: i32 = (F32::min(self.fConst144, (std::cmp::max(0, iTemp111)) as F32)) as i32;
			let mut fTemp117: F32 = self.fVec45[((i32::wrapping_sub(self.IOTA0, iTemp116)) & 32767) as usize] * fTemp115 + fTemp114 * self.fVec45[((i32::wrapping_sub(self.IOTA0, iTemp112)) & 32767) as usize];
			let mut fTemp118: F32 = F32::abs(if iSlow5 != 0 {1.0} else {fTemp117});
			self.fRec74[0] = F32::max(fTemp118, self.fConst146 * self.fRec74[1] + self.fConst147 * fTemp118);
			self.fRec73[0] = self.fConst148 * self.fRec74[0] + self.fConst97 * self.fRec73[1];
			let mut fTemp119: F32 = self.fRec77[0] - self.fRec78[0];
			self.fVec46[0] = fTemp119;
			self.fRec161[0] = fSlow158 * (fSlow157 * (fTemp119 - self.fVec46[1]) - fSlow8 * self.fRec161[1]);
			let mut fTemp120: F32 = 0.37 * fTemp91 + fSlow159 * self.fRec161[0];
			self.fVec47[(self.IOTA0 & 32767) as usize] = fTemp120;
			let mut fTemp121: F32 = fTemp115 * self.fVec47[((i32::wrapping_sub(self.IOTA0, iTemp116)) & 32767) as usize] + fTemp114 * self.fVec47[((i32::wrapping_sub(self.IOTA0, iTemp112)) & 32767) as usize];
			let mut fTemp122: F32 = F32::abs(if iSlow5 != 0 {1.0} else {fTemp121});
			self.fRec160[0] = F32::max(fTemp122, self.fConst146 * self.fRec160[1] + self.fConst147 * fTemp122);
			self.fRec159[0] = self.fConst148 * self.fRec160[0] + self.fConst97 * self.fRec159[1];
			let mut fTemp123: F32 = F32::min(1.0, 1e+04 * F32::max(self.fRec73[0], self.fRec159[0]));
			self.fRec5[0] = fTemp121 + fSlow163 * fTemp123 * (fTemp64 + fSlow4 * (self.fConst96 * (self.fRec54[2] + self.fRec54[0] + 2.0 * self.fRec54[1]) - fTemp64)) - (fTemp17 * self.fRec5[2] + 2.0 * fTemp16 * self.fRec5[1]) / fTemp13;
			self.fRec3[0] = (self.fRec5[2] + (self.fRec5[0] - 2.0 * self.fRec5[1])) / fTemp15 - (fTemp9 * self.fRec3[2] + 2.0 * fTemp6 * self.fRec3[1]) / fTemp8;
			self.fRec2[0] = (self.fRec3[2] + self.fRec3[0] + 2.0 * self.fRec3[1]) / fTemp8 - (fTemp7 * self.fRec2[2] + 2.0 * fTemp6 * self.fRec2[1]) / fTemp5;
			let mut fTemp124: F32 = (self.fRec2[2] + self.fRec2[0] + 2.0 * self.fRec2[1]) / fTemp5;
			let mut fTemp125: F32 = F32::abs(fTemp124);
			let mut fTemp126: F32 = fTemp125 * (1.0 - (i32::wrapping_mul(2, (fTemp124 < 0.0) as i32)) as F32) / (fSlow165 * (1.0 - fTemp125) + 1.0);
			let mut fTemp127: F32 = fSlow172 * F32::round(fSlow171 * fTemp126) + fSlow169 * fTemp126;
			let mut iTemp128: i32 = ((iTemp30 % iSlow177) == 0) as i32;
			self.fRec164[0] = if iTemp128 != 0 {fTemp127} else {self.fRec164[1]};
			let mut fTemp129: F32 = if (self.fRec164[0] != self.fRec164[1]) as i32 != 0 {fSlow176} else {self.fRec162[1] + -1.0};
			self.fRec162[0] = fTemp129;
			self.fRec163[0] = if (fTemp129 > 0.0) as i32 != 0 {self.fRec163[1] + (self.fRec164[0] - self.fRec163[1]) / fTemp129} else {self.fRec164[0]};
			self.fRec165[0] = fSlow178 * fTemp1 + self.fConst1 * fTemp0 * self.fRec165[1];
			let mut fTemp130: F32 = F32::min(1.0, self.fRec165[0] + 1.0) * (fSlow174 * self.fRec163[0] + fSlow175 * fTemp127);
			let mut fTemp131: F32 = self.fConst37 * self.fRec181[1];
			self.fRec186[0] = if iTemp31 != 0 {4.656613e-10 * (iRec41) as F32} else {self.fRec186[1]};
			let mut fTemp132: F32 = if (self.fRec186[0] != self.fRec186[1]) as i32 != 0 {self.fConst55} else {self.fRec184[1] + -1.0};
			self.fRec184[0] = fTemp132;
			self.fRec185[0] = if (fTemp132 > 0.0) as i32 != 0 {self.fRec185[1] + (self.fRec186[0] - self.fRec185[1]) / fTemp132} else {self.fRec186[0]};
			self.fRec183[0] = self.fRec185[0] * fTemp43 - self.fConst51 * (self.fConst49 * self.fRec183[1] + self.fConst48 * self.fRec183[2]);
			self.fRec187[0] = self.fRec185[0] * fTemp57 - self.fConst51 * (self.fConst49 * self.fRec187[1] + self.fConst48 * self.fRec187[2]);
			self.fRec190[0] = if iTemp31 != 0 {4.656613e-10 * (self.iRec39[0]) as F32} else {self.fRec190[1]};
			let mut fTemp133: F32 = if (self.fRec190[0] != self.fRec190[1]) as i32 != 0 {self.fConst55} else {self.fRec188[1] + -1.0};
			self.fRec188[0] = fTemp133;
			self.fRec189[0] = if (fTemp133 > 0.0) as i32 != 0 {self.fRec189[1] + (self.fRec190[0] - self.fRec189[1]) / fTemp133} else {self.fRec190[0]};
			let mut fTemp134: F32 = fTemp22 * (self.fRec189[0] + self.fConst62 * (3e+01 * (self.fRec187[0] - self.fRec187[2]) + 2e+01 * (self.fRec183[0] - self.fRec183[2])));
			self.fVec48[0] = fTemp134;
			self.fRec182[0] = self.fConst63 * (0.0891251 * (fTemp134 + self.fVec48[1]) - self.fConst40 * self.fRec182[1]);
			self.fRec191[0] = self.fConst63 * (self.fConst64 * (fTemp134 - self.fVec48[1]) - self.fConst40 * self.fRec191[1]);
			let mut fTemp135: F32 = self.fRec191[0] + 7.0794578 * self.fRec182[0];
			self.fVec49[0] = fTemp135;
			self.fRec192[0] = -(self.fConst67 * (self.fConst66 * self.fRec192[1] - (fTemp135 + self.fVec49[1])));
			self.fRec181[0] = 0.75 * self.fRec192[0] + 0.25 * fTemp135 - (self.fRec181[2] * fTemp29 + fTemp131) / fTemp27;
			self.fRec180[0] = (fTemp131 + self.fRec181[0] * fTemp63 + self.fRec181[2] * fTemp62) / fTemp27 - self.fConst30 * (self.fConst29 * self.fRec180[2] + self.fConst5 * self.fRec180[1]);
			self.fRec179[0] = self.fConst30 * (self.fRec180[2] + self.fRec180[0] + 2.0 * self.fRec180[1]) - self.fConst28 * (self.fConst27 * self.fRec179[2] + self.fConst5 * self.fRec179[1]);
			self.fRec178[0] = self.fConst28 * (self.fRec179[2] + self.fRec179[0] + 2.0 * self.fRec179[1]) - self.fConst26 * (self.fConst25 * self.fRec178[2] + self.fConst5 * self.fRec178[1]);
			self.fRec177[0] = self.fConst26 * (self.fRec178[2] + self.fRec178[0] + 2.0 * self.fRec178[1]) - self.fConst24 * (self.fConst23 * self.fRec177[2] + self.fConst5 * self.fRec177[1]);
			self.fRec176[0] = self.fConst24 * (self.fRec177[2] + self.fRec177[0] + 2.0 * self.fRec177[1]) - self.fConst22 * (self.fConst21 * self.fRec176[2] + self.fConst5 * self.fRec176[1]);
			self.fRec175[0] = self.fConst22 * (self.fRec176[2] + self.fRec176[0] + 2.0 * self.fRec176[1]) - self.fConst20 * (self.fConst19 * self.fRec175[2] + self.fConst5 * self.fRec175[1]);
			self.fRec174[0] = self.fConst20 * (self.fRec175[2] + self.fRec175[0] + 2.0 * self.fRec175[1]) - self.fConst18 * (self.fConst17 * self.fRec174[2] + self.fConst5 * self.fRec174[1]);
			self.fRec173[0] = self.fConst18 * (self.fRec174[2] + self.fRec174[0] + 2.0 * self.fRec174[1]) - self.fConst16 * (self.fConst15 * self.fRec173[2] + self.fConst5 * self.fRec173[1]);
			self.fRec172[0] = self.fConst16 * (self.fRec173[2] + self.fRec173[0] + 2.0 * self.fRec173[1]) - self.fConst14 * (self.fConst13 * self.fRec172[2] + self.fConst5 * self.fRec172[1]);
			self.fRec171[0] = self.fConst14 * (self.fRec172[2] + self.fRec172[0] + 2.0 * self.fRec172[1]) - self.fConst12 * (self.fConst11 * self.fRec171[2] + self.fConst5 * self.fRec171[1]);
			self.fRec170[0] = self.fConst12 * (self.fRec171[2] + self.fRec171[0] + 2.0 * self.fRec171[1]) - self.fConst10 * (self.fConst9 * self.fRec170[2] + self.fConst5 * self.fRec170[1]);
			self.fRec169[0] = self.fConst10 * (self.fRec170[2] + self.fRec170[0] + 2.0 * self.fRec170[1]) - self.fConst8 * (self.fConst7 * self.fRec169[2] + self.fConst5 * self.fRec169[1]);
			let mut fTemp136: F32 = self.fConst8 * (self.fRec169[2] + self.fRec169[0] + 2.0 * self.fRec169[1]);
			self.fRec208[0] = if iTemp31 != 0 {4.656613e-10 * (self.iRec70[0]) as F32} else {self.fRec208[1]};
			let mut fTemp137: F32 = if (self.fRec208[0] != self.fRec208[1]) as i32 != 0 {self.fConst55} else {self.fRec206[1] + -1.0};
			self.fRec206[0] = fTemp137;
			self.fRec207[0] = if (fTemp137 > 0.0) as i32 != 0 {self.fRec207[1] + (self.fRec208[0] - self.fRec207[1]) / fTemp137} else {self.fRec208[0]};
			self.fRec205[0] = -(self.fConst63 * (self.fConst40 * self.fRec205[1] - (self.fRec207[0] + self.fRec207[1])));
			self.fRec209[0] = -(self.fConst63 * (self.fConst40 * self.fRec209[1] - self.fConst39 * (self.fRec207[0] - self.fRec207[1])));
			self.fRec204[0] = self.fRec209[0] + 7.0794578 * self.fRec205[0] - self.fConst95 * (self.fConst94 * self.fRec204[2] + self.fConst69 * self.fRec204[1]);
			self.fRec203[0] = self.fConst95 * (self.fRec204[2] + self.fRec204[0] + 2.0 * self.fRec204[1]) - self.fConst93 * (self.fConst92 * self.fRec203[2] + self.fConst69 * self.fRec203[1]);
			self.fRec202[0] = self.fConst93 * (self.fRec203[2] + self.fRec203[0] + 2.0 * self.fRec203[1]) - self.fConst91 * (self.fConst90 * self.fRec202[2] + self.fConst69 * self.fRec202[1]);
			self.fRec201[0] = self.fConst91 * (self.fRec202[2] + self.fRec202[0] + 2.0 * self.fRec202[1]) - self.fConst89 * (self.fConst88 * self.fRec201[2] + self.fConst69 * self.fRec201[1]);
			self.fRec200[0] = self.fConst89 * (self.fRec201[2] + self.fRec201[0] + 2.0 * self.fRec201[1]) - self.fConst87 * (self.fConst86 * self.fRec200[2] + self.fConst69 * self.fRec200[1]);
			self.fRec199[0] = self.fConst87 * (self.fRec200[2] + self.fRec200[0] + 2.0 * self.fRec200[1]) - self.fConst85 * (self.fConst84 * self.fRec199[2] + self.fConst69 * self.fRec199[1]);
			self.fRec198[0] = self.fConst85 * (self.fRec199[2] + self.fRec199[0] + 2.0 * self.fRec199[1]) - self.fConst83 * (self.fConst82 * self.fRec198[2] + self.fConst69 * self.fRec198[1]);
			self.fRec197[0] = self.fConst83 * (self.fRec198[2] + self.fRec198[0] + 2.0 * self.fRec198[1]) - self.fConst81 * (self.fConst80 * self.fRec197[2] + self.fConst69 * self.fRec197[1]);
			self.fRec196[0] = self.fConst81 * (self.fRec197[2] + self.fRec197[0] + 2.0 * self.fRec197[1]) - self.fConst79 * (self.fConst78 * self.fRec196[2] + self.fConst69 * self.fRec196[1]);
			self.fRec195[0] = self.fConst79 * (self.fRec196[2] + self.fRec196[0] + 2.0 * self.fRec196[1]) - self.fConst77 * (self.fConst76 * self.fRec195[2] + self.fConst69 * self.fRec195[1]);
			self.fRec194[0] = self.fConst77 * (self.fRec195[2] + self.fRec195[0] + 2.0 * self.fRec195[1]) - self.fConst75 * (self.fConst74 * self.fRec194[2] + self.fConst69 * self.fRec194[1]);
			self.fRec193[0] = self.fConst75 * (self.fRec194[2] + self.fRec194[0] + 2.0 * self.fRec194[1]) - self.fConst73 * (self.fConst71 * self.fRec193[2] + self.fConst69 * self.fRec193[1]);
			self.fRec168[0] = fTemp117 + fSlow163 * (fTemp136 + fSlow4 * (self.fConst96 * (self.fRec193[2] + self.fRec193[0] + 2.0 * self.fRec193[1]) - fTemp136)) * fTemp123 - (self.fRec168[2] * fTemp17 + 2.0 * self.fRec168[1] * fTemp16) / fTemp13;
			self.fRec167[0] = (self.fRec168[2] + (self.fRec168[0] - 2.0 * self.fRec168[1])) / fTemp15 - (self.fRec167[2] * fTemp9 + 2.0 * self.fRec167[1] * fTemp6) / fTemp8;
			self.fRec166[0] = (self.fRec167[2] + self.fRec167[0] + 2.0 * self.fRec167[1]) / fTemp8 - (self.fRec166[2] * fTemp7 + 2.0 * fTemp6 * self.fRec166[1]) / fTemp5;
			let mut fTemp138: F32 = (self.fRec166[2] + self.fRec166[0] + 2.0 * self.fRec166[1]) / fTemp5;
			let mut fTemp139: F32 = F32::abs(fTemp138);
			let mut fTemp140: F32 = fTemp139 * (1.0 - (i32::wrapping_mul(2, (fTemp138 < 0.0) as i32)) as F32) / (fSlow165 * (1.0 - fTemp139) + 1.0);
			let mut fTemp141: F32 = fSlow172 * F32::round(fSlow171 * fTemp140) + fSlow169 * fTemp140;
			self.fRec212[0] = if iTemp128 != 0 {fTemp141} else {self.fRec212[1]};
			let mut fTemp142: F32 = if (self.fRec212[0] != self.fRec212[1]) as i32 != 0 {fSlow176} else {self.fRec210[1] + -1.0};
			self.fRec210[0] = fTemp142;
			self.fRec211[0] = if (fTemp142 > 0.0) as i32 != 0 {self.fRec211[1] + (self.fRec212[0] - self.fRec211[1]) / fTemp142} else {self.fRec212[0]};
			let mut fTemp143: F32 = F32::min(1.0, 1.0 - self.fRec165[0]) * (fSlow174 * self.fRec211[0] + fSlow175 * fTemp141);
			let mut fTemp144: F32 = fSlow180 * (fTemp143 - fTemp130);
			self.fRec213[0] = fSlow181 * fTemp1 + self.fConst1 * fTemp0 * self.fRec213[1];
			let mut fTemp145: F32 = F32::powf(1e+01, 0.05 * self.fRec213[0]);
			*output0 = fSlow182 * fTemp145 * (fTemp143 - fTemp144);
			*output1 = fSlow182 * fTemp145 * (fTemp130 + fTemp144);
			self.iVec0[1] = self.iVec0[0];
			self.fVec1[1] = self.fVec1[0];
			self.fRec0[1] = self.fRec0[0];
			self.fRec1[1] = self.fRec1[0];
			self.fRec4[1] = self.fRec4[0];
			self.fRec19[1] = self.fRec19[0];
			self.fRec20[1] = self.fRec20[0];
			self.fRec21[1] = self.fRec21[0];
			self.iRec26[1] = self.iRec26[0];
			self.iRec30[1] = self.iRec30[0];
			self.fVec3[1] = self.fVec3[0];
			self.fRec29[1] = self.fRec29[0];
			self.fRec27[1] = self.fRec27[0];
			self.fRec28[1] = self.fRec28[0];
			self.fRec25[1] = self.fRec25[0];
			self.fRec34[1] = self.fRec34[0];
			self.fVec4[1] = self.fVec4[0];
			self.fRec33[1] = self.fRec33[0];
			self.fRec32[1] = self.fRec32[0];
			self.fRec35[1] = self.fRec35[0];
			self.iVec5[1] = self.iVec5[0];
			self.iRec31[1] = self.iRec31[0];
			self.iRec39[1] = self.iRec39[0];
			self.fRec38[1] = self.fRec38[0];
			self.fRec36[1] = self.fRec36[0];
			self.fRec37[1] = self.fRec37[0];
			self.fRec24[2] = self.fRec24[1];
			self.fRec24[1] = self.fRec24[0];
			self.fRec47[1] = self.fRec47[0];
			self.fVec6[1] = self.fVec6[0];
			self.fRec46[1] = self.fRec46[0];
			self.fRec45[1] = self.fRec45[0];
			self.fRec48[1] = self.fRec48[0];
			self.iVec7[1] = self.iVec7[0];
			self.iRec44[1] = self.iRec44[0];
			self.fRec43[2] = self.fRec43[1];
			self.fRec43[1] = self.fRec43[0];
			self.fRec51[1] = self.fRec51[0];
			self.fRec49[1] = self.fRec49[0];
			self.fRec50[1] = self.fRec50[0];
			self.fVec8[1] = self.fVec8[0];
			self.fRec23[1] = self.fRec23[0];
			self.fRec52[1] = self.fRec52[0];
			self.fVec9[1] = self.fVec9[0];
			self.fRec53[1] = self.fRec53[0];
			self.fRec22[2] = self.fRec22[1];
			self.fRec22[1] = self.fRec22[0];
			self.fRec17[2] = self.fRec17[1];
			self.fRec17[1] = self.fRec17[0];
			self.fRec16[2] = self.fRec16[1];
			self.fRec16[1] = self.fRec16[0];
			self.fRec15[2] = self.fRec15[1];
			self.fRec15[1] = self.fRec15[0];
			self.fRec14[2] = self.fRec14[1];
			self.fRec14[1] = self.fRec14[0];
			self.fRec13[2] = self.fRec13[1];
			self.fRec13[1] = self.fRec13[0];
			self.fRec12[2] = self.fRec12[1];
			self.fRec12[1] = self.fRec12[0];
			self.fRec11[2] = self.fRec11[1];
			self.fRec11[1] = self.fRec11[0];
			self.fRec10[2] = self.fRec10[1];
			self.fRec10[1] = self.fRec10[0];
			self.fRec9[2] = self.fRec9[1];
			self.fRec9[1] = self.fRec9[0];
			self.fRec8[2] = self.fRec8[1];
			self.fRec8[1] = self.fRec8[0];
			self.fRec7[2] = self.fRec7[1];
			self.fRec7[1] = self.fRec7[0];
			self.fRec6[2] = self.fRec6[1];
			self.fRec6[1] = self.fRec6[0];
			self.iRec70[1] = self.iRec70[0];
			self.fRec69[1] = self.fRec69[0];
			self.fRec67[1] = self.fRec67[0];
			self.fRec68[1] = self.fRec68[0];
			self.fRec66[1] = self.fRec66[0];
			self.fRec72[1] = self.fRec72[0];
			self.fRec65[2] = self.fRec65[1];
			self.fRec65[1] = self.fRec65[0];
			self.fRec64[2] = self.fRec64[1];
			self.fRec64[1] = self.fRec64[0];
			self.fRec63[2] = self.fRec63[1];
			self.fRec63[1] = self.fRec63[0];
			self.fRec62[2] = self.fRec62[1];
			self.fRec62[1] = self.fRec62[0];
			self.fRec61[2] = self.fRec61[1];
			self.fRec61[1] = self.fRec61[0];
			self.fRec60[2] = self.fRec60[1];
			self.fRec60[1] = self.fRec60[0];
			self.fRec59[2] = self.fRec59[1];
			self.fRec59[1] = self.fRec59[0];
			self.fRec58[2] = self.fRec58[1];
			self.fRec58[1] = self.fRec58[0];
			self.fRec57[2] = self.fRec57[1];
			self.fRec57[1] = self.fRec57[0];
			self.fRec56[2] = self.fRec56[1];
			self.fRec56[1] = self.fRec56[0];
			self.fRec55[2] = self.fRec55[1];
			self.fRec55[1] = self.fRec55[0];
			self.fRec54[2] = self.fRec54[1];
			self.fRec54[1] = self.fRec54[0];
			self.fRec87[1] = self.fRec87[0];
			self.fRec86[1] = self.fRec86[0];
			self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
			self.fRec99[1] = self.fRec99[0];
			self.fRec98[1] = self.fRec98[0];
			self.fRec96[1] = self.fRec96[0];
			self.fRec103[1] = self.fRec103[0];
			self.fRec102[1] = self.fRec102[0];
			self.fRec100[1] = self.fRec100[0];
			self.fRec107[1] = self.fRec107[0];
			self.fRec106[1] = self.fRec106[0];
			self.fRec104[1] = self.fRec104[0];
			self.fRec111[1] = self.fRec111[0];
			self.fRec110[1] = self.fRec110[0];
			self.fRec108[1] = self.fRec108[0];
			self.fRec115[1] = self.fRec115[0];
			self.fRec114[1] = self.fRec114[0];
			self.fRec112[1] = self.fRec112[0];
			self.fRec119[1] = self.fRec119[0];
			self.fRec118[1] = self.fRec118[0];
			self.fRec116[1] = self.fRec116[0];
			self.fRec123[1] = self.fRec123[0];
			self.fRec122[1] = self.fRec122[0];
			self.fRec120[1] = self.fRec120[0];
			self.fRec127[1] = self.fRec127[0];
			self.fRec126[1] = self.fRec126[0];
			self.fRec124[1] = self.fRec124[0];
			self.fRec88[2] = self.fRec88[1];
			self.fRec88[1] = self.fRec88[0];
			self.fRec89[2] = self.fRec89[1];
			self.fRec89[1] = self.fRec89[0];
			self.fRec90[2] = self.fRec90[1];
			self.fRec90[1] = self.fRec90[0];
			self.fRec91[2] = self.fRec91[1];
			self.fRec91[1] = self.fRec91[0];
			self.fRec92[2] = self.fRec92[1];
			self.fRec92[1] = self.fRec92[0];
			self.fRec93[2] = self.fRec93[1];
			self.fRec93[1] = self.fRec93[0];
			self.fRec94[2] = self.fRec94[1];
			self.fRec94[1] = self.fRec94[0];
			self.fRec95[2] = self.fRec95[1];
			self.fRec95[1] = self.fRec95[0];
			self.fRec84[1] = self.fRec84[0];
			self.fRec131[1] = self.fRec131[0];
			self.fRec130[1] = self.fRec130[0];
			self.fRec128[1] = self.fRec128[0];
			self.fRec135[1] = self.fRec135[0];
			self.fRec134[1] = self.fRec134[0];
			self.fRec132[1] = self.fRec132[0];
			self.fRec139[1] = self.fRec139[0];
			self.fRec138[1] = self.fRec138[0];
			self.fRec136[1] = self.fRec136[0];
			self.fRec143[1] = self.fRec143[0];
			self.fRec142[1] = self.fRec142[0];
			self.fRec140[1] = self.fRec140[0];
			self.fRec147[1] = self.fRec147[0];
			self.fRec146[1] = self.fRec146[0];
			self.fRec144[1] = self.fRec144[0];
			self.fRec151[1] = self.fRec151[0];
			self.fRec150[1] = self.fRec150[0];
			self.fRec148[1] = self.fRec148[0];
			self.fRec155[1] = self.fRec155[0];
			self.fRec154[1] = self.fRec154[0];
			self.fRec152[1] = self.fRec152[0];
			self.fRec76[2] = self.fRec76[1];
			self.fRec76[1] = self.fRec76[0];
			self.fRec77[2] = self.fRec77[1];
			self.fRec77[1] = self.fRec77[0];
			self.fRec78[2] = self.fRec78[1];
			self.fRec78[1] = self.fRec78[0];
			self.fRec79[2] = self.fRec79[1];
			self.fRec79[1] = self.fRec79[0];
			self.fRec80[2] = self.fRec80[1];
			self.fRec80[1] = self.fRec80[0];
			self.fRec81[2] = self.fRec81[1];
			self.fRec81[1] = self.fRec81[0];
			self.fRec82[2] = self.fRec82[1];
			self.fRec82[1] = self.fRec82[0];
			self.fRec83[2] = self.fRec83[1];
			self.fRec83[1] = self.fRec83[0];
			self.fVec44[1] = self.fVec44[0];
			self.fRec75[1] = self.fRec75[0];
			self.fRec157[1] = self.fRec157[0];
			self.fRec158[1] = self.fRec158[0];
			self.fRec74[1] = self.fRec74[0];
			self.fRec73[1] = self.fRec73[0];
			self.fVec46[1] = self.fVec46[0];
			self.fRec161[1] = self.fRec161[0];
			self.fRec160[1] = self.fRec160[0];
			self.fRec159[1] = self.fRec159[0];
			self.fRec5[2] = self.fRec5[1];
			self.fRec5[1] = self.fRec5[0];
			self.fRec3[2] = self.fRec3[1];
			self.fRec3[1] = self.fRec3[0];
			self.fRec2[2] = self.fRec2[1];
			self.fRec2[1] = self.fRec2[0];
			self.fRec164[1] = self.fRec164[0];
			self.fRec162[1] = self.fRec162[0];
			self.fRec163[1] = self.fRec163[0];
			self.fRec165[1] = self.fRec165[0];
			self.fRec186[1] = self.fRec186[0];
			self.fRec184[1] = self.fRec184[0];
			self.fRec185[1] = self.fRec185[0];
			self.fRec183[2] = self.fRec183[1];
			self.fRec183[1] = self.fRec183[0];
			self.fRec187[2] = self.fRec187[1];
			self.fRec187[1] = self.fRec187[0];
			self.fRec190[1] = self.fRec190[0];
			self.fRec188[1] = self.fRec188[0];
			self.fRec189[1] = self.fRec189[0];
			self.fVec48[1] = self.fVec48[0];
			self.fRec182[1] = self.fRec182[0];
			self.fRec191[1] = self.fRec191[0];
			self.fVec49[1] = self.fVec49[0];
			self.fRec192[1] = self.fRec192[0];
			self.fRec181[2] = self.fRec181[1];
			self.fRec181[1] = self.fRec181[0];
			self.fRec180[2] = self.fRec180[1];
			self.fRec180[1] = self.fRec180[0];
			self.fRec179[2] = self.fRec179[1];
			self.fRec179[1] = self.fRec179[0];
			self.fRec178[2] = self.fRec178[1];
			self.fRec178[1] = self.fRec178[0];
			self.fRec177[2] = self.fRec177[1];
			self.fRec177[1] = self.fRec177[0];
			self.fRec176[2] = self.fRec176[1];
			self.fRec176[1] = self.fRec176[0];
			self.fRec175[2] = self.fRec175[1];
			self.fRec175[1] = self.fRec175[0];
			self.fRec174[2] = self.fRec174[1];
			self.fRec174[1] = self.fRec174[0];
			self.fRec173[2] = self.fRec173[1];
			self.fRec173[1] = self.fRec173[0];
			self.fRec172[2] = self.fRec172[1];
			self.fRec172[1] = self.fRec172[0];
			self.fRec171[2] = self.fRec171[1];
			self.fRec171[1] = self.fRec171[0];
			self.fRec170[2] = self.fRec170[1];
			self.fRec170[1] = self.fRec170[0];
			self.fRec169[2] = self.fRec169[1];
			self.fRec169[1] = self.fRec169[0];
			self.fRec208[1] = self.fRec208[0];
			self.fRec206[1] = self.fRec206[0];
			self.fRec207[1] = self.fRec207[0];
			self.fRec205[1] = self.fRec205[0];
			self.fRec209[1] = self.fRec209[0];
			self.fRec204[2] = self.fRec204[1];
			self.fRec204[1] = self.fRec204[0];
			self.fRec203[2] = self.fRec203[1];
			self.fRec203[1] = self.fRec203[0];
			self.fRec202[2] = self.fRec202[1];
			self.fRec202[1] = self.fRec202[0];
			self.fRec201[2] = self.fRec201[1];
			self.fRec201[1] = self.fRec201[0];
			self.fRec200[2] = self.fRec200[1];
			self.fRec200[1] = self.fRec200[0];
			self.fRec199[2] = self.fRec199[1];
			self.fRec199[1] = self.fRec199[0];
			self.fRec198[2] = self.fRec198[1];
			self.fRec198[1] = self.fRec198[0];
			self.fRec197[2] = self.fRec197[1];
			self.fRec197[1] = self.fRec197[0];
			self.fRec196[2] = self.fRec196[1];
			self.fRec196[1] = self.fRec196[0];
			self.fRec195[2] = self.fRec195[1];
			self.fRec195[1] = self.fRec195[0];
			self.fRec194[2] = self.fRec194[1];
			self.fRec194[1] = self.fRec194[0];
			self.fRec193[2] = self.fRec193[1];
			self.fRec193[1] = self.fRec193[0];
			self.fRec168[2] = self.fRec168[1];
			self.fRec168[1] = self.fRec168[0];
			self.fRec167[2] = self.fRec167[1];
			self.fRec167[1] = self.fRec167[0];
			self.fRec166[2] = self.fRec166[1];
			self.fRec166[1] = self.fRec166[0];
			self.fRec212[1] = self.fRec212[0];
			self.fRec210[1] = self.fRec210[0];
			self.fRec211[1] = self.fRec211[0];
			self.fRec213[1] = self.fRec213[0];
		}
	}

}

