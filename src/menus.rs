pub enum MenuOptions {
    Start,
    Quit,
    Restart,
    NoSelection
}

pub trait HasOptions {}
impl HasOptions for MenuOptions {}