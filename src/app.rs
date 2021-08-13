use eframe::{egui, epi};
use egui::Widget;
use mathtutor::*;
use std::collections::HashSet;

//#[derive(Serialize, Deserialize, Debug)]
pub struct Round {
    right: i32,
    wrong: i32,
    timers: Vec<f32>,
    //time: SystemTime
}

impl Default for Round {
    fn default() -> Self {
        Self {
            right: 0,
            wrong: 0,
            timers: vec![],
            //time: SystemTime::now()
        }
    }
}
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct GameState {
    rounds: Vec<GameRound>,
    collected_animals: HashSet<Animal>,
    settings_active: bool,
    message_timer: i32,
    message: String
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            settings_active: false,
            rounds: vec![],
            collected_animals: HashSet::new(),
            message_timer: 200,
            message: "".to_string()
        }
    }
}

impl epi::App for GameState {
    fn name(&self) -> &str {
        "mathe!"
    }

    /// Called by the framework to load old app state (if any).
    #[cfg(feature = "persistence")]
    fn load(&mut self, storage: &dyn epi::Storage) {
        *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
    }

    /// Called by the frame work to save state before shutdown.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let GameState {
            settings_active,
            rounds,
            collected_animals,
            message_timer,
            message
        } = self;

        ctx.set_visuals(egui::Visuals::light());
        let mut style: egui::Style = (*ctx.style()).clone();
        // style.visuals.

        egui::SidePanel::left("side_panel", 200.0).show(ctx, |ui| {
            ui.label("Deine Tiere");
            //ctx.settings_ui(ui);

            let mut font_definitions = ctx.fonts().definitions().clone();
            //font_definitions.ui(ui);

            let s = font_definitions
                .family_and_size
                .get_mut(&egui::TextStyle::Body)
                .unwrap();
            s.1 = 23.;
            let s = font_definitions
                .family_and_size
                .get_mut(&egui::TextStyle::Heading)
                .unwrap();
            s.1 = 66.;

            ctx.set_fonts(font_definitions);

            // The rewards
            egui::ScrollArea::auto_sized().show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = egui::Vec2::splat(2.0);
                    for animal in collected_animals.iter() {
                        ui.heading(animal.to_string());
                    }
                });
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                if ui.button("Einstellungen").clicked() {
                    *settings_active = !&*settings_active;
                }
            });
        });

        if *settings_active {
            egui::SidePanel::left("settings_panel", 400.0).show(ctx, |ui| {
                ui.label("Einstellungen");
                // let mut gr = GameRound::default();

                if let Some(gr) = rounds.last_mut() {
                    ui.label("Aufgabenlaenge");
                    if ui.add(egui::DragValue::u8(&mut gr.ruleset.operands).speed(1.0)).dragged() {
                        //rounds.push(gr.clone());
                        gr.active_question = Question::generate(gr.ruleset.clone());

                    }
                    
                    let mut end = *gr.ruleset.range.end();
                    ui.label("Hoechste Zahl");
                    if ui.add(egui::DragValue::i32(&mut end).speed(1.0)).dragged() {
                        gr.ruleset.range = 0..=end;
                        //rounds.push(gr.clone());
                    gr.active_question = Question::generate(gr.ruleset.clone());


                    }

                    ui.label("Erlaubte Rechenzeichen");
                    if gr.ruleset.operators.len() >= 1 {
                        for op in &gr.ruleset.operators.clone() {
                            if ui
                                .button(format!("'{}' nicht verwenden", op.as_str()))
                                .clicked()
                            {
                                gr.ruleset.operators.remove(op);
                            }
                        }
                    }

                    ui.label("Verfuegbare Rechenzeichen");
                    if !gr.ruleset.operators.contains(&Operator::Plus) {
                        if ui.button("'+' verwenden").clicked() {
                            gr.ruleset.operators.insert(Operator::Plus);
                        }
                    }

                    if !gr.ruleset.operators.contains(&Operator::Minus) {
                        if ui.button("'-' verwenden").clicked() {
                            gr.ruleset.operators.insert(Operator::Minus);
                        }
                    }
                    if !gr.ruleset.operators.contains(&Operator::Mult) {
                        if ui.button("'*' verwenden").clicked() {
                            gr.ruleset.operators.insert(Operator::Mult);
                        }
                    }
                    if !gr.ruleset.operators.contains(&Operator::Div) {
                        if ui.button("'/' verwenden").clicked() {
                            gr.ruleset.operators.insert(Operator::Div);
                        }
                    }
                }

                if ui.button("!!! Alles zuruecksetzen").clicked() {
                    collected_animals.clear();
                    rounds.clear();
                    rounds.push(GameRound::default());

                }

                // Add new game round
                // if ui.button("Neues spiel").clicked() {
                //     rounds.push(gr);
                //     //let (question, solution, num_digits) = question_pair(0..=5, 0..=5, Operator::Plus);
                // }
            });
        }

        // egui::TopPanel::top("top_panel").show(ctx, |ui| {
        //     // The top panel is often a good place for a menu bar:
        //     egui::menu::bar(ui, |ui| {
        //         egui::menu::menu(ui, "File", |ui| {
        //             if ui.button("Quit").clicked() {
        //                 frame.quit();
        //             }
        //         });
        //     });
        // });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::warn_if_debug_build(ui);

            if let Some(gr) = rounds.last_mut() {
                ui.label(&gr.active_question.description);
                ui.text_edit_singleline(&mut gr.active_question.answer);
                if gr.active_question.solved() {
                    gr.active_question = Question::generate(gr.ruleset.clone());
                    collected_animals.insert(random_animal());
                    *message_timer = 200;
                    *message = "RICHTIG!".into();
                }
            }
            //ui.add(egui::Label())
       
            if *message_timer > 0 {
                ui.label(&*message);
            }
        });

        *message_timer -=1;


        // egui::Window::new("Window").show(ctx, |ui| {
        //     ui.label("Windows can be moved by dragging them.");
        //     ui.label("They are automatically sized based on contents.");
        //     ui.label("You can turn on resizing and scrolling if you like.");
        //     ui.label("You would normally chose either panels OR windows.");
        // });
    }
}

// ----------------------------------------------------------------------------
