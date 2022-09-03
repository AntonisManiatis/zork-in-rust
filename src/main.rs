use std::{io, collections::HashMap};

struct Tile {

}

const MAX_X: usize = 16;
const MAX_Y: usize = 16;

struct CLI {
    stopping: bool
}

impl CLI {
    pub fn new() -> Self {
        Self { stopping: false }
    }

    pub fn stop(&mut self) {
        self.stopping = true;
    }
}

const PARDON_ACTION: &dyn Action = &PardonAction {};

fn main() {
    /* 
    const tiles:[[Tile; MAX_X]; MAX_Y] = [[8; MAX_X]; MAX_Y];

    for (i, row) in tiles.iter_mut().enumerate() {
        for (y, col) in tiles.iter_mut().enumerate() {
            println!("{:?}", col);
        }
    }
    */

    let cli = CLI::new();

    // TODO: Should be immutable after construction
    let mut actions = HashMap::<&str, &dyn Action>::new();
    actions.insert("quit", &QuitAction{ cli: &cli });

    let mut buffer = String::new();

    while !cli.stopping {
        io::stdin().read_line(&mut buffer).unwrap();

        let parts: Vec<&str> = buffer.strip_suffix("\r\n") // TODO: Not windows specific
            .unwrap()
            .split(" ").collect();

        let command = parts[0];

        let executable = actions.get(command)
            .or(Some(&PARDON_ACTION)); // I cannot seem to return just a reference and not an option of a reference.
        
        executable.unwrap().execute();

        buffer.clear();
    }
}

// TODO: Move those into their own module when they get many.
pub trait Action { // I have no idea what the 'static is. I think it's called lifetime?
    fn execute(&self); // void result? how do we know we faulted? Option perhaps? ^^
}

struct PardonAction;

impl Action for PardonAction {
    fn execute(&self) {
        println!("I beg your pardon?")
    }
}

struct QuitAction<'a> {
    cli: &'a CLI
}

impl Action for QuitAction<'_> {
    fn execute(&self) {
        println!("bye!");
    }
}