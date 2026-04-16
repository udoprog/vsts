/* ------------------------------------------------------------
author: "Romain Michon (rmichon@ccrma.stanford.edu)", "Punk Labs LLC"
copyright: "Romain Michon", "2024 Punk Labs LLC"
name: "piano"
version: "1.0"
Code generated with Faust 2.72.14 (https://faust.grame.fr)
Compilation options: -a arch.rs -lang rust -ct 0 -cn DSP_Piano -dtl 65536 -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
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




pub struct DSP_PianoSIG0 {
	iRec1: [i32;2],
}

impl DSP_PianoSIG0 {
	
	fn get_num_inputsDSP_PianoSIG0(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsDSP_PianoSIG0(&self) -> i32 {
		return 1;
	}
	
	fn instance_initDSP_PianoSIG0(&mut self, sample_rate: i32) {
		for l0 in 0..2 {
			self.iRec1[l0 as usize] = 0;
		}
	}
	
	fn fillDSP_PianoSIG0(&mut self, count: i32, table: &mut[F32]) {
		for i1 in 0..count {
			self.iRec1[0] = i32::wrapping_add(self.iRec1[1], 1);
			let mut fTemp0: F32 = 7.090909 * (i32::wrapping_add(self.iRec1[0], -1)) as F32;
			let mut fTemp1: F32 = fTemp0 + 21.0;
			let mut iTemp2: i32 = (fTemp1 < 42.0) as i32;
			table[i1 as usize] = if (fTemp1 < 96.0) as i32 != 0 {if (fTemp1 < 84.0) as i32 != 0 {if (fTemp1 < 72.0) as i32 != 0 {if (fTemp1 < 6e+01) as i32 != 0 {if (fTemp1 < 48.0) as i32 != 0 {if iTemp2 != 0 {if (fTemp1 < 41.0) as i32 != 0 {0.05} else {if iTemp2 != 0 {0.075 * (fTemp0 + -2e+01) + 0.05} else {0.125}}} else {0.125}} else {0.125}} else {0.125}} else {0.125}} else {0.125}} else {0.125};
			self.iRec1[1] = self.iRec1[0];
		}
	}

}


pub fn newDSP_PianoSIG0() -> DSP_PianoSIG0 { 
	DSP_PianoSIG0 {
		iRec1: [0;2],
	}
}

pub struct DSP_PianoSIG1 {
	iRec2: [i32;2],
}

impl DSP_PianoSIG1 {
	
	fn get_num_inputsDSP_PianoSIG1(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsDSP_PianoSIG1(&self) -> i32 {
		return 1;
	}
	
	fn instance_initDSP_PianoSIG1(&mut self, sample_rate: i32) {
		for l1 in 0..2 {
			self.iRec2[l1 as usize] = 0;
		}
	}
	
	fn fillDSP_PianoSIG1(&mut self, count: i32, table: &mut[F32]) {
		for i2 in 0..count {
			self.iRec2[0] = i32::wrapping_add(self.iRec2[1], 1);
			let mut fTemp3: F32 = 1.5644693 * (i32::wrapping_add(self.iRec2[0], -1)) as F32;
			let mut fTemp4: F32 = fTemp3 + 21.0;
			let mut iTemp5: i32 = (fTemp4 < 80.404) as i32;
			let mut iTemp6: i32 = (fTemp4 < 72.862) as i32;
			let mut iTemp7: i32 = (fTemp4 < 69.178) as i32;
			let mut iTemp8: i32 = (fTemp4 < 63.226) as i32;
			let mut iTemp9: i32 = (fTemp4 < 57.558) as i32;
			let mut iTemp10: i32 = (fTemp4 < 48.773) as i32;
			let mut iTemp11: i32 = (fTemp4 < 42.0) as i32;
			let mut iTemp12: i32 = (fTemp4 < 41.0) as i32;
			let mut iTemp13: i32 = (fTemp4 < 35.0) as i32;
			table[i2 as usize] = if iTemp5 != 0 {if iTemp6 != 0 {if iTemp7 != 0 {if iTemp8 != 0 {if iTemp9 != 0 {if iTemp10 != 0 {if iTemp11 != 0 {if iTemp12 != 0 {if iTemp13 != 0 {if (fTemp4 < 28.0) as i32 != 0 {5.0} else {if iTemp13 != 0 {5.0 - 0.0062857145 * (fTemp3 + -7.0)} else {4.956}}} else {if iTemp12 != 0 {0.174 * (fTemp3 + -14.0) + 4.956} else {6.0}}} else {if iTemp11 != 0 {6.0 - 4.0 * (fTemp3 + -2e+01)} else {2.0}}} else {if iTemp10 != 0 {2.0 - 0.13701461 * (fTemp3 + -21.0)} else {1.072}}} else {if iTemp9 != 0 {1.072 - 0.008081958 * (fTemp3 + -27.773)} else {1.001}}} else {if iTemp8 != 0 {0.0082921665 * (fTemp3 + -36.558) + 1.001} else {1.048}}} else {if iTemp7 != 0 {0.012096774 * (fTemp3 + -42.226) + 1.048} else {1.12}}} else {if iTemp6 != 0 {0.10993485 * (fTemp3 + -48.178) + 1.12} else {1.525}}} else {if iTemp5 != 0 {0.16746221 * (fTemp3 + -51.862) + 1.525} else {2.788}}} else {if (fTemp4 < 97.659) as i32 != 0 {2.788 - 0.060793974 * (fTemp3 + -59.404)} else {1.739}};
			self.iRec2[1] = self.iRec2[0];
		}
	}

}


pub fn newDSP_PianoSIG1() -> DSP_PianoSIG1 { 
	DSP_PianoSIG1 {
		iRec2: [0;2],
	}
}

pub struct DSP_PianoSIG2 {
	iRec3: [i32;2],
}

impl DSP_PianoSIG2 {
	
	fn get_num_inputsDSP_PianoSIG2(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsDSP_PianoSIG2(&self) -> i32 {
		return 1;
	}
	
	fn instance_initDSP_PianoSIG2(&mut self, sample_rate: i32) {
		for l2 in 0..2 {
			self.iRec3[l2 as usize] = 0;
		}
	}
	
	fn fillDSP_PianoSIG2(&mut self, count: i32, table: &mut[F32]) {
		for i3 in 0..count {
			self.iRec3[0] = i32::wrapping_add(self.iRec3[1], 1);
			let mut fTemp14: F32 = 1.5918367 * (i32::wrapping_add(self.iRec3[0], -1)) as F32;
			let mut fTemp15: F32 = fTemp14 + 21.0;
			let mut iTemp16: i32 = (fTemp15 < 91.791) as i32;
			let mut iTemp17: i32 = (fTemp15 < 74.201) as i32;
			let mut iTemp18: i32 = (fTemp15 < 66.704) as i32;
			let mut iTemp19: i32 = (fTemp15 < 59.928) as i32;
			let mut iTemp20: i32 = (fTemp15 < 48.0) as i32;
			let mut iTemp21: i32 = (fTemp15 < 42.0) as i32;
			let mut iTemp22: i32 = (fTemp15 < 41.0) as i32;
			let mut iTemp23: i32 = (fTemp15 < 35.562) as i32;
			table[i3 as usize] = if iTemp16 != 0 {if iTemp17 != 0 {if iTemp18 != 0 {if iTemp19 != 0 {if (fTemp15 < 54.0) as i32 != 0 {if iTemp20 != 0 {if iTemp21 != 0 {if iTemp22 != 0 {if iTemp23 != 0 {if (fTemp15 < 3e+01) as i32 != 0 {2.0} else {if iTemp23 != 0 {2.0 - 0.02121539 * (fTemp14 + -9.0)} else {1.882}}} else {if iTemp22 != 0 {1.882 - 0.12541376 * (fTemp14 + -14.562)} else {1.2}}} else {if iTemp21 != 0 {1.2 - 0.6 * (fTemp14 + -2e+01)} else {0.6}}} else {if iTemp20 != 0 {0.6 - 0.016666668 * (fTemp14 + -21.0)} else {0.5}}} else {0.5}} else {if iTemp19 != 0 {0.0003373819 * (fTemp14 + -33.0) + 0.5} else {0.502}}} else {if iTemp18 != 0 {0.502 - 0.001918536 * (fTemp14 + -38.928)} else {0.489}}} else {if iTemp17 != 0 {0.489 - 0.0016006402 * (fTemp14 + -45.704)} else {0.477}}} else {if iTemp16 != 0 {0.029732803 * (fTemp14 + -53.201) + 0.477} else {1.0}}} else {1.0};
			self.iRec3[1] = self.iRec3[0];
		}
	}

}


pub fn newDSP_PianoSIG2() -> DSP_PianoSIG2 { 
	DSP_PianoSIG2 {
		iRec3: [0;2],
	}
}

pub struct DSP_PianoSIG3 {
	iRec8: [i32;2],
}

impl DSP_PianoSIG3 {
	
	fn get_num_inputsDSP_PianoSIG3(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsDSP_PianoSIG3(&self) -> i32 {
		return 1;
	}
	
	fn instance_initDSP_PianoSIG3(&mut self, sample_rate: i32) {
		for l5 in 0..2 {
			self.iRec8[l5 as usize] = 0;
		}
	}
	
	fn fillDSP_PianoSIG3(&mut self, count: i32, table: &mut[F32]) {
		for i4 in 0..count {
			self.iRec8[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec8[1]), 12345);
			table[i4 as usize] = 4.656613e-10 * (self.iRec8[0]) as F32;
			self.iRec8[1] = self.iRec8[0];
		}
	}

}


pub fn newDSP_PianoSIG3() -> DSP_PianoSIG3 { 
	DSP_PianoSIG3 {
		iRec8: [0;2],
	}
}

pub struct DSP_PianoSIG4 {
	iRec9: [i32;2],
}

impl DSP_PianoSIG4 {
	
	fn get_num_inputsDSP_PianoSIG4(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsDSP_PianoSIG4(&self) -> i32 {
		return 1;
	}
	
	fn instance_initDSP_PianoSIG4(&mut self, sample_rate: i32) {
		for l6 in 0..2 {
			self.iRec9[l6 as usize] = 0;
		}
	}
	
	fn fillDSP_PianoSIG4(&mut self, count: i32, table: &mut[F32]) {
		for i5 in 0..count {
			self.iRec9[0] = i32::wrapping_add(self.iRec9[1], 1);
			let mut fTemp26: F32 = 2.2040815 * (i32::wrapping_add(self.iRec9[0], -1)) as F32;
			let mut iTemp27: i32 = (fTemp26 < 95.844) as i32;
			let mut iTemp28: i32 = (fTemp26 < 92.116) as i32;
			let mut iTemp29: i32 = (fTemp26 < 88.531) as i32;
			let mut iTemp30: i32 = (fTemp26 < 84.0) as i32;
			let mut iTemp31: i32 = (fTemp26 < 78.0) as i32;
			let mut iTemp32: i32 = (fTemp26 < 72.232) as i32;
			let mut iTemp33: i32 = (fTemp26 < 66.0) as i32;
			let mut iTemp34: i32 = (fTemp26 < 6e+01) as i32;
			let mut iTemp35: i32 = (fTemp26 < 48.0) as i32;
			let mut iTemp36: i32 = (fTemp26 < 42.0) as i32;
			let mut iTemp37: i32 = (fTemp26 < 36.0) as i32;
			let mut iTemp38: i32 = (fTemp26 < 31.0) as i32;
			let mut iTemp39: i32 = (fTemp26 < 29.0) as i32;
			table[i5 as usize] = if iTemp27 != 0 {if iTemp28 != 0 {if iTemp29 != 0 {if iTemp30 != 0 {if iTemp31 != 0 {if iTemp32 != 0 {if iTemp33 != 0 {if iTemp34 != 0 {if (fTemp26 < 54.0) as i32 != 0 {if iTemp35 != 0 {if iTemp36 != 0 {if iTemp37 != 0 {if iTemp38 != 0 {if iTemp39 != 0 {if (fTemp26 < 28.0) as i32 != 0 {0.003} else {if iTemp39 != 0 {0.057 * (fTemp26 + -28.0) + 0.003} else {0.06}}} else {if iTemp38 != 0 {0.02 * (fTemp26 + -29.0) + 0.06} else {0.1}}} else {if iTemp37 != 0 {0.002 * (fTemp26 + -31.0) + 0.1} else {0.11}}} else {if iTemp36 != 0 {0.0016666667 * (fTemp26 + -36.0) + 0.11} else {0.12}}} else {if iTemp35 != 0 {0.013333334 * (fTemp26 + -42.0) + 0.12} else {0.2}}} else {0.2}} else {if iTemp34 != 0 {0.008333334 * (fTemp26 + -54.0) + 0.2} else {0.25}}} else {if iTemp33 != 0 {0.0033333334 * (fTemp26 + -6e+01) + 0.25} else {0.27}}} else {if iTemp32 != 0 {0.004813864 * (fTemp26 + -66.0) + 0.27} else {0.3}}} else {if iTemp31 != 0 {0.008668516 * (fTemp26 + -72.232) + 0.3} else {0.35}}} else {if iTemp30 != 0 {0.025 * (fTemp26 + -78.0) + 0.35} else {0.5}}} else {if iTemp29 != 0 {0.01809755 * (fTemp26 + -84.0) + 0.5} else {0.582}}} else {if iTemp28 != 0 {0.022873083 * (fTemp26 + -88.531) + 0.582} else {0.664}}} else {if iTemp27 != 0 {0.034603003 * (fTemp26 + -92.116) + 0.664} else {0.793}}} else {if (fTemp26 < 99.0) as i32 != 0 {0.06558935 * (fTemp26 + -95.844) + 0.793} else {1.0}};
			self.iRec9[1] = self.iRec9[0];
		}
	}

}


pub fn newDSP_PianoSIG4() -> DSP_PianoSIG4 { 
	DSP_PianoSIG4 {
		iRec9: [0;2],
	}
}

pub struct DSP_PianoSIG5 {
	iRec10: [i32;2],
}

impl DSP_PianoSIG5 {
	
	fn get_num_inputsDSP_PianoSIG5(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsDSP_PianoSIG5(&self) -> i32 {
		return 1;
	}
	
	fn instance_initDSP_PianoSIG5(&mut self, sample_rate: i32) {
		for l7 in 0..2 {
			self.iRec10[l7 as usize] = 0;
		}
	}
	
	fn fillDSP_PianoSIG5(&mut self, count: i32, table: &mut[F32]) {
		for i6 in 0..count {
			self.iRec10[0] = i32::wrapping_add(self.iRec10[1], 1);
			let mut fTemp41: F32 = (i32::wrapping_add(self.iRec10[0], -1)) as F32;
			let mut fTemp42: F32 = 1.56 * fTemp41;
			let mut fTemp43: F32 = fTemp42 + 21.0;
			let mut iTemp44: i32 = (fTemp43 < 36.0) as i32;
			let mut iTemp45: i32 = (fTemp43 < 32.0) as i32;
			let mut iTemp46: i32 = (fTemp43 < 28.0) as i32;
			let mut iTemp47: i32 = (fTemp43 < 26.335) as i32;
			let mut iTemp48: i32 = (fTemp43 < 24.604) as i32;
			table[i6 as usize] = if (fTemp43 < 96.0) as i32 != 0 {if (fTemp43 < 88.0) as i32 != 0 {if (fTemp43 < 84.0) as i32 != 0 {if (fTemp43 < 76.0) as i32 != 0 {if (fTemp43 < 72.0) as i32 != 0 {if (fTemp43 < 66.0) as i32 != 0 {if (fTemp43 < 6e+01) as i32 != 0 {if (fTemp43 < 54.0) as i32 != 0 {if (fTemp43 < 48.0) as i32 != 0 {if (fTemp43 < 42.0) as i32 != 0 {if iTemp44 != 0 {if iTemp45 != 0 {if iTemp46 != 0 {if iTemp47 != 0 {if iTemp48 != 0 {if (fTemp43 < 21.0) as i32 != 0 {0.35} else {if iTemp48 != 0 {0.35 - 0.013851277 * fTemp41} else {0.318}}} else {if iTemp47 != 0 {0.318 - 0.022530328 * (fTemp42 + -3.604)} else {0.279}}} else {if iTemp46 != 0 {0.279 - 0.017417418 * (fTemp42 + -5.335)} else {0.25}}} else {if iTemp45 != 0 {0.25 - 0.025 * (fTemp42 + -7.0)} else {0.15}}} else {if iTemp44 != 0 {0.15 - 0.0375 * (fTemp42 + -11.0)} else {0.0}}} else {0.0}} else {0.0}} else {0.0}} else {0.0}} else {0.0}} else {0.0}} else {0.0}} else {0.0}} else {0.0}} else {0.0}} else {0.0};
			self.iRec10[1] = self.iRec10[0];
		}
	}

}


pub fn newDSP_PianoSIG5() -> DSP_PianoSIG5 { 
	DSP_PianoSIG5 {
		iRec10: [0;2],
	}
}

pub struct DSP_PianoSIG6 {
	iRec11: [i32;2],
}

impl DSP_PianoSIG6 {
	
	fn get_num_inputsDSP_PianoSIG6(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsDSP_PianoSIG6(&self) -> i32 {
		return 1;
	}
	
	fn instance_initDSP_PianoSIG6(&mut self, sample_rate: i32) {
		for l8 in 0..2 {
			self.iRec11[l8 as usize] = 0;
		}
	}
	
	fn fillDSP_PianoSIG6(&mut self, count: i32, table: &mut[F32]) {
		for i7 in 0..count {
			self.iRec11[0] = i32::wrapping_add(self.iRec11[1], 1);
			let mut fTemp49: F32 = 1.56 * (i32::wrapping_add(self.iRec11[0], -1)) as F32;
			let mut fTemp50: F32 = fTemp49 + 21.0;
			let mut iTemp51: i32 = (fTemp50 < 95.684) as i32;
			let mut iTemp52: i32 = (fTemp50 < 92.368) as i32;
			let mut iTemp53: i32 = (fTemp50 < 88.619) as i32;
			let mut iTemp54: i32 = (fTemp50 < 84.0) as i32;
			let mut iTemp55: i32 = (fTemp50 < 78.0) as i32;
			let mut iTemp56: i32 = (fTemp50 < 72.0) as i32;
			let mut iTemp57: i32 = (fTemp50 < 66.0) as i32;
			let mut iTemp58: i32 = (fTemp50 < 6e+01) as i32;
			let mut iTemp59: i32 = (fTemp50 < 48.0) as i32;
			let mut iTemp60: i32 = (fTemp50 < 42.0) as i32;
			let mut iTemp61: i32 = (fTemp50 < 29.0) as i32;
			table[i7 as usize] = if iTemp51 != 0 {if iTemp52 != 0 {if iTemp53 != 0 {if iTemp54 != 0 {if iTemp55 != 0 {if iTemp56 != 0 {if iTemp57 != 0 {if iTemp58 != 0 {if (fTemp50 < 52.836) as i32 != 0 {if iTemp59 != 0 {if iTemp60 != 0 {if (fTemp50 < 36.0) as i32 != 0 {if iTemp61 != 0 {if (fTemp50 < 28.0) as i32 != 0 {-1.5} else {if iTemp61 != 0 {-1.5 - 4.5 * (fTemp49 + -7.0)} else {-6.0}}} else {-6.0}} else {if iTemp60 != 0 {-6.0 - 0.016666668 * (fTemp49 + -15.0)} else {-6.1}}} else {if iTemp59 != 0 {-6.1 - 0.15 * (fTemp49 + -21.0)} else {-7.0}}} else {-7.0}} else {if iTemp58 != 0 {-7.0 - 0.041876048 * (fTemp49 + -31.836)} else {-7.3}}} else {if iTemp57 != 0 {-7.3 - 0.06666667 * (fTemp49 + -39.0)} else {-7.7}}} else {if iTemp56 != 0 {-7.7 - 0.05 * (fTemp49 + -45.0)} else {-8.0}}} else {if iTemp55 != 0 {-8.0 - 0.13333334 * (fTemp49 + -51.0)} else {-8.8}}} else {if iTemp54 != 0 {-8.8 - 0.2 * (fTemp49 + -57.0)} else {-1e+01}}} else {if iTemp53 != 0 {-1e+01 - 0.26304394 * (fTemp49 + -63.0)} else {-11.215}}} else {if iTemp52 != 0 {-11.215 - 0.30221394 * (fTemp49 + -67.619)} else {-12.348}}} else {if iTemp51 != 0 {-12.348 - 0.4782871 * (fTemp49 + -71.368)} else {-13.934}}} else {if (fTemp50 < 99.0) as i32 != 0 {-13.934 - 0.32147166 * (fTemp49 + -74.684)} else {-15.0}};
			self.iRec11[1] = self.iRec11[0];
		}
	}

}


pub fn newDSP_PianoSIG6() -> DSP_PianoSIG6 { 
	DSP_PianoSIG6 {
		iRec11: [0;2],
	}
}

pub struct DSP_PianoSIG7 {
	iRec12: [i32;2],
}

impl DSP_PianoSIG7 {
	
	fn get_num_inputsDSP_PianoSIG7(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsDSP_PianoSIG7(&self) -> i32 {
		return 1;
	}
	
	fn instance_initDSP_PianoSIG7(&mut self, sample_rate: i32) {
		for l9 in 0..2 {
			self.iRec12[l9 as usize] = 0;
		}
	}
	
	fn fillDSP_PianoSIG7(&mut self, count: i32, table: &mut[F32]) {
		for i8 in 0..count {
			self.iRec12[0] = i32::wrapping_add(self.iRec12[1], 1);
			let mut fTemp62: F32 = 2.3636363 * (i32::wrapping_add(self.iRec12[0], -1)) as F32;
			let mut fTemp63: F32 = fTemp62 + 21.0;
			let mut iTemp64: i32 = (fTemp63 < 96.0) as i32;
			let mut iTemp65: i32 = (fTemp63 < 88.0) as i32;
			let mut iTemp66: i32 = (fTemp63 < 84.0) as i32;
			let mut iTemp67: i32 = (fTemp63 < 79.0) as i32;
			let mut iTemp68: i32 = (fTemp63 < 66.0) as i32;
			let mut iTemp69: i32 = (fTemp63 < 6e+01) as i32;
			let mut iTemp70: i32 = (fTemp63 < 54.0) as i32;
			let mut iTemp71: i32 = (fTemp63 < 48.0) as i32;
			let mut iTemp72: i32 = (fTemp63 < 42.0) as i32;
			let mut iTemp73: i32 = (fTemp63 < 36.0) as i32;
			table[i8 as usize] = if iTemp64 != 0 {if iTemp65 != 0 {if iTemp66 != 0 {if iTemp67 != 0 {if (fTemp63 < 72.0) as i32 != 0 {if iTemp68 != 0 {if iTemp69 != 0 {if iTemp70 != 0 {if iTemp71 != 0 {if iTemp72 != 0 {if iTemp73 != 0 {if (fTemp63 < 32.534) as i32 != 0 {-1.0} else {if iTemp73 != 0 {0.08655511 * (fTemp62 + -11.534) + -1.0} else {-0.7}}} else {if iTemp72 != 0 {0.05 * (fTemp62 + -15.0) + -0.7} else {-0.4}}} else {if iTemp71 != 0 {0.033333335 * (fTemp62 + -21.0) + -0.4} else {-0.2}}} else {if iTemp70 != 0 {0.013333334 * (fTemp62 + -27.0) + -0.2} else {-0.12}}} else {if iTemp69 != 0 {0.006666667 * (fTemp62 + -33.0) + -0.12} else {-0.08}}} else {if iTemp68 != 0 {0.0016666667 * (fTemp62 + -39.0) + -0.08} else {-0.07}}} else {-0.07}} else {if iTemp67 != 0 {0.0007142857 * (fTemp62 + -51.0) + -0.07} else {-0.065}}} else {if iTemp66 != 0 {0.0004 * (fTemp62 + -58.0) + -0.065} else {-0.063}}} else {if iTemp65 != 0 {0.00075 * (fTemp62 + -63.0) + -0.063} else {-0.06}}} else {if iTemp64 != 0 {0.00125 * (fTemp62 + -67.0) + -0.06} else {-0.05}}} else {-0.05};
			self.iRec12[1] = self.iRec12[0];
		}
	}

}


pub fn newDSP_PianoSIG7() -> DSP_PianoSIG7 { 
	DSP_PianoSIG7 {
		iRec12: [0;2],
	}
}

pub struct DSP_PianoSIG8 {
	iRec13: [i32;2],
}

impl DSP_PianoSIG8 {
	
	fn get_num_inputsDSP_PianoSIG8(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsDSP_PianoSIG8(&self) -> i32 {
		return 1;
	}
	
	fn instance_initDSP_PianoSIG8(&mut self, sample_rate: i32) {
		for l10 in 0..2 {
			self.iRec13[l10 as usize] = 0;
		}
	}
	
	fn fillDSP_PianoSIG8(&mut self, count: i32, table: &mut[F32]) {
		for i9 in 0..count {
			self.iRec13[0] = i32::wrapping_add(self.iRec13[1], 1);
			let mut fTemp74: F32 = 2.2040815 * (i32::wrapping_add(self.iRec13[0], -1)) as F32;
			let mut iTemp75: i32 = (fTemp74 < 93.81) as i32;
			let mut iTemp76: i32 = (fTemp74 < 73.625) as i32;
			let mut iTemp77: i32 = (fTemp74 < 6e+01) as i32;
			let mut iTemp78: i32 = (fTemp74 < 46.952) as i32;
			let mut iTemp79: i32 = (fTemp74 < 37.725) as i32;
			let mut iTemp80: i32 = (fTemp74 < 29.0) as i32;
			let mut iTemp81: i32 = (fTemp74 < 27.055) as i32;
			table[i9 as usize] = if iTemp75 != 0 {if iTemp76 != 0 {if iTemp77 != 0 {if iTemp78 != 0 {if iTemp79 != 0 {if iTemp80 != 0 {if iTemp81 != 0 {if (fTemp74 < 23.595) as i32 != 0 {-0.85} else {if iTemp81 != 0 {0.0057803467 * (fTemp74 + -23.595) + -0.85} else {-0.83}}} else {if iTemp80 != 0 {0.06683805 * (fTemp74 + -27.055) + -0.83} else {-0.7}}} else {if iTemp79 != 0 {0.021088826 * (fTemp74 + -29.0) + -0.7} else {-0.516}}} else {if iTemp78 != 0 {0.017773924 * (fTemp74 + -37.725) + -0.516} else {-0.352}}} else {if iTemp77 != 0 {0.00781729 * (fTemp74 + -46.952) + -0.352} else {-0.25}}} else {if iTemp76 != 0 {0.015706422 * (fTemp74 + -6e+01) + -0.25} else {-0.036}}} else {if iTemp75 != 0 {0.0014862522 * (fTemp74 + -73.625) + -0.036} else {-0.006}}} else {if (fTemp74 < 99.0) as i32 != 0 {0.0007707129 * (fTemp74 + -93.81) + -0.006} else {-0.002}};
			self.iRec13[1] = self.iRec13[0];
		}
	}

}


pub fn newDSP_PianoSIG8() -> DSP_PianoSIG8 { 
	DSP_PianoSIG8 {
		iRec13: [0;2],
	}
}

pub struct DSP_PianoSIG9 {
	iRec19: [i32;2],
}

impl DSP_PianoSIG9 {
	
	fn get_num_inputsDSP_PianoSIG9(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsDSP_PianoSIG9(&self) -> i32 {
		return 1;
	}
	
	fn instance_initDSP_PianoSIG9(&mut self, sample_rate: i32) {
		for l11 in 0..2 {
			self.iRec19[l11 as usize] = 0;
		}
	}
	
	fn fillDSP_PianoSIG9(&mut self, count: i32, table: &mut[F32]) {
		for i10 in 0..count {
			self.iRec19[0] = i32::wrapping_add(self.iRec19[1], 1);
			let mut fTemp84: F32 = 1.6980612 * (i32::wrapping_add(self.iRec19[0], -1)) as F32;
			let mut fTemp85: F32 = fTemp84 + 21.0;
			let mut iTemp86: i32 = (fTemp85 < 100.982) as i32;
			let mut iTemp87: i32 = (fTemp85 < 91.157) as i32;
			let mut iTemp88: i32 = (fTemp85 < 84.673) as i32;
			let mut iTemp89: i32 = (fTemp85 < 78.0) as i32;
			let mut iTemp90: i32 = (fTemp85 < 6e+01) as i32;
			let mut iTemp91: i32 = (fTemp85 < 54.0) as i32;
			let mut iTemp92: i32 = (fTemp85 < 48.0) as i32;
			table[i10 as usize] = if iTemp86 != 0 {if iTemp87 != 0 {if iTemp88 != 0 {if iTemp89 != 0 {if (fTemp85 < 72.0) as i32 != 0 {if (fTemp85 < 66.0) as i32 != 0 {if iTemp90 != 0 {if iTemp91 != 0 {if iTemp92 != 0 {if (fTemp85 < 42.0) as i32 != 0 {0.99} else {if iTemp92 != 0 {0.99 - 0.00083333335 * (fTemp84 + -21.0)} else {0.985}}} else {if iTemp91 != 0 {0.985 - 0.0025 * (fTemp84 + -27.0)} else {0.97}}} else {if iTemp90 != 0 {0.97 - 0.0016666667 * (fTemp84 + -33.0)} else {0.96}}} else {0.96}} else {0.96}} else {if iTemp89 != 0 {0.0016666667 * (fTemp84 + -51.0) + 0.96} else {0.97}}} else {if iTemp88 != 0 {0.0007492882 * (fTemp84 + -57.0) + 0.97} else {0.975}}} else {if iTemp87 != 0 {0.0023133869 * (fTemp84 + -63.673) + 0.975} else {0.99}}} else {if iTemp86 != 0 {0.99 - 0.0020356234 * (fTemp84 + -70.157)} else {0.97}}} else {if (fTemp85 < 104.205) as i32 != 0 {0.97 - 0.0062053986 * (fTemp84 + -79.982)} else {0.95}};
			self.iRec19[1] = self.iRec19[0];
		}
	}

}


pub fn newDSP_PianoSIG9() -> DSP_PianoSIG9 { 
	DSP_PianoSIG9 {
		iRec19: [0;2],
	}
}

pub struct DSP_PianoSIG10 {
	iRec20: [i32;2],
}

impl DSP_PianoSIG10 {
	
	fn get_num_inputsDSP_PianoSIG10(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsDSP_PianoSIG10(&self) -> i32 {
		return 1;
	}
	
	fn instance_initDSP_PianoSIG10(&mut self, sample_rate: i32) {
		for l12 in 0..2 {
			self.iRec20[l12 as usize] = 0;
		}
	}
	
	fn fillDSP_PianoSIG10(&mut self, count: i32, table: &mut[F32]) {
		for i11 in 0..count {
			self.iRec20[0] = i32::wrapping_add(self.iRec20[1], 1);
			let mut fTemp93: F32 = (i32::wrapping_add(self.iRec20[0], -1)) as F32;
			let mut fTemp94: F32 = 1.7689388 * fTemp93;
			let mut fTemp95: F32 = fTemp94 + 21.0;
			let mut iTemp96: i32 = (fTemp95 < 103.512) as i32;
			let mut iTemp97: i32 = (fTemp95 < 96.463) as i32;
			let mut iTemp98: i32 = (fTemp95 < 84.446) as i32;
			let mut iTemp99: i32 = (fTemp95 < 78.839) as i32;
			let mut iTemp100: i32 = (fTemp95 < 72.0) as i32;
			let mut iTemp101: i32 = (fTemp95 < 6e+01) as i32;
			let mut iTemp102: i32 = (fTemp95 < 54.0) as i32;
			let mut iTemp103: i32 = (fTemp95 < 45.788) as i32;
			let mut iTemp104: i32 = (fTemp95 < 40.671) as i32;
			let mut iTemp105: i32 = (fTemp95 < 36.672) as i32;
			let mut iTemp106: i32 = (fTemp95 < 32.355) as i32;
			let mut iTemp107: i32 = (fTemp95 < 28.996) as i32;
			let mut iTemp108: i32 = (fTemp95 < 27.237) as i32;
			let mut iTemp109: i32 = (fTemp95 < 23.719) as i32;
			table[i11 as usize] = if iTemp96 != 0 {if iTemp97 != 0 {if (fTemp95 < 89.894) as i32 != 0 {if iTemp98 != 0 {if iTemp99 != 0 {if iTemp100 != 0 {if (fTemp95 < 66.0) as i32 != 0 {if iTemp101 != 0 {if iTemp102 != 0 {if (fTemp95 < 47.867) as i32 != 0 {if iTemp103 != 0 {if iTemp104 != 0 {if iTemp105 != 0 {if iTemp106 != 0 {if iTemp107 != 0 {if iTemp108 != 0 {if iTemp109 != 0 {if (fTemp95 < 21.0) as i32 != 0 {0.875} else {if iTemp109 != 0 {0.875 - 0.0026023374 * fTemp93} else {0.871}}} else {if iTemp108 != 0 {0.871 - 0.009948835 * (fTemp94 + -2.719)} else {0.836}}} else {if iTemp107 != 0 {0.836 - 0.004548039 * (fTemp94 + -6.237)} else {0.828}}} else {if iTemp106 != 0 {0.828 - 0.0023816612 * (fTemp94 + -7.996)} else {0.82}}} else {if iTemp105 != 0 {0.82 - 0.0009265694 * (fTemp94 + -11.355)} else {0.816}}} else {if iTemp104 != 0 {0.0010002501 * (fTemp94 + -15.672) + 0.816} else {0.82}}} else {if iTemp103 != 0 {0.82 - 0.0015634161 * (fTemp94 + -19.671)} else {0.812}}} else {0.812}} else {if iTemp102 != 0 {0.812 - 0.00032610467 * (fTemp94 + -26.867)} else {0.81}}} else {if iTemp101 != 0 {0.81 - 0.0016666667 * (fTemp94 + -33.0)} else {0.8}}} else {0.8}} else {if iTemp100 != 0 {0.0016666667 * (fTemp94 + -45.0) + 0.8} else {0.81}}} else {if iTemp99 != 0 {0.0020470829 * (fTemp94 + -51.0) + 0.81} else {0.824}}} else {if iTemp98 != 0 {0.00356697 * (fTemp94 + -57.839) + 0.824} else {0.844}}} else {0.844}} else {if iTemp97 != 0 {0.0006089207 * (fTemp94 + -68.894) + 0.844} else {0.848}}} else {if iTemp96 != 0 {0.848 - 0.0011349127 * (fTemp94 + -75.463)} else {0.84}}} else {0.84};
			self.iRec20[1] = self.iRec20[0];
		}
	}

}


pub fn newDSP_PianoSIG10() -> DSP_PianoSIG10 { 
	DSP_PianoSIG10 {
		iRec20: [0;2],
	}
}

pub struct DSP_PianoSIG11 {
	iRec21: [i32;2],
}

impl DSP_PianoSIG11 {
	
	fn get_num_inputsDSP_PianoSIG11(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsDSP_PianoSIG11(&self) -> i32 {
		return 1;
	}
	
	fn instance_initDSP_PianoSIG11(&mut self, sample_rate: i32) {
		for l13 in 0..2 {
			self.iRec21[l13 as usize] = 0;
		}
	}
	
	fn fillDSP_PianoSIG11(&mut self, count: i32, table: &mut[F32]) {
		for i12 in 0..count {
			self.iRec21[0] = i32::wrapping_add(self.iRec21[1], 1);
			let mut fTemp110: F32 = (i32::wrapping_add(self.iRec21[0], -1)) as F32;
			let mut fTemp111: F32 = 1.5944285 * fTemp110;
			let mut fTemp112: F32 = fTemp111 + 21.873;
			let mut iTemp113: i32 = (fTemp112 < 99.0) as i32;
			let mut iTemp114: i32 = (fTemp112 < 95.668) as i32;
			let mut iTemp115: i32 = (fTemp112 < 91.624) as i32;
			let mut iTemp116: i32 = (fTemp112 < 85.27) as i32;
			let mut iTemp117: i32 = (fTemp112 < 79.06) as i32;
			let mut iTemp118: i32 = (fTemp112 < 72.995) as i32;
			let mut iTemp119: i32 = (fTemp112 < 65.63) as i32;
			let mut iTemp120: i32 = (fTemp112 < 60.72) as i32;
			let mut iTemp121: i32 = (fTemp112 < 53.644) as i32;
			let mut iTemp122: i32 = (fTemp112 < 47.434) as i32;
			let mut iTemp123: i32 = (fTemp112 < 41.513) as i32;
			let mut iTemp124: i32 = (fTemp112 < 35.448) as i32;
			let mut iTemp125: i32 = (fTemp112 < 30.538) as i32;
			let mut iTemp126: i32 = (fTemp112 < 25.194) as i32;
			table[i12 as usize] = if iTemp113 != 0 {if iTemp114 != 0 {if iTemp115 != 0 {if iTemp116 != 0 {if iTemp117 != 0 {if iTemp118 != 0 {if iTemp119 != 0 {if iTemp120 != 0 {if iTemp121 != 0 {if iTemp122 != 0 {if iTemp123 != 0 {if iTemp124 != 0 {if iTemp125 != 0 {if iTemp126 != 0 {if (fTemp112 < 21.873) as i32 != 0 {0.891} else {if iTemp126 != 0 {0.891 - 0.010082204 * fTemp110} else {0.87}}} else {if iTemp125 != 0 {0.87 - 0.0041167666 * (fTemp111 + -3.321)} else {0.848}}} else {if iTemp124 != 0 {0.00101833 * (fTemp111 + -8.665) + 0.848} else {0.853}}} else {if iTemp123 != 0 {0.853 - 0.001813685 * (fTemp111 + -13.575)} else {0.842}}} else {if iTemp122 != 0 {0.842 - 0.0027022462 * (fTemp111 + -19.64)} else {0.826}}} else {if iTemp121 != 0 {0.826 - 0.00096618355 * (fTemp111 + -25.561)} else {0.82}}} else {if iTemp120 != 0 {0.82 - 0.0007066139 * (fTemp111 + -31.771)} else {0.815}}} else {if iTemp119 != 0 {0.00101833 * (fTemp111 + -38.847) + 0.815} else {0.82}}} else {if iTemp118 != 0 {0.0044806516 * (fTemp111 + -43.757) + 0.82} else {0.853}}} else {if iTemp117 != 0 {0.011046991 * (fTemp111 + -51.122) + 0.853} else {0.92}}} else {if iTemp116 != 0 {0.017391304 * (fTemp111 + -57.187) + 0.92} else {1.028}}} else {if iTemp115 != 0 {0.0029902423 * (fTemp111 + -63.397) + 1.028} else {1.047}}} else {if iTemp114 != 0 {0.012116716 * (fTemp111 + -69.751) + 1.047} else {1.096}}} else {if iTemp113 != 0 {0.0012004802 * (fTemp111 + -73.795) + 1.096} else {1.1}}} else {1.1};
			self.iRec21[1] = self.iRec21[0];
		}
	}

}


pub fn newDSP_PianoSIG11() -> DSP_PianoSIG11 { 
	DSP_PianoSIG11 {
		iRec21: [0;2],
	}
}

pub struct DSP_PianoSIG12 {
	iRec27: [i32;2],
}

impl DSP_PianoSIG12 {
	
	fn get_num_inputsDSP_PianoSIG12(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsDSP_PianoSIG12(&self) -> i32 {
		return 1;
	}
	
	fn instance_initDSP_PianoSIG12(&mut self, sample_rate: i32) {
		for l15 in 0..2 {
			self.iRec27[l15 as usize] = 0;
		}
	}
	
	fn fillDSP_PianoSIG12(&mut self, count: i32, table: &mut[F32]) {
		for i13 in 0..count {
			self.iRec27[0] = i32::wrapping_add(self.iRec27[1], 1);
			let mut fTemp129: F32 = 7.25 * (i32::wrapping_add(self.iRec27[0], -1)) as F32;
			let mut fTemp130: F32 = fTemp129 + 21.0;
			let mut iTemp131: i32 = (fTemp130 < 84.0) as i32;
			let mut iTemp132: i32 = (fTemp130 < 72.0) as i32;
			let mut iTemp133: i32 = (fTemp130 < 66.0) as i32;
			let mut iTemp134: i32 = (fTemp130 < 6e+01) as i32;
			let mut iTemp135: i32 = (fTemp130 < 48.0) as i32;
			let mut iTemp136: i32 = (fTemp130 < 36.0) as i32;
			let mut iTemp137: i32 = (fTemp130 < 31.0) as i32;
			table[i13 as usize] = if (fTemp130 < 96.0) as i32 != 0 {if (fTemp130 < 9e+01) as i32 != 0 {if iTemp131 != 0 {if (fTemp130 < 78.0) as i32 != 0 {if iTemp132 != 0 {if iTemp133 != 0 {if iTemp134 != 0 {if iTemp135 != 0 {if iTemp136 != 0 {if iTemp137 != 0 {if (fTemp130 < 24.0) as i32 != 0 {0.05} else {if iTemp137 != 0 {0.05 - 0.0028571428 * (fTemp129 + -3.0)} else {0.03}}} else {if iTemp136 != 0 {0.03 - 0.001 * (fTemp129 + -1e+01)} else {0.025}}} else {if iTemp135 != 0 {0.025 - 0.00125 * (fTemp129 + -15.0)} else {0.01}}} else {if iTemp134 != 0 {0.01 - 0.00041666668 * (fTemp129 + -27.0)} else {0.005}}} else {if iTemp133 != 0 {0.005 - 0.00033333333 * (fTemp129 + -39.0)} else {0.003}}} else {if iTemp132 != 0 {0.003 - 0.00016666666 * (fTemp129 + -45.0)} else {0.002}}} else {0.002}} else {if iTemp131 != 0 {0.00016666666 * (fTemp129 + -57.0) + 0.002} else {0.003}}} else {0.003}} else {0.003}} else {if (fTemp130 < 108.0) as i32 != 0 {0.003 - 8.333333e-05 * (fTemp129 + -75.0)} else {0.002}};
			self.iRec27[1] = self.iRec27[0];
		}
	}

}


pub fn newDSP_PianoSIG12() -> DSP_PianoSIG12 { 
	DSP_PianoSIG12 {
		iRec27: [0;2],
	}
}

pub struct DSP_PianoSIG13 {
	iRec29: [i32;2],
}

impl DSP_PianoSIG13 {
	
	fn get_num_inputsDSP_PianoSIG13(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsDSP_PianoSIG13(&self) -> i32 {
		return 1;
	}
	
	fn instance_initDSP_PianoSIG13(&mut self, sample_rate: i32) {
		for l17 in 0..2 {
			self.iRec29[l17 as usize] = 0;
		}
	}
	
	fn fillDSP_PianoSIG13(&mut self, count: i32, table: &mut[F32]) {
		for i14 in 0..count {
			self.iRec29[0] = i32::wrapping_add(self.iRec29[1], 1);
			let mut fTemp139: F32 = (i32::wrapping_add(self.iRec29[0], -1)) as F32;
			let mut fTemp140: F32 = 1.7591225 * fTemp139;
			let mut fTemp141: F32 = fTemp140 + 21.001;
			let mut iTemp142: i32 = (fTemp141 < 102.4) as i32;
			let mut iTemp143: i32 = (fTemp141 < 97.319) as i32;
			let mut iTemp144: i32 = (fTemp141 < 92.858) as i32;
			let mut iTemp145: i32 = (fTemp141 < 88.557) as i32;
			let mut iTemp146: i32 = (fTemp141 < 83.936) as i32;
			let mut iTemp147: i32 = (fTemp141 < 55.0) as i32;
			let mut iTemp148: i32 = (fTemp141 < 47.977) as i32;
			let mut iTemp149: i32 = (fTemp141 < 40.794) as i32;
			let mut iTemp150: i32 = (fTemp141 < 34.249) as i32;
			let mut iTemp151: i32 = (fTemp141 < 26.587) as i32;
			table[i14 as usize] = if iTemp142 != 0 {if iTemp143 != 0 {if iTemp144 != 0 {if iTemp145 != 0 {if iTemp146 != 0 {if (fTemp141 < 78.0) as i32 != 0 {if (fTemp141 < 71.934) as i32 != 0 {if (fTemp141 < 66.0) as i32 != 0 {if (fTemp141 < 6e+01) as i32 != 0 {if iTemp147 != 0 {if iTemp148 != 0 {if iTemp149 != 0 {if iTemp150 != 0 {if iTemp151 != 0 {if (fTemp141 < 21.001) as i32 != 0 {0.491} else {if iTemp151 != 0 {0.002204414 * fTemp139 + 0.491} else {0.498}}} else {if iTemp150 != 0 {0.498 - 0.0036543983 * (fTemp140 + -5.586)} else {0.47}}} else {if iTemp149 != 0 {0.47 - 0.004430863 * (fTemp140 + -13.248)} else {0.441}}} else {if iTemp148 != 0 {0.441 - 0.006821662 * (fTemp140 + -19.793)} else {0.392}}} else {if iTemp147 != 0 {0.392 - 0.0031325645 * (fTemp140 + -26.976)} else {0.37}}} else {0.37}} else {0.37}} else {0.37}} else {0.37}} else {if iTemp146 != 0 {0.0033692722 * (fTemp140 + -56.999) + 0.37} else {0.39}}} else {if iTemp145 != 0 {0.39 - 0.0006492101 * (fTemp140 + -62.935)} else {0.387}}} else {if iTemp144 != 0 {0.003022553 * (fTemp140 + -67.556) + 0.387} else {0.4}}} else {if iTemp143 != 0 {0.015467384 * (fTemp140 + -71.857) + 0.4} else {0.469}}} else {if iTemp142 != 0 {0.0061011612 * (fTemp140 + -76.318) + 0.469} else {0.5}}} else {if (fTemp141 < 107.198) as i32 != 0 {0.5 - 0.001250521 * (fTemp140 + -81.399)} else {0.494}};
			self.iRec29[1] = self.iRec29[0];
		}
	}

}


pub fn newDSP_PianoSIG13() -> DSP_PianoSIG13 { 
	DSP_PianoSIG13 {
		iRec29: [0;2],
	}
}

pub struct DSP_PianoSIG14 {
	iRec35: [i32;2],
}

impl DSP_PianoSIG14 {
	
	fn get_num_inputsDSP_PianoSIG14(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsDSP_PianoSIG14(&self) -> i32 {
		return 1;
	}
	
	fn instance_initDSP_PianoSIG14(&mut self, sample_rate: i32) {
		for l29 in 0..2 {
			self.iRec35[l29 as usize] = 0;
		}
	}
	
	fn fillDSP_PianoSIG14(&mut self, count: i32, table: &mut[F32]) {
		for i15 in 0..count {
			self.iRec35[0] = i32::wrapping_add(self.iRec35[1], 1);
			let mut fTemp155: F32 = (i32::wrapping_add(self.iRec35[0], -1)) as F32;
			let mut fTemp156: F32 = 8.666667 * fTemp155;
			let mut fTemp157: F32 = fTemp156 + 21.0;
			let mut iTemp158: i32 = (fTemp157 < 88.0) as i32;
			let mut iTemp159: i32 = (fTemp157 < 84.0) as i32;
			let mut iTemp160: i32 = (fTemp157 < 72.0) as i32;
			let mut iTemp161: i32 = (fTemp157 < 6e+01) as i32;
			let mut iTemp162: i32 = (fTemp157 < 48.0) as i32;
			let mut iTemp163: i32 = (fTemp157 < 36.0) as i32;
			let mut iTemp164: i32 = (fTemp157 < 29.0) as i32;
			let mut iTemp165: i32 = (fTemp157 < 24.0) as i32;
			table[i15 as usize] = if iTemp158 != 0 {if iTemp159 != 0 {if iTemp160 != 0 {if iTemp161 != 0 {if iTemp162 != 0 {if iTemp163 != 0 {if iTemp164 != 0 {if iTemp165 != 0 {if (fTemp157 < 21.0) as i32 != 0 {0.865} else {if iTemp165 != 0 {0.043333333 * fTemp155 + 0.865} else {0.88}}} else {if iTemp164 != 0 {0.0032 * (fTemp156 + -3.0) + 0.88} else {0.896}}} else {if iTemp163 != 0 {0.002 * (fTemp156 + -8.0) + 0.896} else {0.91}}} else {if iTemp162 != 0 {0.00083333335 * (fTemp156 + -15.0) + 0.91} else {0.92}}} else {if iTemp161 != 0 {0.0025 * (fTemp156 + -27.0) + 0.92} else {0.95}}} else {if iTemp160 != 0 {0.00125 * (fTemp156 + -39.0) + 0.95} else {0.965}}} else {if iTemp159 != 0 {0.0019166666 * (fTemp156 + -51.0) + 0.965} else {0.988}}} else {if iTemp158 != 0 {0.00225 * (fTemp156 + -63.0) + 0.988} else {0.997}}} else {if (fTemp157 < 99.0) as i32 != 0 {0.997 - 0.00081818184 * (fTemp156 + -67.0)} else {0.988}};
			self.iRec35[1] = self.iRec35[0];
		}
	}

}


pub fn newDSP_PianoSIG14() -> DSP_PianoSIG14 { 
	DSP_PianoSIG14 {
		iRec35: [0;2],
	}
}
static mut ftbl0DSP_PianoSIG0: [F32;12] = [0.0;12];
static mut ftbl1DSP_PianoSIG1: [F32;50] = [0.0;50];
static mut ftbl2DSP_PianoSIG2: [F32;50] = [0.0;50];
static mut ftbl3DSP_PianoSIG3: [F32;256] = [0.0;256];
static mut ftbl4DSP_PianoSIG4: [F32;50] = [0.0;50];
static mut ftbl5DSP_PianoSIG5: [F32;51] = [0.0;51];
static mut ftbl6DSP_PianoSIG6: [F32;51] = [0.0;51];
static mut ftbl7DSP_PianoSIG7: [F32;34] = [0.0;34];
fn DSP_Piano_faustpower2_f(value: F32) -> F32 {
	return value * value;
}
static mut ftbl8DSP_PianoSIG8: [F32;50] = [0.0;50];
static mut ftbl9DSP_PianoSIG9: [F32;50] = [0.0;50];
static mut ftbl10DSP_PianoSIG10: [F32;50] = [0.0;50];
static mut ftbl11DSP_PianoSIG11: [F32;50] = [0.0;50];
static mut ftbl12DSP_PianoSIG12: [F32;13] = [0.0;13];
static mut ftbl13DSP_PianoSIG13: [F32;50] = [0.0;50];
static mut ftbl14DSP_PianoSIG14: [F32;10] = [0.0;10];
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
pub struct DSP_Piano {
	fEntry0: F32,
	fHslider0: F32,
	fEntry1: F32,
	fEntry2: F32,
	fSampleRate: i32,
	fConst0: F32,
	fConst1: F32,
	fConst2: F32,
	fButton0: F32,
	fVec0: [F32;2],
	fConst3: F32,
	fRec7: [F32;2],
	fHslider1: F32,
	fHslider2: F32,
	fHslider3: F32,
	fConst4: F32,
	IOTA0: i32,
	fConst5: F32,
	fConst6: F32,
	iConst7: i32,
	iConst8: i32,
	fConst9: F32,
	fConst10: F32,
	iConst11: i32,
	fConst12: F32,
	fHslider4: F32,
	fEntry3: F32,
	fButton1: F32,
	fConst13: F32,
	fRec26: [F32;2],
	fConst14: F32,
	fConst15: F32,
	fRec25: [F32;2],
	fConst16: F32,
	fRec28: [F32;2],
	fConst17: F32,
	iConst18: i32,
	iRec33: [i32;2],
	iRec34: [i32;2],
	fRec32: [F32;2],
	fRec30: [F32;2],
	fRec31: [F32;2],
	fRec24: [F32;6],
	fRec23: [F32;6],
	fRec22: [F32;6],
	fRec18: [F32;6],
	fConst19: F32,
	fRec17: [F32;2],
	fButton2: F32,
	fEntry4: F32,
	fVec1: [F32;2],
	fRec16: [F32;2],
	fRec15: [F32;2],
	fRec14: [F32;32768],
	fConst20: F32,
	fVec2: [F32;2],
	fRec39: [F32;2],
	fRec38: [F32;2],
	fRec37: [F32;32768],
	fVec3: [F32;6],
	iConst21: i32,
	iConst22: i32,
	fConst23: F32,
	fConst24: F32,
	iConst25: i32,
	fConst26: F32,
	fRec36: [F32;6],
	fRec4: [F32;2],
	fRec5: [F32;2],
	fRec0: [F32;3],
	fConst27: F32,
}

impl FaustDsp for DSP_Piano {
	type T = F32;
		
	fn new() -> DSP_Piano { 
		DSP_Piano {
			fEntry0: 0.0,
			fHslider0: 0.0,
			fEntry1: 0.0,
			fEntry2: 0.0,
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			fConst2: 0.0,
			fButton0: 0.0,
			fVec0: [0.0;2],
			fConst3: 0.0,
			fRec7: [0.0;2],
			fHslider1: 0.0,
			fHslider2: 0.0,
			fHslider3: 0.0,
			fConst4: 0.0,
			IOTA0: 0,
			fConst5: 0.0,
			fConst6: 0.0,
			iConst7: 0,
			iConst8: 0,
			fConst9: 0.0,
			fConst10: 0.0,
			iConst11: 0,
			fConst12: 0.0,
			fHslider4: 0.0,
			fEntry3: 0.0,
			fButton1: 0.0,
			fConst13: 0.0,
			fRec26: [0.0;2],
			fConst14: 0.0,
			fConst15: 0.0,
			fRec25: [0.0;2],
			fConst16: 0.0,
			fRec28: [0.0;2],
			fConst17: 0.0,
			iConst18: 0,
			iRec33: [0;2],
			iRec34: [0;2],
			fRec32: [0.0;2],
			fRec30: [0.0;2],
			fRec31: [0.0;2],
			fRec24: [0.0;6],
			fRec23: [0.0;6],
			fRec22: [0.0;6],
			fRec18: [0.0;6],
			fConst19: 0.0,
			fRec17: [0.0;2],
			fButton2: 0.0,
			fEntry4: 0.0,
			fVec1: [0.0;2],
			fRec16: [0.0;2],
			fRec15: [0.0;2],
			fRec14: [0.0;32768],
			fConst20: 0.0,
			fVec2: [0.0;2],
			fRec39: [0.0;2],
			fRec38: [0.0;2],
			fRec37: [0.0;32768],
			fVec3: [0.0;6],
			iConst21: 0,
			iConst22: 0,
			fConst23: 0.0,
			fConst24: 0.0,
			iConst25: 0,
			fConst26: 0.0,
			fRec36: [0.0;6],
			fRec4: [0.0;2],
			fRec5: [0.0;2],
			fRec0: [0.0;3],
			fConst27: 0.0,
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("author", r"Romain Michon (rmichon@ccrma.stanford.edu)");
		m.declare("contributor", r"Punk Labs LLC");
		m.declare("basics.lib/downSample:author", r"Romain Michon");
		m.declare("basics.lib/name", r"Faust Basic Element Library");
		m.declare("basics.lib/sAndH:author", r"Romain Michon");
		m.declare("basics.lib/tabulate:author", r"Stephane Letz");
		m.declare("basics.lib/tabulateNd", r"Copyright (C) 2023 Bart Brouns <bart@magnetophon.nl>");
		m.declare("basics.lib/version", r"1.15.0");
		m.declare("compile_options", r"-a arch.rs -lang rust -ct 0 -cn DSP_Piano -dtl 65536 -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
		m.declare("copyright", r"Romain Michon");
		m.declare("delays.lib/name", r"Faust Delay Library");
		m.declare("delays.lib/version", r"1.1.0");
		m.declare("description", r"WaveGuide Commuted Piano");
		m.declare("filename", r"piano.dsp");
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
		m.declare("filters.lib/name", r"Faust Filters Library");
		m.declare("filters.lib/tf1:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf1:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf1:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/tf1s:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf1s:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf1s:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/tf2:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf2:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf2:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/version", r"1.3.0");
		m.declare("instruments.lib/author", r"Romain Michon (rmichon@ccrma.stanford.edu)");
		m.declare("instruments.lib/copyright", r"Romain Michon");
		m.declare("instruments.lib/licence", r"STK-4.3");
		m.declare("instruments.lib/name", r"Faust-STK Tools Library");
		m.declare("instruments.lib/version", r"1.0.0");
		m.declare("interpolators.lib/interpolate_linear:author", r"Stéphane Letz");
		m.declare("interpolators.lib/interpolate_linear:licence", r"MIT");
		m.declare("interpolators.lib/name", r"Faust Interpolator Library");
		m.declare("interpolators.lib/remap:author", r"David Braun");
		m.declare("interpolators.lib/version", r"1.3.1");
		m.declare("licence", r"STK-4.3");
		m.declare("maths.lib/author", r"GRAME");
		m.declare("maths.lib/copyright", r"GRAME");
		m.declare("maths.lib/license", r"LGPL with exception");
		m.declare("maths.lib/name", r"Faust Math Library");
		m.declare("maths.lib/version", r"2.8.0");
		m.declare("name", r"piano");
		m.declare("noises.lib/name", r"Faust Noise Generator Library");
		m.declare("noises.lib/version", r"1.4.1");
		m.declare("onetrick.lib/copyright", r"Copyright (c) 2023 Punk Labs LLC");
		m.declare("onetrick.lib/license", r"GPLv3 (or later)");
		m.declare("onetrick.lib/name", r"OneTrick DSP Library");
		m.declare("platform.lib/name", r"Generic Platform Library");
		m.declare("platform.lib/version", r"1.3.0");
		m.declare("signals.lib/name", r"Faust Signal Routing Library");
		m.declare("signals.lib/version", r"1.5.0");
		m.declare("version", r"1.0");
	}

	fn get_sample_rate(&self) -> i32 {
		return self.fSampleRate;
	}
	fn get_num_inputs(&self) -> i32 {
		return 0;
	}
	fn get_num_outputs(&self) -> i32 {
		return 1;
	}
	
	fn class_init(sample_rate: i32) {
		let mut sig0: DSP_PianoSIG0 = newDSP_PianoSIG0();
		sig0.instance_initDSP_PianoSIG0(sample_rate);
		sig0.fillDSP_PianoSIG0(12, unsafe { &mut ftbl0DSP_PianoSIG0 });
		let mut sig1: DSP_PianoSIG1 = newDSP_PianoSIG1();
		sig1.instance_initDSP_PianoSIG1(sample_rate);
		sig1.fillDSP_PianoSIG1(50, unsafe { &mut ftbl1DSP_PianoSIG1 });
		let mut sig2: DSP_PianoSIG2 = newDSP_PianoSIG2();
		sig2.instance_initDSP_PianoSIG2(sample_rate);
		sig2.fillDSP_PianoSIG2(50, unsafe { &mut ftbl2DSP_PianoSIG2 });
		let mut sig3: DSP_PianoSIG3 = newDSP_PianoSIG3();
		sig3.instance_initDSP_PianoSIG3(sample_rate);
		sig3.fillDSP_PianoSIG3(256, unsafe { &mut ftbl3DSP_PianoSIG3 });
		let mut sig4: DSP_PianoSIG4 = newDSP_PianoSIG4();
		sig4.instance_initDSP_PianoSIG4(sample_rate);
		sig4.fillDSP_PianoSIG4(50, unsafe { &mut ftbl4DSP_PianoSIG4 });
		let mut sig5: DSP_PianoSIG5 = newDSP_PianoSIG5();
		sig5.instance_initDSP_PianoSIG5(sample_rate);
		sig5.fillDSP_PianoSIG5(51, unsafe { &mut ftbl5DSP_PianoSIG5 });
		let mut sig6: DSP_PianoSIG6 = newDSP_PianoSIG6();
		sig6.instance_initDSP_PianoSIG6(sample_rate);
		sig6.fillDSP_PianoSIG6(51, unsafe { &mut ftbl6DSP_PianoSIG6 });
		let mut sig7: DSP_PianoSIG7 = newDSP_PianoSIG7();
		sig7.instance_initDSP_PianoSIG7(sample_rate);
		sig7.fillDSP_PianoSIG7(34, unsafe { &mut ftbl7DSP_PianoSIG7 });
		let mut sig8: DSP_PianoSIG8 = newDSP_PianoSIG8();
		sig8.instance_initDSP_PianoSIG8(sample_rate);
		sig8.fillDSP_PianoSIG8(50, unsafe { &mut ftbl8DSP_PianoSIG8 });
		let mut sig9: DSP_PianoSIG9 = newDSP_PianoSIG9();
		sig9.instance_initDSP_PianoSIG9(sample_rate);
		sig9.fillDSP_PianoSIG9(50, unsafe { &mut ftbl9DSP_PianoSIG9 });
		let mut sig10: DSP_PianoSIG10 = newDSP_PianoSIG10();
		sig10.instance_initDSP_PianoSIG10(sample_rate);
		sig10.fillDSP_PianoSIG10(50, unsafe { &mut ftbl10DSP_PianoSIG10 });
		let mut sig11: DSP_PianoSIG11 = newDSP_PianoSIG11();
		sig11.instance_initDSP_PianoSIG11(sample_rate);
		sig11.fillDSP_PianoSIG11(50, unsafe { &mut ftbl11DSP_PianoSIG11 });
		let mut sig12: DSP_PianoSIG12 = newDSP_PianoSIG12();
		sig12.instance_initDSP_PianoSIG12(sample_rate);
		sig12.fillDSP_PianoSIG12(13, unsafe { &mut ftbl12DSP_PianoSIG12 });
		let mut sig13: DSP_PianoSIG13 = newDSP_PianoSIG13();
		sig13.instance_initDSP_PianoSIG13(sample_rate);
		sig13.fillDSP_PianoSIG13(50, unsafe { &mut ftbl13DSP_PianoSIG13 });
		let mut sig14: DSP_PianoSIG14 = newDSP_PianoSIG14();
		sig14.instance_initDSP_PianoSIG14(sample_rate);
		sig14.fillDSP_PianoSIG14(10, unsafe { &mut ftbl14DSP_PianoSIG14 });
	}
	fn instance_reset_params(&mut self) {
		self.fEntry0 = 0.0;
		self.fHslider0 = 1.0;
		self.fEntry1 = 0.0;
		self.fEntry2 = 4.4e+02;
		self.fButton0 = 0.0;
		self.fHslider1 = 1e+02;
		self.fHslider2 = 2.75;
		self.fHslider3 = 15.0;
		self.fHslider4 = 5e+01;
		self.fEntry3 = 0.0;
		self.fButton1 = 0.0;
		self.fButton2 = 0.0;
		self.fEntry4 = 0.0;
	}
	fn instance_clear(&mut self) {
		for l3 in 0..2 {
			self.fVec0[l3 as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.fRec7[l4 as usize] = 0.0;
		}
		self.IOTA0 = 0;
		for l14 in 0..2 {
			self.fRec26[l14 as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.fRec25[l16 as usize] = 0.0;
		}
		for l18 in 0..2 {
			self.fRec28[l18 as usize] = 0.0;
		}
		for l19 in 0..2 {
			self.iRec33[l19 as usize] = 0;
		}
		for l20 in 0..2 {
			self.iRec34[l20 as usize] = 0;
		}
		for l21 in 0..2 {
			self.fRec32[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fRec30[l22 as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec31[l23 as usize] = 0.0;
		}
		for l24 in 0..6 {
			self.fRec24[l24 as usize] = 0.0;
		}
		for l25 in 0..6 {
			self.fRec23[l25 as usize] = 0.0;
		}
		for l26 in 0..6 {
			self.fRec22[l26 as usize] = 0.0;
		}
		for l27 in 0..6 {
			self.fRec18[l27 as usize] = 0.0;
		}
		for l28 in 0..2 {
			self.fRec17[l28 as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fVec1[l30 as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fRec16[l31 as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec15[l32 as usize] = 0.0;
		}
		for l33 in 0..32768 {
			self.fRec14[l33 as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fVec2[l34 as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fRec39[l35 as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fRec38[l36 as usize] = 0.0;
		}
		for l37 in 0..32768 {
			self.fRec37[l37 as usize] = 0.0;
		}
		for l38 in 0..6 {
			self.fVec3[l38 as usize] = 0.0;
		}
		for l39 in 0..6 {
			self.fRec36[l39 as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fRec4[l40 as usize] = 0.0;
		}
		for l41 in 0..2 {
			self.fRec5[l41 as usize] = 0.0;
		}
		for l42 in 0..3 {
			self.fRec0[l42 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		self.fConst1 = 6.2831855 / self.fConst0;
		self.fConst2 = 1.0 / self.fConst0;
		self.fConst3 = F32::exp(-(1e+02 / self.fConst0));
		self.fConst4 = 0.15915494 * self.fConst0;
		self.fConst5 = F32::max(1.0, 2.2675737e-05 * self.fConst0);
		self.fConst6 = self.fConst5 + -1.0;
		self.iConst7 = (self.fConst6) as i32;
		self.iConst8 = i32::wrapping_add(std::cmp::min(17, std::cmp::max(0, i32::wrapping_add(self.iConst7, 1))), 1);
		self.fConst9 = F32::floor(self.fConst6);
		self.fConst10 = self.fConst5 + (-1.0 - self.fConst9);
		self.iConst11 = i32::wrapping_add(std::cmp::min(17, std::cmp::max(0, self.iConst7)), 1);
		self.fConst12 = self.fConst9 + (2.0 - self.fConst5);
		self.fConst13 = 0.2 * self.fConst0;
		self.fConst14 = F32::exp(-(0.5 / self.fConst0));
		self.fConst15 = F32::exp(-(5.0 / self.fConst0));
		self.fConst16 = 7.0 / self.fConst0;
		self.fConst17 = self.fConst0 / F32::min(F32::min(4.8e+04, self.fConst0), self.fConst0);
		self.iConst18 = (self.fConst17) as i32;
		self.fConst19 = 3.1415927 / self.fConst0;
		self.fConst20 = 0.08533333 * self.fConst0 + 1.0;
		self.iConst21 = (self.fConst5) as i32;
		self.iConst22 = std::cmp::min(17, std::cmp::max(0, i32::wrapping_add(self.iConst21, 1)));
		self.fConst23 = F32::floor(self.fConst5);
		self.fConst24 = self.fConst5 - self.fConst23;
		self.iConst25 = std::cmp::min(17, std::cmp::max(0, self.iConst21));
		self.fConst26 = self.fConst23 + (1.0 - self.fConst5);
		self.fConst27 = 1.0 / DSP_Piano_faustpower2_f(self.fConst0);
	}
	fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	fn init(&mut self, sample_rate: i32) {
		DSP_Piano::class_init(sample_rate);
		self.instance_init(sample_rate);
	}
	
	fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
		Self::build_user_interface_static(ui_interface);
	}
	
	fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) {
		ui_interface.open_vertical_box("piano");
		ui_interface.add_num_entry("PitchWheel", ParamIndex(0), 0.0, -1.0, 1.0, 0.001);
		ui_interface.add_button("Sustain", ParamIndex(1));
		ui_interface.add_num_entry("Transpose", ParamIndex(2), 0.0, -48.0, 48.0, 0.001);
		ui_interface.add_button("WakeUp", ParamIndex(3));
		ui_interface.declare(Some(ParamIndex(4)), "010", "");
		ui_interface.declare(Some(ParamIndex(4)), "export", "Bend Range");
		ui_interface.declare(Some(ParamIndex(4)), "group", "MIDI");
		ui_interface.declare(Some(ParamIndex(4)), "type", "int");
		ui_interface.declare(Some(ParamIndex(4)), "unit", "st");
		ui_interface.add_horizontal_slider("Bend_Range", ParamIndex(4), 1.0, 1.0, 12.0, 1.0);
		ui_interface.declare(Some(ParamIndex(5)), "020", "");
		ui_interface.declare(Some(ParamIndex(5)), "export", "Sustain Lock");
		ui_interface.declare(Some(ParamIndex(5)), "group", "MIDI");
		ui_interface.declare(Some(ParamIndex(5)), "type", "bool");
		ui_interface.add_num_entry("Sustain_Lock", ParamIndex(5), 0.0, 0.0, 1.0, 1.0);
		ui_interface.declare(Some(ParamIndex(6)), "030", "");
		ui_interface.declare(Some(ParamIndex(6)), "export", "Brightness");
		ui_interface.declare(Some(ParamIndex(6)), "group", "Strings");
		ui_interface.declare(Some(ParamIndex(6)), "unit", "%");
		ui_interface.add_horizontal_slider("Brightness", ParamIndex(6), 5e+01, 0.0, 1e+02, 0.01);
		ui_interface.declare(Some(ParamIndex(7)), "040", "");
		ui_interface.declare(Some(ParamIndex(7)), "export", "In Tune");
		ui_interface.declare(Some(ParamIndex(7)), "group", "Strings");
		ui_interface.declare(Some(ParamIndex(7)), "unit", "%");
		ui_interface.add_horizontal_slider("In_Tune", ParamIndex(7), 1e+02, 0.0, 1e+02, 0.01);
		ui_interface.declare(Some(ParamIndex(8)), "050", "");
		ui_interface.declare(Some(ParamIndex(8)), "export", "Detuned");
		ui_interface.declare(Some(ParamIndex(8)), "group", "Strings");
		ui_interface.declare(Some(ParamIndex(8)), "unit", "%");
		ui_interface.add_horizontal_slider("Detuned", ParamIndex(8), 2.75, 0.0, 1e+02, 0.01);
		ui_interface.declare(Some(ParamIndex(9)), "220", "");
		ui_interface.declare(Some(ParamIndex(9)), "export", "Damp");
		ui_interface.declare(Some(ParamIndex(9)), "group", "Body");
		ui_interface.declare(Some(ParamIndex(9)), "unit", "%");
		ui_interface.add_horizontal_slider("Body_Damp", ParamIndex(9), 15.0, 0.0, 1e+02, 0.01);
		ui_interface.add_num_entry("freq", ParamIndex(10), 4.4e+02, 2e+01, 2e+04, 0.001);
		ui_interface.add_num_entry("gain", ParamIndex(11), 0.0, 0.0, 1.0, 0.01);
		ui_interface.add_button("gate", ParamIndex(12));
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			3 => Some(self.fButton0),
			12 => Some(self.fButton1),
			1 => Some(self.fButton2),
			0 => Some(self.fEntry0),
			2 => Some(self.fEntry1),
			10 => Some(self.fEntry2),
			11 => Some(self.fEntry3),
			5 => Some(self.fEntry4),
			4 => Some(self.fHslider0),
			7 => Some(self.fHslider1),
			8 => Some(self.fHslider2),
			9 => Some(self.fHslider3),
			6 => Some(self.fHslider4),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			3 => { self.fButton0 = value }
			12 => { self.fButton1 = value }
			1 => { self.fButton2 = value }
			0 => { self.fEntry0 = value }
			2 => { self.fEntry1 = value }
			10 => { self.fEntry2 = value }
			11 => { self.fEntry3 = value }
			5 => { self.fEntry4 = value }
			4 => { self.fHslider0 = value }
			7 => { self.fHslider1 = value }
			8 => { self.fHslider2 = value }
			9 => { self.fHslider3 = value }
			6 => { self.fHslider4 = value }
			_ => {}
		}
	}
	
	fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut[&mut[Self::T]]) {
		let (outputs0) = if let [outputs0, ..] = outputs {
			let outputs0 = outputs0[..count as usize].iter_mut();
			(outputs0)
		} else {
			panic!("wrong number of outputs");
		};
		let mut fSlow0: F32 = self.fEntry1 + self.fHslider0 * self.fEntry0;
		let mut fSlow1: F32 = F32::powf(2.0, 0.083333336 * fSlow0);
		let mut fSlow2: F32 = self.fEntry2;
		let mut fSlow3: F32 = fSlow2 * fSlow1;
		let mut fSlow4: F32 = 17.31234 * F32::log(0.0022727272 * fSlow3, std::f32::consts::E);
		let mut fSlow5: F32 = fSlow4 + 48.0;
		let mut fSlow6: F32 = 0.14102565 * fSlow5;
		let mut iSlow7: i32 = (fSlow6) as i32;
		let mut fSlow8: F32 = unsafe { ftbl0DSP_PianoSIG0[(std::cmp::max(0, std::cmp::min(iSlow7, 11))) as usize] };
		let mut fSlow9: F32 = F32::cos(self.fConst1 * fSlow3 / (fSlow8 + (fSlow6 - (iSlow7) as F32) * (unsafe { ftbl0DSP_PianoSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iSlow7, 1), 11))) as usize] } - fSlow8)));
		let mut fSlow10: F32 = 0.63919437 * fSlow5;
		let mut iSlow11: i32 = (fSlow10) as i32;
		let mut fSlow12: F32 = unsafe { ftbl1DSP_PianoSIG1[(std::cmp::max(0, std::cmp::min(iSlow11, 49))) as usize] };
		let mut fSlow13: F32 = fSlow12 + (fSlow10 - (iSlow11) as F32) * (unsafe { ftbl1DSP_PianoSIG1[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iSlow11, 1), 49))) as usize] } - fSlow12);
		let mut fSlow14: F32 = self.fConst2 * fSlow3;
		let mut fSlow15: F32 = 0.6282051 * fSlow5;
		let mut iSlow16: i32 = (fSlow15) as i32;
		let mut fSlow17: F32 = unsafe { ftbl2DSP_PianoSIG2[(std::cmp::max(0, std::cmp::min(iSlow16, 49))) as usize] };
		let mut fSlow18: F32 = fSlow17 + (fSlow15 - (iSlow16) as F32) * (unsafe { ftbl2DSP_PianoSIG2[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iSlow16, 1), 49))) as usize] } - fSlow17);
		let mut fSlow19: F32 = self.fButton0;
		let mut fSlow20: F32 = fSlow4 + 69.0;
		let mut iSlow21: i32 = (fSlow20) as i32;
		let mut fSlow22: F32 = unsafe { ftbl3DSP_PianoSIG3[(std::cmp::max(0, std::cmp::min(iSlow21, 255))) as usize] };
		let mut fSlow23: F32 = self.fHslider1;
		let mut fSlow24: F32 = F32::max(0.0, F32::powf(2.0, 0.020833334 * (1.0 - 0.01 * fSlow23)) + -1.0);
		let mut fSlow25: F32 = 1.0 / (fSlow24 + 1.0);
		let mut fSlow26: F32 = fSlow25 + 0.5 * (fSlow24 + (1.0 - fSlow25)) * (fSlow22 + (fSlow4 + (69.0 - (iSlow21) as F32)) * (unsafe { ftbl3DSP_PianoSIG3[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iSlow21, 1), 255))) as usize] } - fSlow22) + 1.0);
		let mut iSlow27: i32 = (fSlow4 + 197.0) as i32;
		let mut fSlow28: F32 = unsafe { ftbl3DSP_PianoSIG3[(std::cmp::max(0, std::cmp::min(iSlow27, 255))) as usize] };
		let mut fSlow29: F32 = F32::max(0.0, 0.005 * fSlow23);
		let mut fSlow30: F32 = 1.0 / (fSlow29 + 1.0);
		let mut fSlow31: F32 = 0.4537037 * fSlow20;
		let mut iSlow32: i32 = (fSlow31) as i32;
		let mut iSlow33: i32 = std::cmp::max(0, std::cmp::min(iSlow32, 49));
		let mut fSlow34: F32 = unsafe { ftbl4DSP_PianoSIG4[iSlow33 as usize] };
		let mut iSlow35: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iSlow32, 1), 49));
		let mut fSlow36: F32 = fSlow31 - (iSlow32) as F32;
		let mut fSlow37: F32 = 0.05 * self.fHslider2 * (fSlow34 + fSlow36 * (unsafe { ftbl4DSP_PianoSIG4[iSlow35 as usize] } - fSlow34)) * (fSlow30 + 0.5 * (fSlow29 + (1.0 - fSlow30)) * (fSlow28 + (fSlow4 + (197.0 - (iSlow27) as F32)) * (unsafe { ftbl3DSP_PianoSIG3[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iSlow27, 1), 255))) as usize] } - fSlow28) + 1.0));
		let mut fSlow38: F32 = 0.64102566 * fSlow5;
		let mut iSlow39: i32 = (fSlow38) as i32;
		let mut iSlow40: i32 = std::cmp::max(0, std::cmp::min(iSlow39, 50));
		let mut fSlow41: F32 = unsafe { ftbl5DSP_PianoSIG5[iSlow40 as usize] };
		let mut iSlow42: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iSlow39, 1), 50));
		let mut fSlow43: F32 = fSlow38 - (iSlow39) as F32;
		let mut fSlow44: F32 = fSlow41 + fSlow43 * (unsafe { ftbl5DSP_PianoSIG5[iSlow42 as usize] } - fSlow41);
		let mut fSlow45: F32 = unsafe { ftbl6DSP_PianoSIG6[iSlow40 as usize] };
		let mut fSlow46: F32 = F32::powf(1e+01, 0.05 / fSlow3 * (fSlow45 + fSlow43 * (unsafe { ftbl6DSP_PianoSIG6[iSlow42 as usize] } - fSlow45)));
		let mut fSlow47: F32 = (fSlow46 + (0.4 - 0.0038 * self.fHslider3) * (1.0 - fSlow46)) * (1.0 - fSlow44);
		let mut fSlow48: F32 = 0.42307693 * fSlow5;
		let mut iSlow49: i32 = (fSlow48) as i32;
		let mut fSlow50: F32 = unsafe { ftbl7DSP_PianoSIG7[(std::cmp::max(0, std::cmp::min(iSlow49, 33))) as usize] };
		let mut fSlow51: F32 = (fSlow48 - (iSlow49) as F32) * (unsafe { ftbl7DSP_PianoSIG7[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iSlow49, 1), 33))) as usize] } - fSlow50);
		let mut fSlow52: F32 = fSlow50 + fSlow51;
		let mut fSlow53: F32 = 1.0 - fSlow52;
		let mut fSlow54: F32 = 3.0 * fSlow53 - fSlow47;
		let mut fSlow55: F32 = fSlow3 * fSlow26;
		let mut fSlow56: F32 = self.fConst1 * (fSlow55 + fSlow37);
		let mut fSlow57: F32 = F32::cos(fSlow56);
		let mut fSlow58: F32 = fSlow47 * fSlow52;
		let mut fSlow59: F32 = fSlow44 * fSlow53;
		let mut fSlow60: F32 = fSlow59 - fSlow58;
		let mut fSlow61: F32 = 4.0 * fSlow60;
		let mut fSlow62: F32 = 3.0 * fSlow59;
		let mut fSlow63: F32 = fSlow58 - fSlow62;
		let mut fSlow64: F32 = fSlow63 + fSlow61;
		let mut fSlow65: F32 = fSlow51 + fSlow47 + fSlow50 + -1.0;
		let mut fSlow66: F32 = 4.0 * fSlow65;
		let mut fSlow67: F32 = (fSlow66 + fSlow64 * fSlow57) / fSlow54 + 1.0;
		let mut fSlow68: F32 = fSlow63 * fSlow57 / fSlow54 + 1.0;
		let mut fSlow69: F32 = F32::sin(fSlow56);
		let mut fSlow70: F32 = DSP_Piano_faustpower2_f(fSlow54);
		let mut fSlow71: F32 = DSP_Piano_faustpower2_f(fSlow69);
		let mut fSlow72: F32 = unsafe { ftbl8DSP_PianoSIG8[iSlow33 as usize] };
		let mut fSlow73: F32 = fSlow72 + fSlow36 * (unsafe { ftbl8DSP_PianoSIG8[iSlow35 as usize] } - fSlow72);
		let mut fSlow74: F32 = 0.25 * DSP_Piano_faustpower2_f(fSlow73);
		let mut fSlow75: F32 = fSlow74 + -1.0;
		let mut fSlow76: F32 = fSlow74 + 1.0;
		let mut fSlow77: F32 = 3.0 * F32::atan2(fSlow75 * fSlow69, fSlow73 + fSlow76 * fSlow57);
		let mut fSlow78: F32 = fSlow77 + F32::atan2(-(fSlow69 * (fSlow64 * fSlow68 - fSlow63 * fSlow67) / fSlow54), fSlow68 * fSlow67 + fSlow63 * fSlow64 * fSlow71 / fSlow70) + 6.2831855;
		let mut fSlow79: F32 = 0.58890694 * fSlow5;
		let mut iSlow80: i32 = (fSlow79) as i32;
		let mut fSlow81: F32 = unsafe { ftbl9DSP_PianoSIG9[(std::cmp::max(0, std::cmp::min(iSlow80, 49))) as usize] };
		let mut fSlow82: F32 = fSlow81 + (fSlow79 - (iSlow80) as F32) * (unsafe { ftbl9DSP_PianoSIG9[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iSlow80, 1), 49))) as usize] } - fSlow81);
		let mut fSlow83: F32 = 0.5653107 * fSlow5;
		let mut iSlow84: i32 = (fSlow83) as i32;
		let mut fSlow85: F32 = unsafe { ftbl10DSP_PianoSIG10[(std::cmp::max(0, std::cmp::min(iSlow84, 49))) as usize] };
		let mut fSlow86: F32 = self.fEntry3;
		let mut fSlow87: F32 = fSlow82 + fSlow86 * (fSlow85 + (fSlow83 - (iSlow84) as F32) * (unsafe { ftbl10DSP_PianoSIG10[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iSlow84, 1), 49))) as usize] } - fSlow85) + (0.02 - fSlow82) - 0.25 * (0.0025 * self.fHslider4 + -0.05));
		let mut fSlow88: F32 = 0.627184 * (fSlow4 + 47.127);
		let mut iSlow89: i32 = (fSlow88) as i32;
		let mut fSlow90: F32 = unsafe { ftbl11DSP_PianoSIG11[(std::cmp::max(0, std::cmp::min(iSlow89, 49))) as usize] };
		let mut fSlow91: F32 = fSlow90 + (fSlow88 - (iSlow89) as F32) * (unsafe { ftbl11DSP_PianoSIG11[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iSlow89, 1), 49))) as usize] } - fSlow90);
		let mut fSlow92: F32 = 1.0 - fSlow87;
		let mut fSlow93: F32 = self.fButton1;
		let mut iSlow94: i32 = (fSlow93 > 0.0) as i32;
		let mut fSlow95: F32 = 0.13793103 * fSlow5;
		let mut iSlow96: i32 = (fSlow95) as i32;
		let mut fSlow97: F32 = unsafe { ftbl12DSP_PianoSIG12[(std::cmp::max(0, std::cmp::min(iSlow96, 12))) as usize] };
		let mut fSlow98: F32 = fSlow97 + (fSlow95 - (iSlow96) as F32) * (unsafe { ftbl12DSP_PianoSIG12[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iSlow96, 1), 12))) as usize] } - fSlow97);
		let mut fSlow99: F32 = self.fConst15 * (fSlow93 + -1.0);
		let mut fSlow100: F32 = 0.56846523 * (fSlow4 + 47.999);
		let mut iSlow101: i32 = (fSlow100) as i32;
		let mut fSlow102: F32 = unsafe { ftbl13DSP_PianoSIG13[(std::cmp::max(0, std::cmp::min(iSlow101, 49))) as usize] };
		let mut fSlow103: F32 = F32::exp(-(self.fConst16 / fSlow86 / (fSlow102 + (fSlow100 - (iSlow101) as F32) * (unsafe { ftbl13DSP_PianoSIG13[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iSlow101, 1), 49))) as usize] } - fSlow102))));
		let mut iSlow104: i32 = (fSlow93 < 1.0) as i32;
		let mut fSlow105: F32 = 1.0 / F32::tan(self.fConst19 * fSlow3);
		let mut fSlow106: F32 = 1.0 - fSlow105;
		let mut fSlow107: F32 = 1.0 / (fSlow105 + 1.0);
		let mut fSlow108: F32 = 0.115384616 * fSlow5;
		let mut iSlow109: i32 = (fSlow108) as i32;
		let mut fSlow110: F32 = unsafe { ftbl14DSP_PianoSIG14[(std::cmp::max(0, std::cmp::min(iSlow109, 9))) as usize] };
		let mut fSlow111: F32 = F32::max(fSlow93, F32::max(self.fButton2, self.fEntry4));
		let mut fSlow112: F32 = fSlow111 + 0.9 * (1.0 - fSlow111) * (fSlow110 + (fSlow108 - (iSlow109) as F32) * (unsafe { ftbl14DSP_PianoSIG14[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iSlow109, 1), 9))) as usize] } - fSlow110));
		let mut fSlow113: F32 = fSlow58 + fSlow61 - fSlow62;
		let mut fSlow114: F32 = (fSlow66 + fSlow113 * fSlow57) / fSlow54 + 1.0;
		let mut fSlow115: F32 = fSlow77 + F32::atan2(-(fSlow69 * (fSlow113 * fSlow68 - fSlow63 * fSlow114) / fSlow54), fSlow68 * fSlow114 + fSlow63 * fSlow113 * fSlow71 / fSlow70) + 6.2831855;
		let mut fSlow116: F32 = self.fConst1 * (fSlow55 - fSlow37);
		let mut fSlow117: F32 = F32::cos(fSlow116);
		let mut fSlow118: F32 = (fSlow66 + fSlow117 * fSlow113) / fSlow54 + 1.0;
		let mut fSlow119: F32 = fSlow117 * fSlow63 / fSlow54 + 1.0;
		let mut fSlow120: F32 = F32::sin(fSlow116);
		let mut fSlow121: F32 = DSP_Piano_faustpower2_f(fSlow120) * fSlow63;
		let mut fSlow122: F32 = 3.0 * F32::atan2(fSlow75 * fSlow120, fSlow73 + fSlow76 * fSlow117);
		let mut fSlow123: F32 = fSlow122 + F32::atan2(-(fSlow120 * (fSlow119 * fSlow113 - fSlow63 * fSlow118) / fSlow54), fSlow119 * fSlow118 + fSlow121 * fSlow113 / fSlow70) + 6.2831855;
		let mut fSlow124: F32 = (fSlow117 * fSlow64 + fSlow66) / fSlow54 + 1.0;
		let mut fSlow125: F32 = fSlow122 + F32::atan2(-(fSlow120 * (fSlow64 * fSlow119 - fSlow63 * fSlow124) / fSlow54), fSlow119 * fSlow124 + fSlow121 * fSlow64 / fSlow70) + 6.2831855;
		let mut fSlow126: F32 = 1.0 - self.fConst27 * DSP_Piano_faustpower2_f(fSlow2) * DSP_Piano_faustpower2_f(fSlow1) * DSP_Piano_faustpower2_f(fSlow13);
		let zipped_iterators = outputs0;
		for output0 in zipped_iterators {
			self.fVec0[0] = fSlow19;
			let mut fTemp24: F32 = ((fSlow19 <= self.fVec0[1]) as i32) as u32 as F32;
			self.fRec7[0] = fSlow0 * (1.0 - self.fConst3 * fTemp24) + self.fConst3 * fTemp24 * self.fRec7[1];
			let mut fTemp25: F32 = fSlow2 * fSlow26 * F32::powf(2.0, 0.083333336 * self.fRec7[0]);
			let mut fTemp40: F32 = fSlow37 + fTemp25;
			let mut fTemp82: F32 = self.fConst4 * (fSlow78 / fTemp40);
			let mut fTemp83: F32 = F32::floor(fTemp82 + -1.0);
			self.fRec26[0] = fSlow93 * self.fRec26[1] + 1.0;
			let mut fTemp127: F32 = self.fRec26[0] + -1.0;
			let mut iTemp128: i32 = (fTemp127 < self.fConst13) as i32;
			let mut fTemp138: F32 = fSlow93 * (self.fConst15 * (iTemp128) as F32 + self.fConst14 * ((fTemp127 >= self.fConst13) as i32) as u32 as F32);
			self.fRec25[0] = self.fRec25[1] * (fTemp138 - fSlow99) + 0.2 * (fSlow99 + (1.0 - fTemp138)) * fSlow98 * (iTemp128 & iSlow94) as F32;
			let mut fTemp152: F32 = (((fTemp127 < 2.0) as i32) & iSlow94) as F32;
			let mut fTemp153: F32 = 0.030197384 * fTemp152 + (((fTemp127 >= 2.0) as i32) | iSlow104) as F32 * fSlow103;
			self.fRec28[0] = self.fRec28[1] * fTemp153 + 0.15 * fTemp152 * (1.0 - fTemp153);
			self.iRec33[0] = i32::wrapping_add(self.iRec33[1], 1);
			self.iRec34[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec34[1]), 12345);
			self.fRec32[0] = if (((i32::wrapping_add(self.iRec33[0], -1)) % self.iConst18) == 0) as i32 != 0 {4.656613e-10 * (self.iRec34[0]) as F32} else {self.fRec32[1]};
			let mut fTemp154: F32 = if (self.fRec32[0] != self.fRec32[1]) as i32 != 0 {self.fConst17} else {self.fRec30[1] + -1.0};
			self.fRec30[0] = fTemp154;
			self.fRec31[0] = if (fTemp154 > 0.0) as i32 != 0 {self.fRec31[1] + (self.fRec32[0] - self.fRec31[1]) / fTemp154} else {self.fRec32[0]};
			self.fRec24[0] = 3.0 * self.fRec31[0] * fSlow92 * fSlow91 * (self.fRec28[0] + self.fRec25[0]) + fSlow87 * (self.fConst12 * self.fRec24[self.iConst11 as usize] + self.fConst10 * self.fRec24[self.iConst8 as usize]);
			self.fRec23[0] = self.fRec24[0] * fSlow92 * fSlow91 + fSlow87 * (self.fConst12 * self.fRec23[self.iConst11 as usize] + self.fConst10 * self.fRec23[self.iConst8 as usize]);
			self.fRec22[0] = self.fRec23[0] * fSlow92 * fSlow91 + fSlow87 * (self.fConst12 * self.fRec22[self.iConst11 as usize] + self.fConst10 * self.fRec22[self.iConst8 as usize]);
			self.fRec18[0] = self.fRec22[0] * fSlow92 * fSlow91 + fSlow87 * (self.fConst12 * self.fRec18[self.iConst11 as usize] + self.fConst10 * self.fRec18[self.iConst8 as usize]);
			self.fRec17[0] = -(fSlow107 * (fSlow106 * self.fRec17[1] - fSlow105 * (self.fRec18[0] - self.fRec18[1])));
			let mut fTemp166: F32 = fSlow112 * (self.fRec17[0] + self.fRec4[1]);
			self.fVec1[0] = fTemp166;
			self.fRec16[0] = self.fVec1[1] + 0.5 * fSlow73 * (fTemp166 - self.fRec16[1]);
			self.fRec15[0] = self.fRec16[1] + 0.5 * fSlow73 * (self.fRec16[0] - self.fRec15[1]);
			self.fRec14[(self.IOTA0 & 32767) as usize] = self.fRec15[1] + 0.5 * fSlow73 * (self.fRec15[0] - self.fRec14[((i32::wrapping_sub(self.IOTA0, 1)) & 32767) as usize]);
			let mut iTemp167: i32 = (self.fConst4 * (fSlow115 / fTemp40) + -1.0) as i32;
			let mut fTemp168: F32 = self.fRec14[((i32::wrapping_sub(self.IOTA0, (F32::min(self.fConst20, (std::cmp::max(0, iTemp167)) as F32)) as i32)) & 32767) as usize] * (fTemp83 + (2.0 - fTemp82));
			let mut fTemp169: F32 = self.fRec17[0] + self.fRec5[1] * fSlow112;
			self.fVec2[0] = fTemp169;
			self.fRec39[0] = self.fVec2[1] + 0.5 * fSlow73 * (fTemp169 - self.fRec39[1]);
			self.fRec38[0] = self.fRec39[1] + 0.5 * fSlow73 * (self.fRec39[0] - self.fRec38[1]);
			self.fRec37[(self.IOTA0 & 32767) as usize] = self.fRec38[1] + 0.5 * fSlow73 * (self.fRec38[0] - self.fRec37[((i32::wrapping_sub(self.IOTA0, 1)) & 32767) as usize]);
			let mut fTemp170: F32 = fTemp25 - fSlow37;
			let mut iTemp171: i32 = (self.fConst4 * (fSlow123 / fTemp170) + -1.0) as i32;
			let mut fTemp172: F32 = self.fConst4 * (fSlow125 / fTemp170);
			let mut fTemp173: F32 = F32::floor(fTemp172 + -1.0);
			let mut fTemp174: F32 = (fTemp172 + (-1.0 - fTemp173)) * self.fRec37[((i32::wrapping_sub(self.IOTA0, (F32::min(self.fConst20, (std::cmp::max(0, i32::wrapping_add(iTemp171, 1))) as F32)) as i32)) & 32767) as usize];
			let mut fTemp175: F32 = self.fRec37[((i32::wrapping_sub(self.IOTA0, (F32::min(self.fConst20, (std::cmp::max(0, iTemp171)) as F32)) as i32)) & 32767) as usize] * (fTemp173 + (2.0 - fTemp172));
			let mut fTemp176: F32 = (fTemp82 + (-1.0 - fTemp83)) * self.fRec14[((i32::wrapping_sub(self.IOTA0, (F32::min(self.fConst20, (std::cmp::max(0, i32::wrapping_add(iTemp167, 1))) as F32)) as i32)) & 32767) as usize];
			let mut fTemp177: F32 = fTemp176 + fTemp175 + fTemp174 + fTemp168;
			self.fVec3[0] = fTemp177;
			self.fRec36[0] = (2.0 * (fSlow65 * fTemp177 + fSlow60 * (self.fConst26 * self.fVec3[self.iConst25 as usize] + self.fConst24 * self.fVec3[self.iConst22 as usize])) - fSlow63 * (self.fConst12 * self.fRec36[self.iConst11 as usize] + self.fConst10 * self.fRec36[self.iConst8 as usize])) / fSlow54;
			self.fRec4[0] = fTemp176 + self.fRec36[0] + fTemp168;
			self.fRec5[0] = fTemp174 + self.fRec36[0] + fTemp175;
			let mut fRec6: F32 = fTemp177;
			self.fRec0[0] = fRec6 * fSlow18 - fSlow14 * fSlow13 * (fSlow14 * fSlow13 * self.fRec0[2] - 2.0 * self.fRec0[1] * fSlow9);
			*output0 = 3.5481339 * (fRec6 + 0.5 * fSlow126 * (self.fRec0[0] - self.fRec0[2]));
			self.fVec0[1] = self.fVec0[0];
			self.fRec7[1] = self.fRec7[0];
			self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
			self.fRec26[1] = self.fRec26[0];
			self.fRec25[1] = self.fRec25[0];
			self.fRec28[1] = self.fRec28[0];
			self.iRec33[1] = self.iRec33[0];
			self.iRec34[1] = self.iRec34[0];
			self.fRec32[1] = self.fRec32[0];
			self.fRec30[1] = self.fRec30[0];
			self.fRec31[1] = self.fRec31[0];
			for j0 in (1..=5).rev() {
				self.fRec24[j0 as usize] = self.fRec24[(i32::wrapping_sub(j0, 1)) as usize];
			}
			for j1 in (1..=5).rev() {
				self.fRec23[j1 as usize] = self.fRec23[(i32::wrapping_sub(j1, 1)) as usize];
			}
			for j2 in (1..=5).rev() {
				self.fRec22[j2 as usize] = self.fRec22[(i32::wrapping_sub(j2, 1)) as usize];
			}
			for j3 in (1..=5).rev() {
				self.fRec18[j3 as usize] = self.fRec18[(i32::wrapping_sub(j3, 1)) as usize];
			}
			self.fRec17[1] = self.fRec17[0];
			self.fVec1[1] = self.fVec1[0];
			self.fRec16[1] = self.fRec16[0];
			self.fRec15[1] = self.fRec15[0];
			self.fVec2[1] = self.fVec2[0];
			self.fRec39[1] = self.fRec39[0];
			self.fRec38[1] = self.fRec38[0];
			for j4 in (1..=5).rev() {
				self.fVec3[j4 as usize] = self.fVec3[(i32::wrapping_sub(j4, 1)) as usize];
			}
			for j5 in (1..=5).rev() {
				self.fRec36[j5 as usize] = self.fRec36[(i32::wrapping_sub(j5, 1)) as usize];
			}
			self.fRec4[1] = self.fRec4[0];
			self.fRec5[1] = self.fRec5[0];
			self.fRec0[2] = self.fRec0[1];
			self.fRec0[1] = self.fRec0[0];
		}
	}

}

