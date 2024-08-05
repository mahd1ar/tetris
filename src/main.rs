use std::{
    io::stdout,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use anyhow::Result;
use crossterm::{
    cursor::MoveTo,
    style::Print,
    terminal::{self, ClearType},
    ExecutableCommand,
};

fn main() -> Result<()> {
    println!("begin!");
    let mut stdout = stdout();

    let state: Arc<Mutex<Vec<Vec<usize>>>> = Arc::new(Mutex::new(vec![vec!(3; 3); 0]));

    let nstate = Arc::clone(&state);
    thread::spawn(move || {
        // block

        thread::sleep(Duration::new(3, 0));
        nstate.lock().unwrap().push(vec![0]);
    });

    loop {
        stdout.execute(terminal::Clear(ClearType::All))?;
        thread::sleep(Duration::from_millis(500));

        for x in 0..state.lock().unwrap().len() {
            for y in 0..state.lock().unwrap()[0].len() {
                let cord = state.lock().unwrap()[x][y];

                let blk = match cord {
                    1 => "]",
                    0 => "x",
                    _ => "_",
                };

                stdout
                    .execute(MoveTo(x as u16, y as u16))?
                    .execute(Print(blk))?;
            }
        }
        // println!("{:?}" , state);
        thread::sleep(Duration::from_millis(500));
    }

    // Ok(())
    // println!("Hello, world!");
}
