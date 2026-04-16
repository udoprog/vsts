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

use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::Arc;

#[cfg(feature = "profile")]
use std::sync::atomic::AtomicI32;

use nih_plug::prelude::*;

#[cfg(feature = "egui")]
use nih_plug_egui::{
    create_egui_editor,
    egui::{
        epaint::Shadow, pos2, style, vec2, Align2, Area, CentralPanel, Color32, Context,
        CornerRadius, FontDefinitions, FontFamily, FontId, Frame, Id, Margin, Order, Rect, Sense,
        Stroke, UiBuilder,
    },
    EguiState,
};

mod dsp;
use onetrick::prelude::*;

//256 is 187.5Hz at 48kHz
//512 is 94Hz at 48kHz
//1024 is 47Hz at 48kHz
const MAX_BLOCK_SIZE: usize = 4096;
const NUM_CHANNELS: u32 = 2;

#[cfg(feature = "monophonic")]
const MAX_VOICES: usize = 1;
#[cfg(not(feature = "monophonic"))]
const MAX_VOICES: usize = 16;

macro_rules! for_each_voice_dsp {
    ($this:ident, $expression:expr) => {{
        $this.dsp_voices.borrow_mut().for_each_voice(|voice| {
            $expression(&mut *voice);
        })
    }};
}

macro_rules! for_each_dsp {
    ($this:ident, $expression:expr) => {{
        for_each_voice_dsp!($this, $expression);
        $expression(&mut *$this.dsp_output.borrow_mut());
    }};
}

/// Main Struct for OneTrick Keys
pub struct OneTrickKeys {
    params: Arc<OneTrickPluginParams>,

    accum_buffer: ResizableBuffer,

    dsp_output: RefCell<OneTrickDSP<dsp::modules::DSP_Output>>,
    dsp_voices: RefCell<OneTrickVoices<dsp::modules::DSP_Piano>>,

    param_global_sensitivity: usize,
    piano_indicator: Arc<[AtomicU8; 16]>,

    #[cfg(feature = "profile")]
    dsp_cost: Arc<AtomicI32>,
    sample_rate: usize,
}

impl Default for OneTrickKeys {
    fn default() -> Self {
        let mut params = OneTrickPluginParams::default();
        #[cfg(feature = "egui")]
        {
            params.editor_state = EguiState::from_size(950, 725);
        }

        let mut dsp_output = OneTrickDSP::new();
        params.append_dsp(&mut dsp_output, "");

        let mut dsp_voices = OneTrickVoices::new(MAX_VOICES);
        dsp_voices.for_each_voice(|voice| {
            params.append_dsp(voice, "");
        });

        let param_global_sensitivity = params.append_float(
            FloatParam::new(
                "MIDI Sensitivity",
                100.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 100.0,
                },
            )
            .with_unit("%")
            .with_value_to_string(Arc::new(|v| {
                format!("{value:.precision$}", precision = 1, value = v)
            })),
            "MIDI",
        );

        Self {
            //params: Arc::new(OneTrickPluginParams::default()),
            params: Arc::new(params),

            accum_buffer: ResizableBuffer::default(),

            dsp_output: RefCell::new(dsp_output),
            dsp_voices: RefCell::new(dsp_voices),

            param_global_sensitivity,
            //param_global_transpose,
            piano_indicator: Arc::default(),

            #[cfg(feature = "profile")]
            dsp_cost: Arc::new(AtomicI32::new(0)),

            sample_rate: 1, // Invalid until initialized
        }
    }
}

impl OneTrickKeys {
    /// Set up fonts and default styles for Egui
    #[cfg(feature = "egui")]
    fn setup_egui_style(ctx: &Context) {
        // Start with the default fonts (we will be adding to them rather than replacing them).
        let mut fonts = FontDefinitions::default();

        BasicFonts::add_fonts(&mut fonts);
        Icons::add_fonts(&mut fonts);

        let mut style = style::Style::default();

        // Setup defaults for using generic controls:
        let font_id = FontId::new(17.0, FontFamily::Proportional);
        style.override_font_id = Some(font_id);
        //style.override_text_style = Some(TextStyle::Body);

        //style.widgets.noninteractive.bg_stroke =
        //    Stroke::new(2.0, palette.grey().alpha(0.666).to_color32());

        // Set the base Style
        style.visuals = style::Visuals::dark();

        // Tooltips and Popup Windows:
        style.visuals.window_fill = Color32::DARK_GRAY;
        style.visuals.window_stroke = Stroke::default();
        style.visuals.popup_shadow = Shadow {
            offset: [0, 0],
            blur: 6,
            spread: 0,
            color: Color32::from_black_alpha(32),
        };
        style.visuals.override_text_color = Some(Color32::WHITE);

        style.visuals.selection.stroke = Stroke::new(2.0, Color32::WHITE);
        ctx.set_style(style);

        // Tell egui to use these fonts:
        ctx.set_fonts(fonts);
    }

    fn set_internal_params(&mut self) {}
}

impl Plugin for OneTrickKeys {
    const NAME: &'static str = "OneTrick KEYS";
    const VENDOR: &'static str = "Punk Labs";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "contact@punklabs.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        // Stereo
        main_input_channels: NonZeroU32::new(0),
        main_output_channels: NonZeroU32::new(NUM_CHANNELS),
        names: PortNames {
            layout: Some("Stereo"),

            main_input: None,
            main_output: Some("Output"),
            aux_inputs: &[],
            aux_outputs: &[],
        },
        ..AudioIOLayout::const_default()
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::MidiCCs; //Basic or MidiCCs
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = false; // Too bad this doesn't split on Midi NoteOn events

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // Resize buffers and perform other potentially expensive initialization operations here.
        // The `reset()` function is always called right after this function. You can remove this
        // function if you do not need it.

        let sample_rate = buffer_config.sample_rate as usize;

        self.accum_buffer.resize(2, MAX_BLOCK_SIZE);

        //nih_log!("Sample Rate: {}", sample_rate);
        self.sample_rate = sample_rate;
        const SILENCE_MS: u32 = 100;
        for_each_dsp!(self, &mut |dsp: &mut dyn OneTrickDSPGeneral| {
            dsp.initialize(sample_rate)
                .resize_buffer(MAX_BLOCK_SIZE)
                .track_silence(SILENCE_MS);
        });

        self.set_internal_params();

        true
    }

    fn reset(&mut self) {
        // Reset buffers and envelopes here. This can be called from the audio thread and may not
        // allocate. You can remove this function if you do not need it.
        // for_each_dsp!(self, &mut |dsp: &mut dyn OneTrickDSPGeneral| {
        //     dsp.reset();
        // });
        self.dsp_output.borrow_mut().reset();
        self.dsp_voices.borrow_mut().reset();

        for (_index, indicator) in self.piano_indicator.iter().enumerate() {
            indicator.store(0, Ordering::Relaxed);
        }

        self.set_internal_params();
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        #[cfg(feature = "profile")]
        let dsp_timer = std::time::Instant::now();

        let _is_standalone = match context.plugin_api() {
            PluginApi::Standalone => true,
            _ => false,
        };

        ProcessEx::process_split_notes(
            buffer,
            aux,
            MAX_BLOCK_SIZE,
            context,
            |event| {
                // MIDI
                match event {
                    NoteEvent::MidiPitchBend {
                        timing: _,
                        channel: _,
                        value,
                    } => {
                        self.dsp_voices.borrow_mut().pitch_wheel(value);
                    }
                    NoteEvent::MidiCC {
                        timing: _,
                        channel: _,
                        cc,
                        value,
                    } => {
                        if let Some(cc) = MidiCC::try_from(cc).ok() {
                            match cc {
                                MidiCC::ModWheel => {
                                    #[cfg(feature = "fake_sustain")]
                                    self.dsp_voices.borrow_mut().sustain(value > 0.5);
                                    #[cfg(not(feature = "fake_sustain"))]
                                    self.dsp_output.borrow_mut().mod_wheel(value);
                                }
                                MidiCC::Sustain => {
                                    self.dsp_voices.borrow_mut().sustain(value > 0.5);
                                }
                                MidiCC::Sostenuto => {
                                    self.dsp_voices.borrow_mut().sostenuto(value > 0.5);
                                }
                                _ => {}
                            }
                        }
                    }
                    NoteEvent::NoteOn {
                        timing: _,
                        voice_id,
                        channel: _,
                        note,
                        velocity,
                    } => {
                        // Piano range is A0->C8
                        // Safe range with trebble dsp: A0->G6, Gs6 feels unstable
                        // Safe range without trebble dsp: A0->C8
                        // Might have to disable trebble dsp for now...
                        if note >= MidiNote::A0 as u8 && note <= MidiNote::C8 as u8 {
                            //nih_log!("{}", note);
                            // Explodes at MidiNote::A6... (1760hz)
                            let velocity = if let Some(p) =
                                self.params.param_float_at(self.param_global_sensitivity)
                            {
                                let sensitivity = p.value() * 0.01;
                                velocity * sensitivity + (1.0 - sensitivity) * (80.0 / 127.0)
                            } else {
                                velocity
                            };

                            if self
                                .dsp_voices
                                .borrow_mut()
                                .note_on(note, velocity, voice_id)
                            {
                                self.dsp_output.borrow_mut().wake_up();
                            }

                            self.dsp_output
                                .borrow_mut()
                                .hold(self.dsp_voices.borrow().active_voice_count() > 0);

                            let voices = self.dsp_voices.borrow();
                            for (index, indicator) in self.piano_indicator.iter().enumerate() {
                                let note = if index < voices.active_voice_count() {
                                    voices.active_voices()[index]
                                        .get_last_note_u8()
                                        .unwrap_or(0)
                                } else {
                                    0
                                };
                                indicator.store(note, Ordering::Relaxed);
                            }
                            //self.piano_indicator[0].store(self.dsp_voices.borrow().active_voice_count() as u8, Ordering::Relaxed);
                        }
                    }
                    NoteEvent::NoteOff {
                        timing: _,
                        voice_id,
                        channel: _,
                        note,
                        velocity: _,
                    } => {
                        self.dsp_voices.borrow_mut().note_off(note, voice_id);
                        let voices = self.dsp_voices.borrow();

                        self.dsp_output
                            .borrow_mut()
                            .hold(self.dsp_voices.borrow().active_voice_count() > 0);

                        for (index, indicator) in self.piano_indicator.iter().enumerate() {
                            let note = if index < voices.active_voice_count() {
                                voices.active_voices()[index]
                                    .get_last_note_u8()
                                    .unwrap_or(0)
                            } else {
                                0
                            };
                            indicator.store(note, Ordering::Relaxed);
                        }
                    }
                    _ => (),
                }
            },
            #[allow(unused_variables)]
            &mut |buffer: &mut Buffer, aux: &mut AuxiliaryBuffers, block_start, block_end| // AUDIO
            {
                //let frames = output[0].len() as i32;
                let frames = block_end - block_start;

                // Attempt to process audio in blocks split by events:
                let output = &mut buffer.slice2ch_range_mut(block_start, block_end);
                for channel in output.iter_mut() {
                    channel.fill(0.0);
                }

                //self.accum_buffer.clear();
                self.accum_buffer.buffer.clear_frames(frames);

                for_each_voice_dsp!(self, &mut |dsp: &mut dyn OneTrickDSPGeneral| {
                    // Compute Voice:
                    if dsp.is_active() {
                        //dsp.transpose(pitch_shift);
                        dsp.compute(frames, &[]);
                    } else {
                        dsp.skip_compute();
                    }
                    // Accumulate Voice:
                    dsp.add_to_buffer(&mut self.accum_buffer.buffer);
                });

                // Process Main Output:
                {
                    let mut dsp = self.dsp_output.borrow_mut();
                    if dsp.is_active() {
                        dsp.compute_to(frames, &self.accum_buffer.buffer.as_slice_actually_immutable(), Some(output));
                    } else {
                        dsp.skip_compute();
                    }
                }

            },
        );

        #[cfg(feature = "profile")]
        {
            let sec: f64 = dsp_timer.elapsed().as_secs_f64().try_into().unwrap();
            let max_sec: f64 = buffer.samples() as f64 / self.sample_rate as f64;
            let ratio_int = (sec / max_sec * 100. * 100000.) as i32; // Put it into %, then 5 decimals of precision
                                                                     // VU-style falling:
            let old_ratio_int = self.dsp_cost.load(Ordering::Relaxed);
            let ratio_int = ratio_int.max(old_ratio_int - 2000);
            self.dsp_cost.store(ratio_int, Ordering::Relaxed);
        }

        ProcessStatus::Normal
    }

    #[cfg(feature = "egui")]
    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        //async_executor.execute_gui((||{nih_log!("Hello from the GUI")})());
        //async_executor.execute_background((||{nih_log!("Hello from a Background Thread")})());

        let params = self.params.clone();
        let mut preset_manager =
            PresetManager::new("OneTrick KEYS").with_excluded_params(&["Sustain Lock"]);
        let default_preset = Preset::from_param_defaults("Basic Keys", &params);
        preset_manager.add_factory(default_preset);
        preset_manager.add_factory_string(
            "Grand Piano",
            include_str!("assets/presets/Grand Piano.preset"),
        );
        preset_manager.add_factory_string(
            "Concert Hall",
            include_str!("assets/presets/Concert Hall.preset"),
        );
        preset_manager.add_factory_string("Honky", include_str!("assets/presets/Honky.preset"));
        preset_manager.add_factory_string(
            "Lofi Hiphop Radio",
            include_str!("assets/presets/Lofi Hiphop Radio.preset"),
        );
        preset_manager.add_factory_string(
            "Cassette Memories",
            include_str!("assets/presets/Cassette Memories.preset"),
        );
        preset_manager.add_factory_string(
            "Murder House",
            include_str!("assets/presets/Murder House.preset"),
        );
        preset_manager.add_factory_string(
            "Defunct Studio",
            include_str!("assets/presets/Defunct Studio.preset"),
        );
        preset_manager.add_factory_string(
            "Distant Memory",
            include_str!("assets/presets/Distant Memory.preset"),
        );
        preset_manager.add_factory_string(
            "Home Recording",
            include_str!("assets/presets/Home Recording.preset"),
        );
        preset_manager
            .add_factory_string("Jazz Bar", include_str!("assets/presets/Jazz Bar.preset"));
        preset_manager.add_factory_string(
            "Auditorium",
            include_str!("assets/presets/Auditorium.preset"),
        );
        preset_manager
            .add_factory_string("Uplifting", include_str!("assets/presets/Uplifting.preset"));
        preset_manager
            .add_factory_string("Regretful", include_str!("assets/presets/Regretful.preset"));
        preset_manager
            .add_factory_string("Monogram", include_str!("assets/presets/Monogram.preset"));
        preset_manager
            .add_factory_string("Cute Toy", include_str!("assets/presets/Cute Toy.preset"));
        preset_manager
            .add_factory_string("Fireside", include_str!("assets/presets/Fireside.preset"));
        preset_manager.add_factory_string(
            "Lost at Sea",
            include_str!("assets/presets/Lost at Sea.preset"),
        );
        preset_manager
            .add_factory_string("Mournful", include_str!("assets/presets/Mournful.preset"));
        preset_manager
            .add_factory_string("Peacetime", include_str!("assets/presets/Peacetime.preset"));
        preset_manager.add_factory_string(
            "So-So Sorry",
            include_str!("assets/presets/So-So Sorry.preset"),
        );
        preset_manager.add_factory_string(
            "Upright Citizen",
            include_str!("assets/presets/Upright Citizen.preset"),
        );
        preset_manager.add_factory_string(
            "Midsummer Sonata",
            include_str!("assets/presets/Midsummer Sonata.preset"),
        );
        preset_manager.add_factory_string(
            "Autumn Rain",
            include_str!("assets/presets/Autumn Rain.preset"),
        );
        preset_manager.add_factory_string(
            "Cold of Winter",
            include_str!("assets/presets/Cold of Winter.preset"),
        );
        preset_manager.add_factory_string("Dirge", include_str!("assets/presets/Dirge.preset"));

        preset_manager.refresh();

        let palette = Palette::new(8)
            .shades(6)
            .white_level(0.9)
            .black_level(0.15)
            .saturation(0.75)
            .shift(0.49)
            .alt_hue_step(4.533)
            .dark_shift(0.1)
            .dark_desaturation(0.5);
        //let palette_alt = palette.alternate(1.5);

        let mut knob_style = ParamKnobStyle {
            radius: 35.0,
            indicator_style: ParamKnobIndicatorStyle::Continuous,
            ..Default::default()
        };
        let column_width = knob_style.required_width();

        let mut heading_style = LabelStyle::default_heading();
        heading_style.width = Some(column_width);
        heading_style.color = palette.black().into();
        heading_style.bg_color = Color32::TRANSPARENT;
        heading_style.shadow = None;
        let mut subheading_style = heading_style.clone();
        subheading_style.color = palette.white().into();
        subheading_style.bg_color = Color32::TRANSPARENT;
        subheading_style.shadow = None;

        let separator_style = SeparatorStyle {
            width: 2.0,
            color: palette.white().into(),
            ..Default::default()
        };
        knob_style.knob_color = palette.grey().into();
        knob_style.label_style.bg_color = Color32::TRANSPARENT;
        knob_style.label_style.color = palette.white().into();
        knob_style.label_style_hover.bg_color = Color32::TRANSPARENT;
        knob_style.label_style_hover.color = palette.color(0).into();
        knob_style.bg_color = palette.black().into();
        knob_style.indicator_fill_color = palette.grey().into();

        let mut knob_style_1 = knob_style.clone();
        knob_style_1.indicator_fill_color = palette.color_alt(0, 1).into();
        let mut knob_style_2 = knob_style.clone();
        knob_style_2.indicator_fill_color = palette.color(0).into();
        let mut knob_style_3 = knob_style.clone();
        knob_style_3.indicator_fill_color = palette.color(5).into();
        let mut knob_style_4 = knob_style.clone();
        knob_style_4.indicator_fill_color = palette.color(2).into();
        let mut knob_style_5 = knob_style.clone();
        knob_style_5.indicator_fill_color = palette.color(4).into();

        let mut slider_style = ParamSliderStyle {
            length: 70.0,
            ..Default::default()
        };
        /*
        slider_style.knob_aspect = 2.0/1.0;
        slider_style.knob_width = 20.0;
        slider_style.knob_round_ratio = 0.5;
        */
        slider_style.knob_color = knob_style.knob_color;
        slider_style.label_width = 70.0;
        slider_style.label_style.bg_color = Color32::TRANSPARENT;
        slider_style.label_style.color = knob_style.label_style.color;
        slider_style.label_style_hover.bg_color = knob_style.label_style_hover.bg_color;
        slider_style.label_style_hover.color = knob_style.label_style.color;
        slider_style.bg_color = palette.black().into();
        slider_style.fill_color = Color32::LIGHT_GREEN;

        let mut slider_style_1 = slider_style.clone();
        slider_style_1.fill_color = knob_style_1.indicator_fill_color;
        let mut slider_style_2 = slider_style.clone();
        slider_style_2.fill_color = knob_style_2.indicator_fill_color;
        let mut slider_style_3 = slider_style.clone();
        slider_style_3.fill_color = knob_style_3.indicator_fill_color;
        let mut slider_style_4 = slider_style.clone();
        slider_style_4.fill_color = knob_style_4.indicator_fill_color;
        #[allow(clippy::redundant_clone)]
        let mut slider_style_5 = slider_style.clone();
        slider_style_5.fill_color = knob_style_5.indicator_fill_color;

        let show_credits = Arc::new(AtomicBool::new(false));
        //let anim_id = Id::new("local_anim"); // Can use in each context once
        let credits_anim_id = Id::new("credits_anim");
        //let piano_anim_id = Id::new("piano_anim");

        #[cfg(feature = "profile")]
        let dsp_cost = self.dsp_cost.clone();

        let piano_indicator = self.piano_indicator.clone();

        create_egui_editor(
            self.params.editor_state.clone(),
            preset_manager,
            move |ctx, _| {
                // DPI:
                // egui_baseview needs to be updated to support DPI.
                // We can adjust egui's rendering, but not egui_baseview's resolution.
                //ctx.set_pixels_per_point(2.0);

                //nih_log!("Editor::Open()");
                Self::setup_egui_style(ctx);
            },
            move |ctx, setter, state| {
                #[cfg(feature = "stress_test")] // Add some randomizing functionality
                {
                    // Stress regular automation (not multiple at once!)
                    Preset::from_random_values(&params)
                        .filter_params_include(&["Mix Gain"])
                        .apply_with_automation(&params, setter);

                    // Stress applying a preset
                    Preset::from_random_values(&params).apply(&params, setter);
                }

                let preset_manager = state;

                #[cfg(feature = "profile")]
                let dsp_cost = dsp_cost.load(Ordering::Relaxed) as f32 * 0.00001; // Scale for 5 decimals of precision

                let show_credits_amount = ctx.animate_bool_with_time(
                    credits_anim_id,
                    show_credits.load(Ordering::Relaxed),
                    0.300,
                );

                if show_credits_amount > 0.0 {
                    Area::new("credits_area".into())
                        .fixed_pos(pos2(0.0, 0.0))
                        .order(Order::Foreground)
                        .show(ctx, |ui| {
                            Frame::new()
                                .outer_margin(Margin::same(0))
                                .inner_margin(Margin::same(10))
                                .fill(Color32::from_black_alpha(
                                    (200.0 * show_credits_amount) as u8,
                                ))
                                //.corner_radius(panel_rounding)
                                .show(ui, |ui| {
                                    let available_size =
                                        vec2(ui.available_width(), ui.available_height());
                                    //let animated_offset = vec2(0.0, available_size.y * (1.0-show_credits_amount));
                                    ui.painter().text(
                                        (available_size * 0.5).to_pos2(),
                                        Align2::CENTER_CENTER,
                                        format!(
                                            include_str!("CREDITS"),
                                            VERSION = env!("CARGO_PKG_VERSION")
                                        ),
                                        FontId::new(24.0, FontFamily::Name("Title".into())),
                                        Color32::from_white_alpha(
                                            (255.0 * show_credits_amount) as u8,
                                        ),
                                    );
                                    /*
                                    let galley = ui.painter().layout_no_wrap(
                                        include_str!("CREDITS").to_string(),
                                        FontId::new(20.0, FontFamily::Name("Title".into())),
                                        Color32::WHITE);
                                    let rect = Align2::CENTER_CENTER.anchor_rect(Rect::from_min_size((available_size * 0.5).to_pos2(), galley.size()));
                                    ui.painter().galley(rect.min, galley);
                                    */

                                    if ui
                                        .allocate_response(
                                            available_size,
                                            Sense::click(), // Sense{click:true, drag:false, focusable:false},
                                        )
                                        .clicked()
                                    {
                                        show_credits.store(false, Ordering::Relaxed);
                                    }
                                });
                        });
                }
                CentralPanel::default()
                    .frame(
                        Frame::new()
                            .outer_margin(Margin::same(0))
                            .inner_margin(Margin::same(10))
                            .fill(palette.white().brightness(0.1).into()),
                    )
                    .show(ctx, |ui| {
                        // if let Some(focus) = ui.memory(|m|m.focus()) {
                        //     nih_log!("Focus: {}", focus.short_debug_format());
                        // }

                        //ctx.set_debug_on_hover(false);

                        let header_height = 70.0;

                        ui.painter().rect_filled(
                            Rect::from_min_max(pos2(0.0, 0.0), pos2(415.0, header_height)),
                            CornerRadius::ZERO,
                            palette.white().brightness(0.05).to_color32(),
                        );

                        let mut active: Vec<u8> = Vec::with_capacity(MAX_VOICES);
                        for voice in piano_indicator.iter() {
                            let note = voice.load(Ordering::Relaxed);
                            if note > 0 {
                                active.push(note);
                            }
                        }
                        let piano_rect = Rect::from_min_size(
                            pos2(30.0, ui.available_height() - 60.0),
                            vec2(890.0, 60.0),
                        );
                        ui.painter().rect_filled(
                            piano_rect.expand(2.0),
                            CornerRadius::same(2),
                            palette.black().to_color32(),
                        );
                        ui.painter().midi_keyboard(
                            piano_rect,
                            MidiKeyboardStyle {
                                active_color: palette.color(5).to_color32(),
                                black_color: palette.black().to_color32(),
                                white_color: palette.white().to_color32(),
                            },
                            MidiNote::A0 as u8,
                            MidiNote::C8 as u8,
                            active.as_slice(),
                        );

                        ui.painter().parallelograms(
                            Rect::from_min_size(pos2(325.0, 0.0), vec2(220.0, header_height)),
                            1.0,
                            &[
                                //palette.color(0).shade(1).to_color32(),
                                palette.white().to_color32(),
                                palette.black().to_color32(),
                                palette.white().to_color32(),
                                palette.black().to_color32(),
                                palette.white().to_color32(),
                                //palette.color(0).shade(1).to_color32(),
                            ],
                        );
                        let hack_height = header_height * 0.63;
                        ui.painter().parallelograms(
                            Rect::from_min_size(
                                pos2(325.0, hack_height),
                                vec2(220.0 - hack_height, header_height - hack_height),
                            ),
                            1.0,
                            &[palette.white().to_color32()],
                        );

                        ui.set_width(ui.available_width());

                        let panel_rounding = CornerRadius::same(15);
                        //let panel_shadow = Shadow::small_light();
                        let panel_shadow = Shadow {
                            offset: [0, 0],
                            blur: 24,
                            spread: 0,
                            color: Color32::from_black_alpha(24),
                        };

                        ui.horizontal(|ui| {
                            // =========================================
                            // ============== MAIN PANEL ==============
                            // =========================================

                            Frame::new()
                                .outer_margin(Margin::same(0))
                                .inner_margin(Margin::symmetric(10, 10))
                                .fill(Color32::TRANSPARENT)
                                //.stroke(Stroke::new(1.0, palette.black()))
                                .show(ui, |ui| {
                                    ui.vertical(|ui| {
                                        // MAIN
                                        ui.set_width(75.0 * 8.0);
                                        ui.set_height(660.0);
                                        ui.add_space(header_height);
                                        /*
                                        ui.vertical_centered(|ui| {
                                            ui.add(
                                                Label::new("Kit").with_style(&heading_style),
                                            );
                                        });
                                        ui.add_space(5.0);
                                        */
                                        let center_offset_x = 45.0;
                                        let spacing_y = 15.0;
                                        ui.horizontal(|ui| {
                                            let justify_spacing_x = 20.0; //22.0;
                                            ui.spacing_mut().item_spacing =
                                                vec2(justify_spacing_x, spacing_y);

                                            ui.set_height(232.0);
                                            ui.vertical(|ui| {
                                                ui.horizontal(|ui| {
                                                    ui.add_space(center_offset_x * 1.25);
                                                    ui.add(
                                                        Label::new("Mix")
                                                            .with_style(&subheading_style),
                                                    );
                                                });
                                                ui.horizontal(|ui| {
                                                    ui.add_space(center_offset_x - 35.0);
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_float("Mix Gain"),
                                                            setter,
                                                        )
                                                        .with_label("Gain")
                                                        .with_style(&knob_style_5),
                                                    );
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_float("Mix Saturation"),
                                                            setter,
                                                        )
                                                        .with_label("Saturation")
                                                        .with_style(&knob_style_5),
                                                    );
                                                });
                                                ui.horizontal(|ui| {
                                                    ui.add_space(center_offset_x - 25.0);
                                                    ui.add(
                                                        ParamSlider::for_param(
                                                            params.param_float("Mix Pan"),
                                                            setter,
                                                        )
                                                        .with_label("Pan")
                                                        .with_style(&slider_style_5)
                                                        .with_indicator_from_center()
                                                        .with_orientation(
                                                            ParamSliderOrientation::Horizontal,
                                                        )
                                                        .with_length(140.0)
                                                        .with_label_width(70.0),
                                                    );
                                                });
                                            });
                                            ui.add(Separator::new().with_style(&separator_style));
                                            ui.vertical(|ui| {
                                                ui.horizontal(|ui| {
                                                    ui.add_space(center_offset_x);
                                                    ui.add(
                                                        Label::new("Strings")
                                                            .with_style(&subheading_style),
                                                    );
                                                });
                                                ui.horizontal(|ui| {
                                                    ui.add_space(center_offset_x);
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params
                                                                .param_float("Strings Brightness"),
                                                            setter,
                                                        )
                                                        .with_label("Brightness")
                                                        .with_style(&knob_style_3),
                                                    );
                                                });
                                                ui.horizontal(|ui| {
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_float("Strings In Tune"),
                                                            setter,
                                                        )
                                                        .with_label("In Tune")
                                                        .with_style(&knob_style_3),
                                                    );
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_float("Strings Detuned"),
                                                            setter,
                                                        )
                                                        .with_label("Detuned")
                                                        .with_style(&knob_style_3),
                                                    );
                                                });
                                            });

                                            ui.add(Separator::new().with_style(&separator_style));

                                            ui.vertical(|ui| {
                                                ui.horizontal(|ui| {
                                                    //ui.add_space(center_offset_x);
                                                    ui.add(
                                                        Label::new("Body")
                                                            .with_style(&subheading_style),
                                                    );
                                                });
                                                ui.horizontal(|ui| {
                                                    //ui.add_space(center_offset_x);
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_float("Body Size"),
                                                            setter,
                                                        )
                                                        .with_label("Size")
                                                        .with_style(&knob_style_4),
                                                    );
                                                });
                                                ui.horizontal(|ui| {
                                                    //ui.add_space(center_offset_x);
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_float("Body Damp"),
                                                            setter,
                                                        )
                                                        .with_label("Damp")
                                                        .with_style(&knob_style_4),
                                                    );
                                                });
                                            });
                                        });

                                        ui.set_width(910.0);

                                        ui.add_space(20.0);
                                        ui.vertical(|ui| {
                                            ui.set_width(525.0);
                                            ui.add(
                                                Separator::new()
                                                    .with_style(&separator_style)
                                                    .with_orientation(
                                                        SeparatorOrientation::Horizontal,
                                                    ),
                                            );
                                        });
                                        ui.add_space(20.0);

                                        ui.horizontal(|ui| {
                                            let justify_spacing_x = 20.0; //22.0;
                                            ui.spacing_mut().item_spacing =
                                                vec2(justify_spacing_x, spacing_y);

                                            ui.vertical(|ui| {
                                                ui.horizontal(|ui| {
                                                    ui.add_space(center_offset_x);
                                                    ui.add(
                                                        Label::new("Reverb")
                                                            .with_style(&subheading_style),
                                                    );
                                                });
                                                ui.horizontal(|ui| {
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_float("Reverb Amount"),
                                                            setter,
                                                        )
                                                        .with_label("Amount")
                                                        .with_style(&knob_style_4),
                                                    );
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_float("Reverb Size"),
                                                            setter,
                                                        )
                                                        .with_label("Size")
                                                        .with_style(&knob_style_4),
                                                    );
                                                });
                                                ui.horizontal(|ui| {
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_float("Reverb Damp"),
                                                            setter,
                                                        )
                                                        .with_label("Damp")
                                                        .with_style(&knob_style_4),
                                                    );
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_float("Reverb Highpass"),
                                                            setter,
                                                        )
                                                        .with_label("Highpass")
                                                        .with_style(&knob_style_4),
                                                    );
                                                });
                                            });

                                            ui.add(Separator::new().with_style(&separator_style));

                                            ui.vertical(|ui| {
                                                ui.horizontal(|ui| {
                                                    ui.add_space(center_offset_x * 2.0);
                                                    ui.add(
                                                        Label::new("Media")
                                                            .with_style(&subheading_style),
                                                    );
                                                });
                                                ui.horizontal(|ui| {
                                                    //ui.add_space(center_offset_x);
                                                    ui.add(
                                                        ParamSlider::for_param(
                                                            params.param_int("Media Noise Type"),
                                                            setter,
                                                        )
                                                        .with_label("Format")
                                                        .with_style(&slider_style_1)
                                                        .with_label_only_value()
                                                        .with_tick_count(2)
                                                        .without_fill(),
                                                    );
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params
                                                                .param_float("Media Noise Amount"),
                                                            setter,
                                                        )
                                                        .with_label("Noise")
                                                        .with_style(&knob_style_1),
                                                    );
                                                    ui.add(
                                                        ParamSlider::for_param(
                                                            params.param_float("Media Mono"),
                                                            setter,
                                                        )
                                                        .with_label("Mono")
                                                        .with_style(&slider_style_1),
                                                    );
                                                });
                                                ui.horizontal(|ui| {
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_float("Media Flutter"),
                                                            setter,
                                                        )
                                                        .with_label("Flutter")
                                                        .with_style(&knob_style_1),
                                                    );
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_float("Media Speed"),
                                                            setter,
                                                        )
                                                        .with_label("Speed")
                                                        .with_style(&knob_style_1),
                                                    );
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_float("Media Shape"),
                                                            setter,
                                                        )
                                                        .with_label("Shape")
                                                        .with_style(&knob_style_1),
                                                    );
                                                });
                                            });

                                            ui.add(Separator::new().with_style(&separator_style));

                                            ui.vertical(|ui| {
                                                ui.horizontal(|ui| {
                                                    ui.add_space(center_offset_x);
                                                    ui.add(
                                                        Label::new("Sampler")
                                                            .with_style(&subheading_style),
                                                    );
                                                });
                                                ui.horizontal(|ui| {
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params
                                                                .param_float("Sampler Samplerate"),
                                                            setter,
                                                        )
                                                        .with_label("Samplerate")
                                                        .with_style(&knob_style_2),
                                                    );
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_int("Sampler Bits"),
                                                            setter,
                                                        )
                                                        .with_label("Bits")
                                                        .with_style(&knob_style_2)
                                                        .with_indicator_style(
                                                            ParamKnobIndicatorStyle::Dots,
                                                        )
                                                        .with_indicator_count(9),
                                                    );
                                                });
                                                ui.horizontal(|ui| {
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_float("Sampler Highpass"),
                                                            setter,
                                                        )
                                                        .with_label("Low Cut")
                                                        .with_style(&knob_style_2),
                                                    );
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_float("Sampler Lowpass"),
                                                            setter,
                                                        )
                                                        .with_label("High Cut")
                                                        .with_style(&knob_style_2)
                                                        .with_indicator_from_end(),
                                                    );
                                                });
                                            });

                                            ui.add(Separator::new().with_style(&separator_style));

                                            ui.vertical(|ui| {
                                                ui.horizontal(|ui| {
                                                    ui.add_space(center_offset_x);
                                                    ui.add(
                                                        Label::new("MIDI")
                                                            .with_style(&subheading_style),
                                                    );
                                                });
                                                ui.horizontal(|ui| {
                                                    ui.add_space(center_offset_x);
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_float("MIDI Sensitivity"),
                                                            setter,
                                                        )
                                                        .with_label("Sensitivity")
                                                        .with_style(&knob_style_5),
                                                    );
                                                });
                                                ui.horizontal(|ui| {
                                                    ui.add(
                                                        ParamSlider::for_param(
                                                            params.param_int("MIDI Bend Range"),
                                                            setter,
                                                        )
                                                        .with_label("Bend Range")
                                                        .with_style(&slider_style_5),
                                                    );
                                                    ui.add(
                                                        ParamSlider::for_param(
                                                            params.param_bool("MIDI Sustain Lock"),
                                                            setter,
                                                        )
                                                        .with_label("Sustain Lock")
                                                        .with_style(&slider_style_5),
                                                    );
                                                });
                                            });
                                        });
                                    });
                                });

                            //ui.add(Separator::new().with_style(&separator_style));

                            // ============================================
                            // ============== GLOBALS PANELS ==============
                            // ============================================
                            ui.allocate_new_ui(
                                UiBuilder::new().max_rect(Rect::from_min_size(
                                    pos2(560.0, 10.0),
                                    vec2(380.0, 500.0),
                                )),
                                |ui| {
                                    Frame::new() // Global Panel Wrapper
                                        /*
                                        .outer_margin(Margin::same(0.0))
                                        .inner_margin(Margin::same(20.0))
                                        .corner_radius(panel_rounding)
                                        .fill(palette.white().into())
                                        .stroke(Stroke::new(2.0, palette.black()))
                                        */
                                        .show(ui, |ui| {
                                            let frame_outer_margin = Margin {
                                                top: 5,
                                                ..Default::default()
                                            };
                                            //let frame_inner_margin = Margin::same(10.0);
                                            let frame_inner_margin = Margin {
                                                left: 0,
                                                right: 0,
                                                top: 5,
                                                bottom: 15,
                                            };
                                            let subframe_inner_margin = Margin::same(10);

                                            //ui.set_max_width(75.0*5.0+30.0);
                                            ui.vertical_centered(|ui| {
                                                // ===========================================
                                                // ============== PRESETS PANEL ==============
                                                // ===========================================
                                                Frame::new()
                                                    .outer_margin(frame_outer_margin)
                                                    .inner_margin(frame_inner_margin)
                                                    //.fill(palette.color(6).shade(1).into())
                                                    .fill(palette.white().brightness(0.8).into())
                                                    .corner_radius(panel_rounding)
                                                    .shadow(panel_shadow)
                                                    .show(ui, |ui| {
                                                        ui.vertical_centered(|ui| {
                                                            ui.add(
                                                                Label::new("Presets")
                                                                    .with_style(&heading_style),
                                                            );
                                                        });

                                                        Frame::new()
                                                            .outer_margin(Margin::same(0))
                                                            .inner_margin(subframe_inner_margin)
                                                            //.fill(palette.color(6).shade(1).into())
                                                            .fill(
                                                                palette
                                                                    .white()
                                                                    .brightness(0.3)
                                                                    .into(),
                                                            )
                                                            .show(ui, |ui| {
                                                                // ============================================
                                                                // ============== PRESETS PANEL ==============
                                                                // ============================================
                                                                //let preset_manager: PresetManager = preset_manager.borrow();
                                                                let list_style = PresetListStyle {
                                                                    bg_color: None, //Some(palette.dark_layer(1).into()),
                                                                    color: palette.white().into(),
                                                                    icon_style: IconButtonStyle {
                                                                        color: palette
                                                                            .white()
                                                                            .into(),
                                                                        bg_color:
                                                                            Color32::TRANSPARENT,
                                                                        color_hover: palette
                                                                            .white()
                                                                            .into(),
                                                                        bg_color_hover: palette
                                                                            .black()
                                                                            .into(),
                                                                        ..Default::default()
                                                                    },
                                                                    label_style: LabelStyle {
                                                                        bg_color: palette
                                                                            .black()
                                                                            .alpha(0.35)
                                                                            .into(),
                                                                        bg_color_hover: Some(
                                                                            palette
                                                                                .black()
                                                                                .alpha(0.5)
                                                                                .into(),
                                                                        ),
                                                                        color: palette
                                                                            .white()
                                                                            .into(),
                                                                        color_hover: Some(
                                                                            palette.white().into(),
                                                                        ),
                                                                        ..Default::default()
                                                                    },
                                                                    highlight_color: palette
                                                                        .color(0)
                                                                        .into(),
                                                                    highlight_bg_color: palette
                                                                        .black()
                                                                        .alpha(0.9)
                                                                        .into(),
                                                                    search_bg_color: Some(
                                                                        palette
                                                                            .black()
                                                                            .alpha(0.5)
                                                                            .into(),
                                                                    ),
                                                                    search_color: Some(
                                                                        palette.white().into(),
                                                                    ),
                                                                    popup_offset_normalized: vec2(
                                                                        0.0, 1.04,
                                                                    ),
                                                                    ..Default::default()
                                                                };

                                                                ui.vertical_centered(|ui| {
                                                                    ui.set_width(
                                                                        ui.available_width(),
                                                                    );
                                                                    //ui.set_height(ui.available_height()-10.0);
                                                                    ui.set_height(260.0);
                                                                    PresetList::new()
                                                                        .with_style(&list_style)
                                                                        .show(
                                                                            ui,
                                                                            preset_manager,
                                                                            &params,
                                                                            setter,
                                                                        );
                                                                    /*
                                                                    ListView::new([].iter(), ())
                                                                        .title("Search".into())
                                                                        .hold_text("something".into())
                                                                        .striped()
                                                                        .show(ctx, ui);
                                                                    */
                                                                });
                                                            });
                                                    });
                                            });
                                        });
                                },
                            );
                        });

                        // Moved Logo down here to eliminate Area causing focus issues...
                        ui.allocate_new_ui(
                            UiBuilder::new()
                                .max_rect(Rect::from_min_size(pos2(0.0, 0.0), vec2(0.0, 0.0))),
                            |ui| {
                                let response = ui.allocate_rect(
                                    Rect::from_min_size(pos2(0.0, 0.0), vec2(190.0, 70.0)),
                                    Sense::CLICK,
                                );
                                if response.clicked() {
                                    show_credits.store(true, Ordering::Relaxed);
                                }
                                ui.painter().one_trick_logo(
                                    "KEYS",
                                    pos2(15.0, 35.0),
                                    52.0,
                                    palette.white().brightness(0.66).into(),
                                    if response.hovered() {
                                        palette.color(5).into()
                                    } else {
                                        palette.white().into()
                                    },
                                );
                            },
                        );
                    });
            },
        )
    }
}

impl ClapPlugin for OneTrickKeys {
    const CLAP_ID: &'static str = "com.punklabs.onetrick.keys";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A Physically Modeled Piano Synth");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::Instrument, //Plugin Category
        ClapFeature::Stereo,
    ]; // Audio Capabilities
       /*
       const CLAP_POLY_MODULATION_CONFIG: Option<PolyModulationConfig> = Some(PolyModulationConfig {
           // If the plugin's voice capacity changes at runtime (for instance, when switching to a
           // monophonic mode), then the plugin should inform the host in the `initialize()` function
           // as well as in the `process()` function if it changes at runtime using
           // `context.set_next_voice_capacity()`
           max_voice_capacity: MAX_VOICES as u32,
           // This enables voice stacking in Bitwig.
           supports_overlapping_voices: true,
       });
       */
}

impl Vst3Plugin for OneTrickKeys {
    const VST3_CLASS_ID: [u8; 16] = *b"OneTrick-KEYS   ";
    //const VST3_CATEGORIES: &'static str = "Instrument|Synth";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Instrument, Vst3SubCategory::Synth];
}

nih_export_clap!(OneTrickKeys);
nih_export_vst3!(OneTrickKeys);
