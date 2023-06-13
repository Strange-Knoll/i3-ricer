#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, CreationContext};
use egui::{Ui, Color32, Frame, epaint::Shadow, Stroke, Image, Align2, TextureHandle, Rect, Vec2, Pos2, ColorImage, RichText};
use image::DynamicImage;
use std::{ num::ParseIntError, fs, io::Write, path::Path, ops::RangeInclusive};

mod wallpaper;
use wallpaper::Wallpaper;
mod theme;
use theme::{Theme, ColorPalette, color32_from_u8};
mod compiler;
use compiler::{compile, save_config};

fn main() -> Result<(), eframe::Error>{
    println!("Hello, world!");
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc: &CreationContext| Box::<Rice>::default()),
    )
}


struct Rice{
    save_path:String,
    theme:Theme,
}

impl Default for Rice {
    fn default() -> Self {
        Self{
            save_path:"ABSOLUTE PATH".to_owned(),
            theme:Theme::default(),
        }
    }
}

impl eframe::App for Rice{

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) { 

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui|{
            ui.horizontal(|ui|{
                ui.label("Ricer");
                if ui.button("Quit").clicked(){
                    std::process::exit(0);
                }
            });
        });

        egui::Window::new("I3 Config")
        .scroll2([true, true])
        .resizable(true)
        .collapsible(true)
        .frame(self.theme.frame())
        .show(ctx, |ui| {
            ui.horizontal(|ui|{
                ui.label("Save");
                
                ui.text_edit_singleline(&mut self.save_path)
                    .on_hover_text("Copy this into your i3 config file");
                
                if ui.button("Save").clicked(){
                    compiler::save_config(self.theme.clone(), &self.save_path)
                }
            });
            
            ui.horizontal(|ui|{
                ui.label("Wallpaper");
                ui.text_edit_singleline(&mut self.theme.wallpaper.path)
                    .on_hover_text("This path must be absolute !");
                if ui.button("Apply").clicked(){
                    self.theme.wallpaper = Wallpaper::new(self.theme.wallpaper.path.clone());
                }

            });

            ui.spacing();

            ui.horizontal(|ui|{
                ui.label("Border ");
                ui.add(egui::DragValue::new(&mut self.theme.border)
                    .clamp_range(RangeInclusive::new(0.0, 10.0)));
            });
            
            ui.horizontal(|ui|{
                ui.label("Gaps");
                ui.add(egui::DragValue::new(&mut self.theme.gaps)
                    .clamp_range(RangeInclusive::new(0.0, 10.0)));

            });
            

            ui.spacing();
            ui.vertical(|ui|{
                ui.label("Color Palette");
                self.theme.ui(ui);
            })
        });

        egui::Window::new("Compiled Config")
        .scroll2([true, true])
        .resizable(true)
        .frame(self.theme.frame())
        .show(ctx, |ui|{
            ui.monospace(
                RichText::new(
                compile(self.theme.clone()))
                .color(theme::color32_from_u8(&self.theme.focused.text))
            );
        });

        egui::CentralPanel::default()
        .frame(Frame::default()
            .fill(Color32::BLACK)
        )
        .show(ctx, |ui|{
            //show bg image
            self.theme.wallpaper.ui(ui);
        });
    }
}