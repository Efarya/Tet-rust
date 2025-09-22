use std::io::{stdout, Write};
use std::thread;
use std::time::{Duration, Instant};
use crossterm::cursor::MoveTo;
use crate::board;
use crate::board::{Board, HEIGHT, SHAPE, WIDTH};
use crate::piece::{check_move_down_allowed, check_move_lat_allowed, check_rotate_allowed, generate_bag, move_down, move_lat, new_piece, rotate, Piece, PieceType};
use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::{cursor, execute, queue, style::{self, Stylize}};
use crossterm::style::{Color, ResetColor, SetForegroundColor};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, SetSize};

struct Game {
    board: Board,
    current_piece: Piece,
    bag: Vec<PieceType>,
    is_over: bool,
    level: usize,
    score: usize,
    completed_lines: usize,
}

impl Game {
    fn new() -> Self {
        let mut bag = generate_bag();
        Game {
            board: Board::new(),
            current_piece: new_piece(bag.pop().unwrap()),
            bag,
            is_over: false,
            level: 0,
            score: 0,
            completed_lines: 0,
        }
    }
}

pub fn start_game() {
    enable_raw_mode().unwrap();

    // initiate game
    let mut game = Game::new();
    let mut tick_down = Instant::now();

    // loop
    loop {
        // read inputs
        let mut need_redraw = handle_keyboard(&mut game);

        // make current_piece falls down
        if tick_down.elapsed() >= fall_delay(game.level) {
            if check_move_down_allowed(&game.current_piece, &game.board) {
                move_down(&mut game.current_piece);
            } else {
                lock_piece(&mut game);
                clear_lines(&mut game);
                allocate_new_piece(&mut game);
            }

            tick_down = Instant::now();
            need_redraw = true;
        }

        // render board and current piece
        if need_redraw {
            render(&game);
        }

        if game.is_over {
            break;
        }

        // 16ms would be around 60fps, provokes flickering in embedded CLI
        // disabling fps limit is probably using cpu A LOT
        thread::sleep(Duration::from_millis(16));
    }

    // TODO register new score here, game ended
    // maybe return the score and register that in the menu, not sure
    // better let the menu be stoupid

    disable_raw_mode().unwrap();
    execute!(stdout(), cursor::Show).unwrap();
}

// TODO improvement, move the borders outside of the render, only render board, would increase perf
fn render(game: &Game) {
    let board = &game.board;
    let current_piece = &game.current_piece;
    let mut stdout = stdout();
    queue!(
        stdout,
        SetSize((WIDTH + 20) as u16, (HEIGHT + 5) as u16),
        MoveTo(0, 0),
        Clear(ClearType::All),
        cursor::Hide,
        style::PrintStyledContent( "+----------+".green()),
    ).unwrap();

    for y in 0..HEIGHT {
        queue!(
            stdout,
            MoveTo(0, (y + 1) as u16),
            style::PrintStyledContent( "|".grey()) // line border (left)
        ).unwrap();
        for x in 0..WIDTH {
            // get default fixed board cell
            let mut cell = board.cells[y][x];

            // if cell is already taken, no need to process the rest
            if let Some(ch) = cell {
                queue!(stdout, style::Print(ch)).unwrap();
            } else {
                // for each piece shape's vector, check if it's on the current cell
                for (dx, dy) in &current_piece.shape {
                    let px = current_piece.x + dx;
                    let py = current_piece.y + dy;

                    if px == x as i32 && py == y as i32 {
                        cell = Some(SHAPE);
                    }
                }

                if let Some(ch) = cell {
                    queue!(stdout, SetForegroundColor(Color::DarkRed), style::Print(ch), ResetColor).unwrap();
                } else {
                    queue!(stdout, style::Print(' ')).unwrap();
                }
            }
        }
        queue!(stdout, style::PrintStyledContent( "|".grey())).unwrap(); // line border (right)
    }
    queue!(
        stdout,
        MoveTo(0, (HEIGHT + 1) as u16),
        style::PrintStyledContent( "+----------+".green()),
    ).unwrap();

    // score and level
    queue!(stdout, MoveTo(12, 1), style::Print(format!("level: {}", game.level))).unwrap();
    queue!(stdout, MoveTo(12, 2), style::Print(format!("score: {}", game.score))).unwrap();
    queue!(stdout, MoveTo(12, 3), style::Print(format!("lines: {}", game.completed_lines))).unwrap();

    stdout.flush().unwrap();
}

fn handle_keyboard(game: &mut Game) -> bool {
    // read every 10ms from input
    if poll(Duration::from_millis(5)).unwrap() {
        if let Event::Key(event) = read().unwrap() {
            match event.code {
                KeyCode::Left => {
                    if check_move_lat_allowed(&game.current_piece, &game.board, -1) {
                        move_lat(&mut game.current_piece, -1);
                        return true
                    }
                }
                KeyCode::Right => {
                    if check_move_lat_allowed(&game.current_piece, &game.board, 1) {
                        move_lat(&mut game.current_piece, 1);
                        return true
                    }
                }
                KeyCode::Up => {
                    if check_rotate_allowed(&game.current_piece, &game.board) {
                        rotate(&mut game.current_piece);
                        return true
                    }
                }
                KeyCode::Down => {
                    if check_move_down_allowed(&game.current_piece, &game.board) {
                        move_down(&mut game.current_piece)
                    } else {
                        lock_piece(game);
                        clear_lines(game);
                        allocate_new_piece(game);
                    }
                    return true
                }
                KeyCode::Esc => { game.is_over = true; }
                _ => {}
            }
        }
    }

    false
}

fn lock_piece(game: &mut Game) {
    // copy current piece shape into board
    for (dx, dy) in &game.current_piece.shape {
        let v_dx = dx + game.current_piece.x;
        let v_dy = dy + game.current_piece.y;

        // make sure we have positive shape index
        if v_dy >= 0 {
            game.board.cells[v_dy as usize][v_dx as usize] = Some(SHAPE);
        }

        // if one is outside the board, game is ending
        if v_dy <= 0 {
            game.is_over = true;
        }
    }
}

fn allocate_new_piece(game: &mut Game) {
    game.current_piece = new_piece(game.bag.pop().unwrap());

    // check if we have pieces left in the bag
    // we always assure we have some, so we can display the next one
    if game.bag.is_empty() {
        game.bag = generate_bag();
    }
}

fn clear_lines(game: &mut Game) {
    // keep only row with at least one cell empty
    game.board.cells.retain(|row| row.iter().any(|cell| cell.is_none()));

    // compute how many rows are missing
    let completed_lines = board::HEIGHT - game.board.cells.len();

    // early return, if there is no completed lines, we don't need to clear and compute score
    if completed_lines == 0 {
        return;
    }

    for _ in 0..completed_lines {
        game.board.cells.insert(0, vec![None; WIDTH]);
    }

    game.score += compute_score(completed_lines, game.level);
    game.completed_lines += completed_lines;
    game.level = game.completed_lines / 10;
}

fn compute_score(lines: usize, level: usize) -> usize {
    match lines {
        1 => 100 * level,
        2 => 300 * level,
        3 => 500 * level,
        4 => 800 * level,
        _ => 0, // you can only complete 1 to 4 lines
    }
}

fn fall_delay(level: usize) -> Duration {
    let ms = match level {
        0 => 800,
        1 => 716,
        2 => 633,
        3 => 550,
        4 => 466,
        5 => 383,
        6 => 300,
        7 => 216,
        8 => 133,
        _ => 100,
    };
    Duration::from_millis(ms)
}