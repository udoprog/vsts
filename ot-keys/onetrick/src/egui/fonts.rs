/*
   ┏━━━┓╱╱╱╱╱┏┓╱╱┏┓╱╱╱╱╱┏┓╱╱
   ┃┏━┓┃╱╱╱╱╱┃┃╱╱┃┃╱╱╱╱╱┃┃╱╱╱
   ┃┗━┛┣┓┏┳━━┫┃┏┓┃┃╱╱┏━━┫┗━┳━━┓
   ┃┏━━┫┃┃┃┏┓┃┗┛┃┃┃╱┏┫┏┓┃┏┓┃━━┫
   ┃┃╱╱┃┗┛┃┃┃┃┏┓┃┃┗━┛┃┏┓┃┗┛┣━━┃
   ┗┛╱╱┗━━┻┛┗┻┛┗┛┗━━━┻┛┗┻━━┻━━┛
    ━━━━━━━━━━━━━━━━━━━━━━━━━━

    Copyright (c) 2023 Punk Labs LLC

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

use std::sync::Arc;

use nih_plug_egui::egui::{
    FontData,
    FontDefinitions,
    FontFamily,
    FontTweak,
    //FontId,
};

/// Helpers to add common fonts used by OneTrick plugins
pub struct BasicFonts {}

impl BasicFonts {
    /// Adds common fonts used by OneTrick plugins to a FontDefinition
    /// Currently, this is Teko, and Rajdhani
    pub fn add_fonts(fonts: &mut FontDefinitions) {
        // Families:
        fonts
            .families
            .insert(FontFamily::Name("Label".into()), vec![]);
        fonts
            .families
            .insert(FontFamily::Name("Title".into()), vec![]);

            /* FontTweak {
            scale: f32,
            y_offset_factor: f32,
            y_offset: f32,
            baseline_offset_factor: f32,
            } */
        // Label Font: Teko
        
        let font_data = FontData::from_static(include_bytes!("../assets/fonts/Teko/Teko-Regular.ttf")).tweak(
                FontTweak {
                    scale: 1.00,
                    y_offset_factor: 0.08,
                    ..Default::default()
                }
            );
        
        fonts.font_data.insert(
            "Teko".to_owned(),
            Arc::new(font_data),
        );

        fonts
            .families
            .entry(FontFamily::Name("Label".into()))
            .or_default()
            .insert(0, "Teko".to_owned());

        let font_data = Arc::new(FontData::from_static(include_bytes!("../assets/fonts/Rajdhani/Rajdhani-Bold.ttf"))
                .tweak(
                    FontTweak {
                        scale: 1.00,
                        ..Default::default()
                    }
                ));

        // Title Font: Rajdhani
        fonts.font_data.insert(
            "Rajdhani".to_owned(),
            font_data
        );

        fonts
            .families
            .entry(FontFamily::Name("Title".into()))
            .or_default()
            .insert(0, "Rajdhani".to_owned()); // Highest Priority

        // Default (Proportional) Font: Teko
        fonts
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .insert(0, "Teko".to_owned()); // Highest Priority
    }
}
