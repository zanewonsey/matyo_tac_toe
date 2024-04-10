use std::ops::Index;
use eframe::{egui::CentralPanel, App, NativeOptions};

#[derive(PartialEq)]
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
    /// Stores the state of the tic-tac-toe board
    pub game_board: [[GameCell; 3]; 3],
    /// Represents whos turn it is. True if player one and False if player two.
    turn: bool,
    /// Is the game being played
    game_in_progress: bool
}

impl Application<'_> {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        println!("doesthis run");
        Self {
            empty_image:  egui::Image::new(egui::include_image!("../Empty.png")),
            circle_image: egui::Image::new(egui::include_image!("../Circle.png")),
            cross_image:  egui::Image::new(egui::include_image!("../CrossX.png")),
            game_board: [[GameCell::new(), GameCell::new(), GameCell::new()],
                         [GameCell::new(), GameCell::new(), GameCell::new()],
                         [GameCell::new(), GameCell::new(), GameCell::new()]],
            turn: true,
            game_in_progress: true // TODO change this to false when menu added
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

    fn check_board_for_win(&self) -> (bool, CellType) {
        let mut win_state = (false, CellType::Empty);

        // check columns
        for column in 0..=2 {
            if (self.game_board.index(0).index(column).celltype != CellType::Empty) &
               (self.game_board.index(0).index(column).celltype  ==
                self.game_board.index(1).index(column).celltype) &
               (self.game_board.index(1).index(column).celltype  ==
                self.game_board.index(2).index(column).celltype)
            {
                win_state = (true, match self.game_board.index(0).index(column).celltype
                {
                    CellType::Empty => CellType::Empty,
                    CellType::Circle => CellType::Circle,
                    CellType::Cross => CellType::Cross,
                });
            }
        }
        

        // check rows
        for row in 0..=2 {
            if (self.game_board.index(row).index(0).celltype != CellType::Empty) &
               (self.game_board.index(row).index(0).celltype  ==
                self.game_board.index(row).index(1).celltype) &
               (self.game_board.index(row).index(1).celltype  ==
                self.game_board.index(row).index(2).celltype)
            {
                win_state = (true, match self.game_board.index(row).index(0).celltype
                {
                    CellType::Empty => CellType::Empty,
                    CellType::Circle => CellType::Circle,
                    CellType::Cross => CellType::Cross,
                });
            }
        }

        // check diagonals
        if (self.game_board.index(0).index(0).celltype != CellType::Empty) &
           (self.game_board.index(0).index(0).celltype  ==
            self.game_board.index(1).index(1).celltype) &
           (self.game_board.index(1).index(1).celltype  ==
            self.game_board.index(2).index(2).celltype)
        {
            win_state = (true, match self.game_board.index(0).index(0).celltype
            {
                CellType::Empty => CellType::Empty,
                CellType::Circle => CellType::Circle,
                CellType::Cross => CellType::Cross,
            });
        }

        if (self.game_board.index(0).index(2).celltype != CellType::Empty) &
           (self.game_board.index(0).index(2).celltype  ==
            self.game_board.index(1).index(1).celltype) &
           (self.game_board.index(1).index(1).celltype  ==
            self.game_board.index(2).index(0).celltype)
        {
            win_state = (true, match self.game_board.index(0).index(2).celltype
            {
                CellType::Empty => CellType::Empty,
                CellType::Circle => CellType::Circle,
                CellType::Cross => CellType::Cross,
            });
        }

        // return
        win_state
    }
}

impl App for Application<'_> {

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        egui_extras::install_image_loaders(ctx);

        CentralPanel::default().show(ctx, |ui|{
            egui::Grid::new("GameTable").show(ui, |ui| {
                
                // This probably doesn't need to be a Vec. The idea wasto be able to process more than one event at once.
                // It shouldn't be possible to play twice in one frame.
                let mut cells_to_update: Vec<(usize, usize)> = Vec::new();

                for (row, cells_in_row) in self.game_board.iter().enumerate() {

                    for (column, _cell) in cells_in_row.iter().enumerate() {
                        let image_of_celltype = match self.get_cell_at(column, row) {
                            CellType::Empty  => egui::Image::clone(&self.empty_image),
                            CellType::Circle => egui::Image::clone(&self.circle_image),
                            CellType::Cross  => egui::Image::clone(&self.cross_image)
                        };

                        if ui.add_sized([180.0, 180.0],egui::widgets::ImageButton::new(image_of_celltype)).clicked() {
                            cells_to_update.push((row, column));
                            println!("Cell row {} column {}", row, column)
                        }
                    }
                    ui.end_row();
                }

                for cell in cells_to_update {
                    let cell_to_use = if self.turn {CellType::Cross} else {CellType::Circle};
                    self.turn = !self.turn;
                    self.update_cell_at(cell.0, cell.1, cell_to_use);
                }

                let win_state = self.check_board_for_win();
                if win_state.0 {
                    match win_state.1 {
                        CellType::Empty => todo!(),
                        CellType::Circle => println!("Circle wins"),
                        CellType::Cross => println!("Cross wins"),
                    }
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
