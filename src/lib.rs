use std::sync::mpsc::{self, Receiver, Sender};

use tui::widgets::ListState;

pub mod sniffer;
pub mod ui;

pub struct StateList<T> {
    items: Vec<T>,
    state: ListState,
}

impl<T> StateList<T> {
    pub fn new() -> StateList<T> {
        StateList {
            items: vec![],
            state: ListState::default(),
        }
    }
}

pub struct App {
    // TODO: Change string for a made struct
    pub list: StateList<String>,
    pub rx: Receiver<String>,
    pub scroll: usize,
}

impl App {
    pub fn new() -> (App, Sender<String>) {
        let (tx, rx) = mpsc::channel();
        (
            App {
                list: StateList::new(),
                rx,
                scroll: 0,
            },
            tx,
        )
    }
}
