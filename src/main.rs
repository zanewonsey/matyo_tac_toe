use std::ops::Index;

use eframe::{egui::CentralPanel, App, NativeOptions};

enum Cell_Type {
    Empty,
    Circle,
    Cross
}
struct Game_Cell {
    celltype: Cell_Type
}

impl Game_Cell {
    fn new() -> Self{
        Self {
            celltype: Cell_Type::Empty
        }
    }
}

struct Application<'a> {
    empty_image: egui::Image<'a>,
    circle_image: egui::Image<'a>,
    cross_image: egui::Image<'a>,
    game_board: [[Game_Cell; 3]; 3]
}

impl Application<'_> {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        println!("doesthis run");
        Self {
            empty_image:  egui::Image::new(egui::include_image!("../Empty.png")),
            circle_image: egui::Image::new(egui::include_image!("../Circle.png")),
            cross_image:  egui::Image::new(egui::include_image!("../CrossX.png")),
            game_board: [[Game_Cell::new(), Game_Cell::new(), Game_Cell::new()],
                         [Game_Cell::new(), Game_Cell::new(), Game_Cell::new()],
                         [Game_Cell::new(), Game_Cell::new(), Game_Cell::new()]]
        }
    }

    fn get_cell_at(self, column: usize, row: usize) -> Cell_Type{
        match self.game_board.index(column).index(row).celltype {
            Cell_Type::Empty => Cell_Type::Empty,
            Cell_Type::Circle => Cell_Type::Circle,
            Cell_Type::Cross => Cell_Type::Cross,
        }
    }
}

impl App for Application<'_> {

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        egui_extras::install_image_loaders(ctx);

        CentralPanel::default().show(ctx, |ui|{
            egui::Grid::new("GameTable").show(ui, |ui| {
                
                if ui.add_sized([180.0, 180.0],egui::Image::clone(&self.empty_image)).clicked() {
                    println!("clicked1");
                };
                if ui.add_sized([180.0, 180.0],egui::Image::clone(&self.circle_image)).clicked() {
                    println!("clicked2");
                };
                if ui.add_sized([180.0, 180.0],egui::Image::clone(&self.cross_image)).clicked() {
                    println!("clicked3");
                };
                ui.end_row();
            
                if ui.add_sized([180.0, 180.0],egui::Image::clone(&self.empty_image)).clicked() {
                    println!("clicked4");
                };
                if ui.add_sized([180.0, 180.0],egui::Image::clone(&self.empty_image)).clicked() {
                    println!("clicked5");
                };
                if ui.add_sized([180.0, 180.0],egui::Image::clone(&self.empty_image)).clicked() {
                    println!("clicked6");
                };
                ui.end_row();

                if ui.add_sized([180.0, 180.0],egui::widgets::ImageButton::new(egui::Image::clone(&self.cross_image))).clicked() {
                    println!("696969")
                }


                let response = ui.allocate_response(egui::vec2(100.0, 200.0), egui::Sense::click());
                if response.clicked() { println!("clicked69") }
                ui.painter().rect_stroke(response.rect, 0.0, (1.0, egui::Color32::WHITE));
            
                if ui.add_sized([180.0, 180.0],egui::Image::clone(&self.empty_image)).clicked() {
                    println!("clicked7");
                };
                if ui.add_sized([180.0, 180.0],egui::Image::clone(&self.empty_image)).clicked() {
                    println!("clicked8");
                };
                if ui.add_sized([180.0, 180.0],egui::Image::clone(&self.empty_image)).clicked() {
                    println!("clicked9");
                };
                ui.end_row();
            });
        });
    }
}

fn main() {
    
    //let app = Application{
    //    counter: 0
    //};
    let mut win_option = NativeOptions::default();
    win_option.centered = true;
    //win_option.window_builder = WindowBuilderHook::new();
    let _ = eframe::run_native("zaneguitest",
        win_option,
        Box::new(
            |app|
            Box::new(Application::new(app)))
    );
}
