use std::ops::Index;
use eframe::{egui::CentralPanel, App, NativeOptions};

enum CellType {
    Empty,
    Circle,
    Cross
}

struct GameCell {
    pub celltype: CellType
}

impl GameCell {
    fn new() -> Self{
        Self {
            celltype: CellType::Empty
        }
    }
}

struct Application<'a> {
    empty_image: egui::Image<'a>,
    circle_image: egui::Image<'a>,
    cross_image: egui::Image<'a>,
    pub game_board: [[GameCell; 3]; 3]
}

impl Application<'_> {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        println!("doesthis run");
        Self {
            empty_image:  egui::Image::new(egui::include_image!("../Empty.png")),
            circle_image: egui::Image::new(egui::include_image!("../Circle.png")),
            cross_image:  egui::Image::new(egui::include_image!("../CrossX.png")),
            game_board: [[GameCell::new(), GameCell::new(), GameCell::new()],
                         [GameCell::new(), GameCell::new(), GameCell::new()],
                         [GameCell::new(), GameCell::new(), GameCell::new()]]
        }
    }

    fn get_cell_at(&self, column: usize, row: usize) -> CellType {
        match self.game_board.index(row).index(column).celltype {
            CellType::Empty => CellType::Empty,
            CellType::Circle => CellType::Circle,
            CellType::Cross => CellType::Cross,
        }
    }

    fn update_cell_at(&mut self, column: usize, row: usize, cell_type: CellType) {
        self.game_board[column][row].celltype = cell_type;
    }
}

impl App for Application<'_> {

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        egui_extras::install_image_loaders(ctx);

        CentralPanel::default().show(ctx, |ui|{
            egui::Grid::new("GameTable").show(ui, |ui| {
                
                // This probably doesn't need to be a Vec
                let mut cells_to_update: Vec<(usize, usize)> = Vec::new();

                for (row, cells_in_row) in self.game_board.iter().enumerate() {

                    for (column, _cell) in cells_in_row.iter().enumerate() {
                        let image_of_CellType = match self.get_cell_at(column, row) {
                            CellType::Empty  => egui::Image::clone(&self.empty_image),
                            CellType::Circle => egui::Image::clone(&self.circle_image),
                            CellType::Cross  => egui::Image::clone(&self.cross_image)
                        };

                        if ui.add_sized([180.0, 180.0],egui::widgets::ImageButton::new(image_of_CellType)).clicked() {
                            cells_to_update.push((row, column));
                            println!("Cell row {} column {}", row, column)
                        }
                    }
                    ui.end_row();
                }

                for cell in cells_to_update {
                    self.update_cell_at(cell.0, cell.1, CellType::Circle);
                }

            }); // </Grid>
        }); // </CentralPanel>
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
