
use egui::{Ui, Style, Frame, Color32, Stroke};
use rand::prelude::*;

use crate::wallpaper::Wallpaper;

pub fn color32_from_u8(bytes:&[u8; 4]) -> Color32{
    Color32::from_rgba_premultiplied(bytes[0], bytes[1], bytes[2], bytes[3])
}

#[derive(Clone)]
pub struct ColorPalette{
    pub border:[u8; 4],
    pub background:[u8; 4],
    pub text:[u8; 4],
    pub indicator:[u8; 4],
    pub child_border:[u8; 4],
}

impl Default for ColorPalette{
    fn default() -> Self{
        Self{
            border: [0xff, 0xff, 0xff, 0xff],
            background: [0x00, 0x00, 0x00, 0xff],
            text: [0xff, 0xff, 0xff, 0xff],
            indicator: [0x00, 0x00, 0x00, 0xff],
            child_border: [0x00, 0x00, 0x00, 0xff],

        }
    }
}
impl ColorPalette{
    fn ui(&mut self, ui:&mut Ui){
        let id:u64 = rand::random();
        ui.horizontal( |ui|{


            ui.color_edit_button_srgba_unmultiplied (&mut self.border)
            .on_hover_text_at_pointer("Border Color");
            ui.color_edit_button_srgba_unmultiplied(&mut self.background)
            .on_hover_text_at_pointer("Background Color");
            ui.color_edit_button_srgba_unmultiplied(&mut self.text)
            .on_hover_text_at_pointer("Text Color");
            ui.color_edit_button_srgba_unmultiplied(&mut self.indicator)
            .on_hover_text_at_pointer("Indicator Color");
            ui.color_edit_button_srgba_unmultiplied(&mut self.child_border)
            .on_hover_text_at_pointer("Child Border Color");

        });
    }
}

#[derive(Clone)]
pub struct Theme{
    pub wallpaper:Wallpaper,

    pub border:f32,
    pub gaps:f32,

    pub focused:ColorPalette,
    pub unfocused:ColorPalette,
    pub focused_inactive:ColorPalette,
    pub placeholder:ColorPalette,
    pub urgent:ColorPalette,
    pub background:Color32,
}


impl Default for Theme{
    fn default() -> Self{
        Self{
            wallpaper:Wallpaper::new("".to_string()),

            border:4.0,
            gaps:0.0,

            focused:ColorPalette::default(),
            unfocused:ColorPalette::default(),
            focused_inactive:ColorPalette::default(),
            placeholder:ColorPalette::default(),
            urgent:ColorPalette::default(),
            background:Color32::BLACK,
        }
    }
}

impl Theme{
    pub fn ui(&mut self, ui:&mut Ui){
        egui::Grid::new("theme_grid")
        .show(ui, |ui|{
            ui.label("Background");
            ui.color_edit_button_srgba(&mut self.background); 
            ui.end_row();

            ui.label("Focused");
            self.focused.ui(ui);
            ui.end_row();
            
            ui.label("Unfocused");
            self.unfocused.ui(ui);
            ui.end_row();
            
            ui.label("Focused Inactive");
            self.focused_inactive.ui(ui);
            ui.end_row();
            
            ui.label("Placeholder");
            self.placeholder.ui(ui);
            ui.end_row();
            
            ui.label("Urgent");
            self.urgent.ui(ui);
            ui.end_row();
            

        });
    }

    pub fn frame(&mut self) -> Frame{
        let mut frame = Frame::default();
        frame = frame.fill(Color32::from_rgba_unmultiplied(
            self.focused.background[0],
            self.focused.background[1],
            self.focused.background[2],
            self.focused.background[3],
        ));
        frame = frame.stroke(Stroke::new(self.border, 
            Color32::from_rgba_unmultiplied(
                self.focused.border[0],
                self.focused.border[1],
                self.focused.border[2],
                self.focused.border[3],
            ))
        );
        frame = frame.inner_margin(8.0+self.border/2.0);


        frame
    }
}