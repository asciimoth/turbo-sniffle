use std::io::{stdout, Write};
use crossterm::{
    terminal,
    execute,
    ExecutableCommand, QueueableCommand,
    cursor, style::{self, Stylize}, self
};
use crossterm::event::{poll, read, Event, KeyCode, KeyModifiers};
use std::time::Duration;
use std::cmp::max;


enum InputMode {
    Cmd,
    Key,
}

impl InputMode {
    fn name(&self) -> &str {
        match self {
            Self::Cmd => "cmd",
            Self::Key => "mov",
        }
    }
    fn next(&self) -> Self{
        match self {
            Self::Cmd => Self::Key,
            Self::Key => Self::Cmd,
        }
    }
}

fn clear_row(){
    let width = match terminal::window_size() {
        Ok(size) => size.columns,
        Err(_) => 80,
    };
    print!("\r");
    for _ in 0..width {
        print!(" ");
    }
}

fn print_input(inp: &str) -> std::io::Result<()>{
    let cols = terminal::window_size()?.columns as isize;
    let col  = cursor::position()?.0 as isize;
    let available = cols-col;
    let skip = max(0, inp.len() as isize - available) as usize;
    let text: String = inp.chars().into_iter().skip(skip).collect();
    print!("{}", text);
    Ok(())
}

fn print_plane(cols: usize, rows: usize, c_col: usize, c_row: usize) {
    //clear_row();
    print!("\r");
    for col in 0..cols+2{
        if col == 0 || col == cols+1 {
            print!("+");
            continue
        }
        print!("-");
    }
    print!("\n\r");
    for row in 0..rows {
        for col in 0..cols+2 {
            if col == 0 || col == cols+1 {
                print!("|");
                continue
            }
            if col-1 == c_col && row == c_row {
                print!("@");
                continue
            }
            print!(" ");
        }
        print!("\n\r");
    }
    for col in 0..cols+2{
        if col == 0 || col == cols+1 {
            print!("+");
        }else{
            print!("-");
        }
    }
    println!();
}

fn main() {
    let mut stdout = stdout();
    let mut input = String::new();
    terminal::enable_raw_mode().unwrap();
    print!("Wellcum\n\r");
    let mut mode = InputMode::Key;
    let rows: usize = 8;
    let cols: usize = rows*2;
    let mut col: usize = cols/2;
    let mut row: usize = rows/2;
    print_plane(cols, rows, col, row);
    loop {
        clear_row();
        print!("\r{}", mode.name());
        match mode {
            InputMode::Cmd => {
                if input.len() > 0 {
                    print!("| ");
                } else {
                    print!("  {} \r{}|", "Type your command here".with(style::Color::DarkGrey), mode.name());
                }
                print_input(&input).unwrap();
            }
            _ => {}
        }
        stdout.flush().unwrap();
        let event = match read().unwrap() {
            Event::Key(key_event) => {
                key_event
            }
            _ => { continue }
        };
        if let KeyCode::Null = event.code { continue }
        if event.modifiers.contains(KeyModifiers::CONTROL) {
            if let KeyCode::Char('q') = event.code { break }
        }
        match mode {
            InputMode::Key => {
                stdout.flush().unwrap();
                match event.code {
                    // Quit
                    KeyCode::Char('q') => { break }
                    KeyCode::Char('w') => {
                        print!("\rNew save writed\n");
                    }
                    KeyCode::Char('i') => { // Idle
                        print_plane(cols, rows, col, row);
                    }
                    KeyCode::Char('h') => { // ←
                        if col > 0 { col -= 1 }
                        print_plane(cols, rows, col, row);
                    }
                    KeyCode::Char('j') => { // ↓
                        if row < rows-1 { row += 1 }
                        print_plane(cols, rows, col, row);
                    }
                    KeyCode::Char('k') => { // ↑
                        if row > 0 { row -= 1 }
                        print_plane(cols, rows, col, row);
                    }
                    KeyCode::Char('l') => { // →
                        if col < cols-1 { col += 1 }
                        print_plane(cols, rows, col, row);
                    }
                    KeyCode::Char(c) => {
                        //print!("\r           \r{}\n", c);
                    }
                    KeyCode::Esc => {
                        mode = mode.next();
                    }
                    _ => {}
                }
                stdout.flush().unwrap();
            }
            InputMode::Cmd => {
                match event.code {
                    KeyCode::Char(c) => {
                        input.push(c);
                    }
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Esc => {
                        mode = mode.next();
                        input = String::new();
                        clear_row();
                    }
                    KeyCode::Enter => {
                        clear_row();
                        print!("\r[CMD]: {}\n\r", input);
                        input = String::new();
                    }
                    _ => {}
                }
            }
        }
    }
    print!("\n\rBye\n\r");
    stdout.flush().unwrap();
    terminal::disable_raw_mode().unwrap();
}


/*
TODO
- Ask dialogs
    - Ask about saving on exit
- CMD mode cursor moves
*/












