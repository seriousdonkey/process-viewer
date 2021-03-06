use graph::Graph;

use gio;
use glib::Cast;
use gtk::{GtkApplicationExt, Inhibit, WidgetExt};

use std::cell::RefCell;
use std::ops::Index;
use std::rc::Rc;

pub const MAIN_WINDOW_NAME: &str = "main-window";

#[derive(Debug)]
pub struct RotateVec<T> {
    data: Vec<T>,
    start: usize,
}

impl<T> RotateVec<T> {
    pub fn new(d: Vec<T>) -> RotateVec<T> {
        RotateVec { data: d, start: 0 }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn move_start(&mut self) {
        if self.start > 0 {
            self.start -= 1;
        } else {
            self.start = self.data.len() - 1;
        }
    }

    /*pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(self.get_real_pos(index))
    }*/

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        let pos = self.get_real_pos(index);
        self.data.get_mut(pos)
    }

    fn get_real_pos(&self, index: usize) -> usize {
        if self.start + index >= self.data.len() {
            self.start + index - self.data.len()
        } else {
            index + self.start
        }
    }
}

pub fn format_number(mut nb: u64) -> String {
    if nb < 1000 {
        return format!("{} B", nb);
    }
    nb >>= 10; // / 1_024
    if nb < 100_000 {
        format!("{} kB", nb)
    } else if nb < 10_000_000 {
        format!("{} MB", nb >> 10) // / 1_024
    } else if nb < 10_000_000_000 {
        format!("{} GB", nb >> 20) // / 1_048_576
    } else {
        format!("{} TB", nb >> 30) // / 1_073_741_824
    }
}

pub fn connect_graph(graph: Graph) -> Rc<RefCell<Graph>> {
    let area = graph.area.clone();
    let graph = Rc::new(RefCell::new(graph));
    area.connect_draw(
        clone!(@weak graph => @default-return Inhibit(false), move |w, c| {
            graph.borrow()
                 .draw(c,
                       f64::from(w.get_allocated_width()),
                       f64::from(w.get_allocated_height()));
            Inhibit(false)
        }),
    );
    graph
}

impl<T> Index<usize> for RotateVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.data[self.get_real_pos(index)]
    }
}

pub fn get_app() -> gtk::Application {
    gio::Application::get_default()
        .expect("No default application")
        .downcast::<gtk::Application>()
        .expect("Default application has wrong type")
}

pub fn get_main_window() -> Option<gtk::Window> {
    for window in get_app().get_windows() {
        if window.get_widget_name().as_ref().map(|ref s| s.as_str()) == Some(MAIN_WINDOW_NAME) {
            return Some(window);
        }
    }
    None
}
