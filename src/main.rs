use iced::button;
use iced::{Align, Button, Column, Row, Text, Element, Sandbox, Settings, Length};

const LEN: u32 = 800;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Num(char),
    Sign(char),
    Ans,
    Dot,
    Neg,
    Clear,
    ClearEnd,
    Backspace,
}

#[derive(Default)]
struct Calculator {
    left: String,
    right: String,
    sign: String,
    shadow: bool,

    zero_button: button::State,
    one_button: button::State,
    two_button: button::State,
    three_button: button::State,
    four_button: button::State,
    five_button: button::State,
    six_button: button::State,
    seven_button: button::State,
    eight_button: button::State,
    nine_button: button::State,
    clear_end_button: button::State,
    clear_button: button::State,
    backspace_button: button::State,
    add_button: button::State,
    sub_button: button::State,
    mul_button: button::State,
    div_button: button::State,
    ans_button: button::State,
    dot_button: button::State,
    neg_button: button::State,
}

impl Calculator {
    fn calculate(&mut self) -> std::result::Result<(), &'static str> {
        self.shadow = true;
        self.left = match self.sign.as_str() {
            "+" => {
                format!("{:.10}", self.left.parse::<f64>().unwrap() + self.right.parse::<f64>().unwrap())
            },
            "-" => {
                format!("{:.10}", self.left.parse::<f64>().unwrap() - self.right.parse::<f64>().unwrap())
            },
            "×" => {
                format!("{:.10}", self.left.parse::<f64>().unwrap() * self.right.parse::<f64>().unwrap())
            },
            "÷" => {
                let r = self.right.parse::<f64>().unwrap();
                if r <= std::f64::EPSILON {
                    "Can't divide by zero".into()
                } else {
                    format!("{:.10}", self.left.parse::<f64>().unwrap() / r)
                }
            },
            _ => unreachable!()
        };
        while self.left.len() > 1 {
            if let Some(a) = self.left.pop() {
                if a != '0' && a != '.' {
                    self.left.push(a);
                    break;
                }
            }
        }
        Ok(())
    }
    #[inline]
    fn clear(&mut self, c: char) {
        self.left = c.into();
        self.sign.clear();
        self.right.clear();
        self.shadow = false;
    }
}

impl Sandbox for Calculator {
    type Message = Message;

    fn new() -> Self {
        Self {
            left: "0".into(),
            ..Self::default()
        }
    }

    fn title(&self) -> String {
        "简单计算器 ~ A Simple Caculator".into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Num(n) => {
                if self.sign.is_empty() {
                    if &self.left == "0" {
                        self.left = n.into()
                    } else {
                        self.left.push(n)
                    }
                } else if !self.left.is_empty() && self.shadow {
                    self.clear(n);
                } else {
                    if &self.right == "0" {
                        self.right = n.into()
                    } else {
                        self.right.push(n)
                    }
                }
            },
            Message::Ans => {
                if !self.sign.is_empty() && !self.left.is_empty() && !self.right.is_empty() { let _ = self.calculate(); }
            },
            Message::Sign(s) => {
                if self.sign.is_empty() {
                    self.sign.push(s);
                } else {
                    if !self.shadow && !self.right.is_empty() {
                        let _ = self.calculate();
                    }
                    self.sign = s.into();
                    self.right.clear();
                    self.shadow = false;
                }
            },
            Message::ClearEnd => {
                if self.right.is_empty() {
                    self.left = "0".into();
                    self.sign.clear();
                    self.shadow = false;
                } else {
                    self.right.clear();
                }
            },
            Message::Clear => {
                self.clear('0')
            },
            Message::Backspace => {
                if self.sign.is_empty() {
                    self.left.pop();
                    if self.left.is_empty() { self.left.push('0'); }
                } else {
                    self.right.pop();
                }
            },
            Message::Dot => {
                if self.sign.is_empty() && self.left.find('.').is_none() {
                    self.left.push('.');
                } else if !self.sign.is_empty() && self.right.find('.').is_none() {
                    if self.right.is_empty() { self.right.push('0'); }
                    self.right.push('.');
                }
            }
            Message::Neg => {
                if self.sign.is_empty(){ 
                    if &self.left != "0" && &self.left != "0." {
                        if self.left.find('-').is_none() {
                            self.left.insert(0, '-');
                        } else {
                            self.left.remove(0);
                        }
                    }
                } else if !self.sign.is_empty() && self.shadow {
                    if &self.left != "0" {
                        if self.left.find('-').is_none() {
                            self.left.insert(0, '-');
                        } else {
                            self.left.remove(0);
                        }
                    }
                } else {
                    if &self.right != "0" && &self.left != "0." {
                        if self.right.find('-').is_none() {
                            self.right.insert(0, '-');
                        } else {
                            self.right.remove(0);
                        }
                    }   
                }
            },
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Column::new()
            .max_width(LEN)
            .width(Length::Fill)
            .spacing(2)
            .padding(8)
            .align_items(Align::Center)
            .push(
                    Text::new(
                        if self.shadow {
                            format!("{}", self.left)
                        } else {
                            format!("{} {} {}", self.left, self.sign, self.right)
                        })
                        .size(50)
                        .width(Length::Fill)
                            .horizontal_alignment(iced::HorizontalAlignment::Right)
            )
            .push(
                Row::new()
                    .spacing(1)
                    .align_items(Align::Center)
                    .push(
                        Button::new(&mut self.clear_end_button, 
                            Text::new("CE")
                            .horizontal_alignment(iced::HorizontalAlignment::Center)
                            )
                            .padding(10)
                            .style(style::Button::Func)
                            .on_press(Message::ClearEnd)
                            .width(Length::Fill)
                    )
                    .push(
                        Button::new(&mut self.clear_button, 
                            Text::new("C")
                            .horizontal_alignment(iced::HorizontalAlignment::Center)
                            )
                            .padding(10)
                            .style(style::Button::Func)
                            .on_press(Message::Clear)
                            .width(Length::Fill)
                    )
                    .push(
                        Button::new(&mut self.backspace_button, 
                            Text::new("←")
                            .horizontal_alignment(iced::HorizontalAlignment::Center)
                            )
                            .padding(10)
                            .style(style::Button::Func)
                            .on_press(Message::Backspace)
                            .width(Length::Fill)
                    )
                    .push(
                        Button::new(&mut self.div_button, 
                            Text::new("÷")
                            .horizontal_alignment(iced::HorizontalAlignment::Center)
                            )
                            .padding(10)
                            .style(style::Button::Func)
                            .on_press(Message::Sign('÷'))
                            .width(Length::Fill)
                    )
            )
            .push(
                Row::new()
                    .spacing(1)
                    .align_items(Align::Center)
                    .push(
                        Button::new(
                            &mut self.seven_button, 
                            Text::new("7")
                                .horizontal_alignment(iced::HorizontalAlignment::Center)
                                )
                                .padding(10)
                                .style(style::Button::Num)
                                .on_press(Message::Num('7'))
                                .width(Length::Fill)
                    )
                    .push(
                        Button::new(
                            &mut self.eight_button, 
                            Text::new("8")
                                .horizontal_alignment(iced::HorizontalAlignment::Center)
                                )
                                .padding(10)
                                .style(style::Button::Num)
                                .on_press(Message::Num('8'))
                                .width(Length::Fill)
                    )
                    .push(
                        Button::new(
                            &mut self.nine_button, 
                            Text::new("9")
                                .horizontal_alignment(iced::HorizontalAlignment::Center)
                                )
                                .padding(10)
                                .style(style::Button::Num)
                                .on_press(Message::Num('9'))
                                .width(Length::Fill)
                    )
                    .push(
                        Button::new(
                            &mut self.mul_button, 
                            Text::new("×")
                                .horizontal_alignment(iced::HorizontalAlignment::Center)
                                )
                                .padding(10)
                                .style(style::Button::Func)
                                .on_press(Message::Sign('×'))
                                .width(Length::Fill)
                    )
            )
            .push(
                Row::new()
                    .spacing(1)
                    .align_items(Align::Center)
                    .push(
                        Button::new(
                            &mut self.four_button, 
                            Text::new("4")
                                .horizontal_alignment(iced::HorizontalAlignment::Center)
                                )
                                .padding(10)
                                .style(style::Button::Num)
                                .on_press(Message::Num('4'))
                                .width(Length::Fill)
                    )
                    .push(
                        Button::new(
                            &mut self.five_button, 
                            Text::new("5")
                                .horizontal_alignment(iced::HorizontalAlignment::Center)
                                )
                                .padding(10)
                                .style(style::Button::Num)
                                .on_press(Message::Num('5'))
                                .width(Length::Fill)
                    )
                    .push(
                        Button::new(
                            &mut self.six_button, 
                            Text::new("6")
                                .horizontal_alignment(iced::HorizontalAlignment::Center)
                                )
                                .padding(10)
                                .style(style::Button::Num)
                                .on_press(Message::Num('6'))
                                .width(Length::Fill)
                    )
                    .push(
                        Button::new(
                            &mut self.sub_button, 
                            Text::new("-")
                                .horizontal_alignment(iced::HorizontalAlignment::Center)
                                )
                                .padding(10)
                                .style(style::Button::Func)
                                .on_press(Message::Sign('-'))
                                .width(Length::Fill)
                    )
            )
            .push(
                Row::new()
                    .spacing(1)
                    .align_items(Align::Center)
                    .push(
                        Button::new(
                            &mut self.one_button, 
                            Text::new("1")
                                .horizontal_alignment(iced::HorizontalAlignment::Center)
                            )
                                .padding(10)
                                .style(style::Button::Num)
                                .on_press(Message::Num('1'))
                                .width(Length::Fill)
                    )
                    .push(
                        Button::new(
                            &mut self.two_button, 
                            Text::new("2")
                                .horizontal_alignment(iced::HorizontalAlignment::Center)
                            )
                                .padding(10)
                                .style(style::Button::Num)
                                .on_press(Message::Num('2'))
                                .width(Length::Fill)
                    )
                    .push(
                        Button::new(
                            &mut self.three_button, 
                            Text::new("3")
                                .horizontal_alignment(iced::HorizontalAlignment::Center)
                                )
                                .padding(10)
                                .style(style::Button::Num)
                                .on_press(Message::Num('3'))
                                .width(Length::Fill)
                    )
                    .push(
                        Button::new(
                            &mut self.add_button, 
                            Text::new('+')
                                .horizontal_alignment(iced::HorizontalAlignment::Center)
                                )
                                .padding(10)
                                .style(style::Button::Func)
                                .on_press(Message::Sign('+'))
                                .width(Length::Fill)
                    )
            )
            .push(
                Row::new()
                    .spacing(1)
                    .align_items(Align::Center)
                    .push(
                        Button::new(
                            &mut self.neg_button, 
                            Text::new("+/-")
                                .horizontal_alignment(iced::HorizontalAlignment::Center)
                                )
                                .padding(10)
                                .style(style::Button::Num)
                                .on_press(Message::Neg)
                                .width(Length::Fill)
                    )
                    .push(
                        Button::new(
                            &mut self.zero_button, 
                            Text::new("0")
                                .horizontal_alignment(iced::HorizontalAlignment::Center)
                            )
                            .padding(10)
                            .style(style::Button::Num)
                            .on_press(Message::Num('0'))
                            .width(Length::Fill)

                    )
                    .push(
                        Button::new(
                            &mut self.dot_button, 
                            Text::new(".")
                                .horizontal_alignment(iced::HorizontalAlignment::Center)
                            )
                            .padding(10)
                            .style(style::Button::Num)
                            .on_press(Message::Dot)
                            .width(Length::Fill)
                    )
                    .push(
                        Button::new(
                            &mut self.ans_button, 
                            Text::new("=")
                                .horizontal_alignment(iced::HorizontalAlignment::Center)
                            )
                            .padding(10)
                            .style(style::Button::Ans)
                            .on_press(Message::Ans)
                            .width(Length::Fill)
                    )
            )
            .push(
                Text::new("0.1.0 - Shanghai")
                    .size(9)
                    .horizontal_alignment(iced::HorizontalAlignment::Right)
                    .width(Length::Fill)
            )
            .into()
    }
}

mod style {
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        Ans,
        Num,
        Func,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Ans => Color::from_rgb8(69, 153, 219),
                    Button::Num => Color::from_rgb8(242, 243, 242),
                    Button::Func => Color::from_rgb8(216, 218, 217),
                })),
                border_radius: match self {
                    _ => 1.0,
                },
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: match self {
                    Button::Ans => Color::WHITE,
                    _ => Color::BLACK,
                },
                ..button::Style::default()
            }
        }
    }
}

fn main() -> iced::Result {
    Calculator::run(
        Settings {
            window: iced::window::Settings { 
                size: (800, 286), 
                resizable: false, 
                transparent: false,
                ..iced::window::Settings::default()
            },
            ..Settings::default()
        }
    )
}
