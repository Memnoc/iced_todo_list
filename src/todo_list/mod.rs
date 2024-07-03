mod todo_item;

use iced::widget::{button, checkbox, text_input, Column, Container, Row, Text};
use iced::{theme, Element, Length, Sandbox};
use todo_item::TodoItem;

pub struct TodoList {
    new_todo: String,
    todos: Vec<TodoItem>,
    filter: Filter,
}

#[derive(Debug, Clone)]
pub enum Message {
    NewTodoChanged(String),
    AddTodo,
    ToggleTodo(usize),
    FilterChanged(Filter),
    ClearCompleted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Sandbox for TodoList {
    type Message = Message;

    fn new() -> Self {
        TodoList {
            new_todo: String::new(),
            todos: Vec::new(),
            filter: Filter::All,
        }
    }

    fn title(&self) -> String {
        String::from("Iced Todo List")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::NewTodoChanged(value) => self.new_todo = value,
            Message::AddTodo => {
                if !self.new_todo.is_empty() {
                    self.todos.push(TodoItem {
                        description: self.new_todo.clone(),
                        completed: false,
                    });
                    self.new_todo.clear();
                }
            }
            Message::ToggleTodo(index) => {
                if let Some(todo) = self.todos.get_mut(index) {
                    todo.completed = !todo.completed;
                }
            }
            Message::FilterChanged(filter) => self.filter = filter,
            Message::ClearCompleted => self.todos.retain(|todo| !todo.completed),
        }
    }

    fn view(&self) -> Element<Message> {
        let title = Text::new("Todo List").size(40);

        let input = text_input("What needs to be done?", &self.new_todo)
            .on_input(Message::NewTodoChanged)
            .padding(15);

        let add_button = button("Add Todo").on_press(Message::AddTodo).padding(15);

        let input_row = Row::new().spacing(20).push(input).push(add_button);

        let filter_buttons = Row::new()
            .spacing(10)
            .push(filter_button("All", Filter::All, self.filter))
            .push(filter_button("Active", Filter::Active, self.filter))
            .push(filter_button("Completed", Filter::Completed, self.filter));

        let clear_completed_button = button("Clear completed").on_press(Message::ClearCompleted);

        let todos: Element<_> = self
            .todos
            .iter()
            .enumerate()
            .filter(|(_, todo)| match self.filter {
                Filter::All => true,
                Filter::Active => !todo.completed,
                Filter::Completed => todo.completed,
            })
            .fold(Column::new().spacing(10), |column, (index, todo)| {
                column.push(checkbox(&todo.description, todo.completed, move |_| {
                    Message::ToggleTodo(index)
                }))
            })
            .into();

        Container::new(
            Column::new()
                .push(title)
                .push(input_row)
                .push(filter_buttons)
                .push(todos)
                .push(clear_completed_button)
                .spacing(20),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .padding(20)
        .into()
    }
}

fn filter_button(label: &str, filter: Filter, current_filter: Filter) -> Element<Message> {
    button(Text::new(label))
        .on_press(Message::FilterChanged(filter))
        .style(if filter == current_filter {
            theme::Button::Primary
        } else {
            theme::Button::Secondary
        })
        .into()
}
