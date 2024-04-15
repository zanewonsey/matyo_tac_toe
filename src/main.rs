use eframe::{egui::CentralPanel, App, HardwareAcceleration, Theme};
use egui::Button;
use tic_tac_toe::{CellType, TicTacToe};

mod tic_tac_toe;

struct Application<'a> {
    empty_image: egui::Image<'a>,
    circle_image: egui::Image<'a>,
    cross_image: egui::Image<'a>,
    game: TicTacToe
}

impl Application<'_> {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        println!("doesthis run");
        Self {
            empty_image:  egui::Image::new(egui::include_image!("../Empty.png")),
            circle_image: egui::Image::new(egui::include_image!("../Circle.png")),
            cross_image:  egui::Image::new(egui::include_image!("../CrossX.png")),
            game: TicTacToe::new(),
        }
    }

    fn title_screen(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {

            ui.style_mut().text_styles.insert(
                egui::TextStyle::Button,
                egui::FontId::new(24.0, eframe::epaint::FontFamily::Proportional),
            );

            if ui.add_sized([200.0, 50.0], egui::Button::new("Play match")).clicked() {

            }

            if ui.add_sized([200.0, 50.0], egui::Button::new("Exit Application")).clicked() {

            }
        });
    }

    fn game_screen() {

    }

    fn win_screen() {
        
    }

}

impl App for Application<'_> {

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        egui_extras::install_image_loaders(ctx);

        self.title_screen(ctx);

        /*CentralPanel::default().show(ctx, |ui|{
            if self.game.in_progress() {
                let win_state = self.game.check_for_win();
                if win_state.0 {
                    match win_state.1 {
                        CellType::Empty => todo!(),
                        CellType::Circle => {
                            println!("Circle wins");
                            ui.add_sized([500.0, 500.0],egui::Image::clone(&self.circle_image));
                        },
                        CellType::Cross => {
                            println!("Cross wins");
                            ui.add_sized([500.0, 500.0],egui::Image::clone(&self.cross_image));
                        },
                    }
                    ui.add_sized([200.0, 500.0], egui::Label::new("Wins!"));
                } else {
                    egui::Grid::new("GameTable").show(ui, |ui| {
                
                        // This probably doesn't need to be a Vec. The idea wasto be able to process more than one event at once.
                        // It shouldn't be possible to play twice in one frame.
                        let mut cells_to_update: Vec<(usize, usize)> = Vec::new();
        
                        for row in 0..self.game.board_size {
        
                            for column in 0..self.game.board_size {
                                let image_of_celltype = match self.game.get_cell_at(column, row) {
                                    CellType::Empty  => egui::Image::clone(&self.empty_image),
                                    CellType::Circle => egui::Image::clone(&self.circle_image),
                                    CellType::Cross  => egui::Image::clone(&self.cross_image)
                                };
                                
                                if self.game.get_cell_at(column, row) != CellType::Empty {
                                    ui.add_sized([180.0, 180.0],image_of_celltype);
                                } else {
                                    if ui.add_sized([180.0, 180.0],egui::widgets::ImageButton::new(image_of_celltype)).clicked() {
                                        cells_to_update.push((row, column));
                                        println!("Cell row {} column {}", row, column)
                                    }
                                }
                                
                            }
                            ui.end_row();
                        }
        
                        for cell in cells_to_update {
                            self.game.play_cell(cell.0, cell.1);
                            self.game.turn = !self.game.turn;
                        }
                    }); // </Grid>
                }

            }
        }); // </CentralPanel>*/
    }
}

fn main() {
    let vp = egui::ViewportBuilder::default()
        .with_resizable(false);

    let options = eframe::NativeOptions {
        viewport: vp,
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        hardware_acceleration: HardwareAcceleration::Preferred,
        renderer: eframe::Renderer::Glow,
        follow_system_theme: cfg!(target_os = "macos") || cfg!(target_os = "windows"),
        default_theme: Theme::Dark,
        run_and_return: true,
        event_loop_builder: None,
        window_builder: None,
        shader_version: None,
        centered: false,
        persist_window: true,
    };

    let _ = eframe::run_native("matyo tac toe",
    options,
        Box::new(
            |app|
            Box::new(Application::new(app)))
    );
}
