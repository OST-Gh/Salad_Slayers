#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use {eframe::egui, rand::*, rand::distributions::{Distribution, Standard}};
fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Puncher",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}
#[derive(Debug,Clone,PartialEq)]
enum Punch {
    None,
    Hit,
    Dodge,
    Counter,
}
enum Direction {
    None,
    Backward(u32),
    Forward(u32),
}
#[derive(Debug,Clone)]
struct Entity {
    name: &'static str,
    health: u32,
}
struct MyApp {
    punch_type: Punch,
    wins: (u16, u16),
    entities: (Entity, Entity),
}
impl Default for MyApp {
    fn default() -> Self {
        Self {
            punch_type: Punch::None,
            wins: (0, 0),
            entities: (
                Entity {
                    name: "Player",
                    health: 100,
                },
                Entity {
                    name: "Kanade",
                    health: 100,
                }
            )
        }
    }
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Kanade - Puncher");
            ui.label("Punch Kanade as hard as you can!\nBut watch out she'll fight back...");
            ui.horizontal(|ui| {
                ui.label(&format!("[ {} ]", self.punch_type));
                ui.label(&format!("{}'s Health {} | {}'s Health {}", self.entities.0.name, self.entities.0.health, self.entities.1.name, self.entities.1.health));
            });
            if self.entities.0.health < 6 {
                self.entities.0.health = 100;
                self.entities.1.health = 100;
                self.wins.0 += 1
            }
            if self.entities.1.health < 6 {
                self.entities.0.health = 100;
                self.entities.1.health = 100;
                self.wins.1 += 1
            }
            if ui.button("Punch!").clicked() {
                self.punch_type = random::<Punch>();
                self.entities = self.entities.1.clone().damage(self.entities.0.clone(), self.punch_type.clone());
            }
            if self.punch_type == Punch::Counter {
                let n1 = thread_rng().gen_range(0..=2) ;
                let n2 = thread_rng().gen_range(0..=2) ;
                let n3 = thread_rng().gen_range(0..=2) ;
                ui.horizontal(|ui| {
                    if ui.button("Counter!").clicked() {
                        if n1 == 2 {
                            self.punch_type = Punch::Hit;
                            self.entities = self.entities.1.clone().damage(self.entities.0.clone(), self.punch_type.clone());
                            println!("Success!");
                        } else {
                            self.entities = self.entities.1.clone().damage(self.entities.0.clone(), self.punch_type.clone());
                            self.punch_type = Punch::None;
                        }
                    }
                    if ui.button("Counter!").clicked() {
                        if n2 == 2 {
                            self.punch_type = Punch::Hit;
                            self.entities = self.entities.1.clone().damage(self.entities.0.clone(), self.punch_type.clone());
                            println!("Success!");
                        } else {
                            self.entities = self.entities.1.clone().damage(self.entities.0.clone(), self.punch_type.clone());
                            self.punch_type = Punch::None;
                        }
                    }
                    if ui.button("Counter!").clicked() {
                        if n3 == 2 {
                            self.punch_type = Punch::Hit;
                            self.entities = self.entities.1.clone().damage(self.entities.0.clone(), self.punch_type.clone());
                            println!("Success!");
                        } else {
                            self.entities = self.entities.1.clone().damage(self.entities.0.clone(), self.punch_type.clone());
                            self.punch_type = Punch::None;
                        }
                    }
                });
            } ;
            ui.label(&format!("| {}'s Win Count : {} |\n| {}'s Win Count : {} |", self.entities.0.name, self.wins.1, self.entities.1.name, self.wins.0))
        });
    }
}
impl Distribution<Punch> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Punch {
        // match rng.gen_range(0, 3) { // rand 0.5, 0.6, 0.7
        match rng.gen_range(0..=2) { // rand 0.8
            0 => Punch::Hit,
            1 => Punch::Dodge,
            2 => Punch::Counter,
            _ => Punch::None,
        }
    }
}
impl Entity {
    fn damage(self, dealer: Entity, punch: Punch) -> (Entity, Entity) {
        let health: Direction = match punch {
            Punch::Hit => Direction::Forward(5),
            Punch::Dodge => Direction::None,
            Punch::Counter => Direction::Backward(5),
            Punch::None => Direction::None,
        };
        match health {
            Direction::Forward(value) => (
                Entity {
                    name: dealer.name,
                    health: dealer.health,
                },
                Entity {
                    name: self.name,
                    health: self.health - value,
                },
            ),
            Direction::Backward(value) => (
                Entity {
                    name: dealer.name,
                    health: dealer.health - value,
                },
                Entity {
                    name: self.name,
                    health: self.health,
                },
            ),
            Direction::None => (
                Entity {
                    name: dealer.name,
                    health: dealer.health,
                },
                Entity {
                    name: self.name,
                    health: self.health,
                },
            ),
        }
    }
}
use std::fmt::*;
impl Display for Punch {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match &self {
            Punch::Hit => write!(f, "Direct Hit"),
            Punch::Dodge => write!(f, "Dodged"),
            Punch::Counter => write!(f, "Countered"),
            Punch::None => write!(f, "Nothing"),
        }
    }
}