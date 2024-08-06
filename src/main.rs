use std::{
    borrow::Borrow,
    io::{stdout, Write},
    process,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use anyhow::Result;
use crossterm::{
    cursor::MoveTo,
    event::{
        read, DisableBracketedPaste, DisableFocusChange, DisableMouseCapture, EnableBracketedPaste,
        EnableFocusChange, EnableMouseCapture, Event, KeyCode,
    },
    execute,
    style::{self, style, Print, Stylize},
    terminal::{self, ClearType},
    ExecutableCommand, QueueableCommand,
};

#[derive(Clone)]
enum Color {
    CYAN,
    RED,
    GREEN,
    YELLOW,
    BLUE,
}

#[derive(PartialEq, Clone)]
enum Shape {
    I,
    L,
    O,
    S,
    T,
}

struct Block {
    color: Color,
    shape: Shape,
    rotation: usize,
    x: usize,
    y: usize,
}

struct SolidBlock {
    color: Color,
}

#[derive(Clone)]
struct Cell {
    filled: bool,
    color: Color,
}

fn print_events(act: Arc<Mutex<KeyAction>>) -> Result<()> {
    execute!(
        std::io::stdout(),
        EnableBracketedPaste,
        EnableFocusChange,
        EnableMouseCapture
    )?;
    loop {
        // `read()` blocks until an `Event` is available
        match read()? {
            Event::FocusGained => println!("FocusGained"),
            Event::FocusLost => println!("FocusLost"),
            Event::Key(event) => {
                *act.lock().unwrap() = match event.code {
                    KeyCode::Char('w') => KeyAction::U,
                    KeyCode::Char('s') => KeyAction::D,
                    KeyCode::Char('a') => KeyAction::L,
                    KeyCode::Char('d') => KeyAction::R,
                    _ => KeyAction::N,
                };

                if event.code == KeyCode::Char('c')
                    && event.modifiers == crossterm::event::KeyModifiers::CONTROL
                {
                    println!("\n Exiting... \n goodbye!");
                    process::exit(0);
                }
            }
            Event::Resize(width, height) => println!("New size {}x{}", width, height),
            // case control + c to exit
            _ => (),
        }
    }
    execute!(
        std::io::stdout(),
        DisableBracketedPaste,
        DisableFocusChange,
        DisableMouseCapture
    )?;
    Ok(())
}

#[derive(Debug)]
enum KeyAction {
    U,
    D,
    L,
    R,
    N,
}

fn main() -> Result<()> {
    println!("begin!");

    let mut stdout = stdout();
    let (win_width, win_height) = terminal::size().unwrap();
    let state: Arc<Mutex<Vec<Vec<Cell>>>> = Arc::new(Mutex::new(vec![
        vec![
            Cell {
                color: Color::CYAN,
                filled: false
            };
            win_height as usize
        ];
        win_width as usize
    ]));
    let nstate = Arc::clone(&state);

    let action = Arc::new(Mutex::new(KeyAction::N));
    let naction = Arc::clone(&action);

    let candidateBlock = Arc::new(Mutex::new(Block {
        color: Color::RED,
        shape: Shape::I,
        rotation: 0,
        x: 5,
        y: 0,
    }));

    let candidateBlock_1 = Arc::clone(&candidateBlock);

    thread::spawn(move || {
        print_events(naction);
    });

    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));
        let lock = candidateBlock_1.lock();
        lock.unwrap().y += 1;
    });

    loop {
        stdout.execute(terminal::Clear(ClearType::All))?;

        let mut  candidateBlock = candidateBlock.lock().unwrap();
        
        match *action.lock().unwrap() {
            KeyAction::L => {
                candidateBlock.x -= 1;
            }
            KeyAction::R => {
                candidateBlock.x += 1;
            }
            KeyAction::D => {
                candidateBlock.y += 1;
            }
            _ => {}
        }
        if candidateBlock.shape == Shape::I {
            state.lock().unwrap()[candidateBlock.x][candidateBlock.y].filled = true;
            state.lock().unwrap()[candidateBlock.x][candidateBlock.y + 1].filled = true;
            state.lock().unwrap()[candidateBlock.x][candidateBlock.y + 2].filled = true;
            state.lock().unwrap()[candidateBlock.x][candidateBlock.y + 3].filled = true;
            state.lock().unwrap()[candidateBlock.x][candidateBlock.y + 3].color =
                candidateBlock.color.clone();
        }

        // draw

        for (x, i) in state.lock().unwrap().iter().enumerate() {
            for (y, j) in i.iter().enumerate() {
                let blk = match j.filled {
                    true => "██",
                    false => " ",
                };

                // println!("{}", blk);

                stdout
                    .queue(MoveTo(x as u16, y as u16))?
                    .queue(style::PrintStyledContent(blk.cyan()))?;

                stdout.flush();
            }
        }

        *action.lock().unwrap() = KeyAction::N;

        thread::sleep(Duration::from_millis(500));
    }

    // Ok(())
    // println!("Hello, world!");
}
