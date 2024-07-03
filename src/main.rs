mod todo_list;

use iced::{Sandbox, Settings};
use todo_list::TodoList;

fn main() -> iced::Result {
    TodoList::run(Settings::default())
}
