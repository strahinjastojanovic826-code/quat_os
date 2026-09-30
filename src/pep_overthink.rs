use eframe::egui;
use crate::kernel::QuatKernel4x8;

/// Taktička uloga igrača definisana kvatnim naponskim nivoom
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QuatRole {
    InvertedFullback0V = 0, // Q0: 0.0V - Ulazak u sredinu
    HalfSpaceOperator11V = 1, // Q1: 1.1V - Plutanje između linija
    FalseNineCreator22V = 2,  // Q2: 2.2V - Lažna devetka / Kreator
    TotalChaosEmergency33V = 3, // Q3: 3.3V - Svi u napad / Panika
}

/// Igrač na kvatnom terenu
#[derive(Clone, Debug)]
pub struct QuatPlayer {
    pub name: String,
    pub number: u8,
    pub role: QuatRole,
    pub x_pos: f32, // 0.0 do 1.0 (Širina terena)
    pub y_pos: f32, // 0.0 do 1.0 (Dužina terena)
    pub voltage: f32,
}

/// Pep Guardiola 4^8 Tactical Engine
pub struct PepOverthinkEngine {
    pub pitch_players: Vec<QuatPlayer>,
    pub match_importance: f32, // 0.0 (Prvenstvo protiv Burnleya) do 1.0 (Finale LŠ)
    pub pitch_humidity: f32,   // Utiče na izbore bekova
    pub hair_loss_index: f32,  // Multiplikator genijalnosti
    pub overthink_risk: f32,   // Procenat rizika od prekomplikovanja
    pub selected_player_idx: Option<usize>,
    pub tactical_logs: Vec<String>,
}

impl Default for PepOverthinkEngine {
    fn default() -> Self {
        let players = vec![
            QuatPlayer { name: "Ederson".into(), number: 31, role: QuatRole::InvertedFullback0V, x_pos: 0.5, y_pos: 0.08, voltage: 0.0 },
            QuatPlayer { name: "J. Stones".into(), number: 5, role: QuatRole::InvertedFullback0V, x_pos: 0.5, y_pos: 0.35, voltage: 0.0 },
            QuatPlayer { name: "R. Dias".into(), number: 3, role: QuatRole::InvertedFullback0V, x_pos: 0.3, y_pos: 0.22, voltage: 0.0 },
            QuatPlayer { name: "K. Walker".into(), number: 2, role: QuatRole::InvertedFullback0V, x_pos: 0.7, y_pos: 0.22, voltage: 0.0 },
            QuatPlayer { name: "Rodri".into(), number: 16, role: QuatRole::HalfSpaceOperator11V, x_pos: 0.5, y_pos: 0.50, voltage: 1.1 },
            QuatPlayer { name: "K. De Bruyne".into(), number: 17, role: QuatRole::HalfSpaceOperator11V, x_pos: 0.35, y_pos: 0.68, voltage: 1.1 },
            QuatPlayer { name: "B. Silva".into(), number: 20, role: QuatRole::FalseNineCreator22V, x_pos: 0.65, y_pos: 0.68, voltage: 2.2 },
            QuatPlayer { name: "P. Foden".into(), number: 47, role: QuatRole::HalfSpaceOperator11V, x_pos: 0.18, y_pos: 0.78, voltage: 1.1 },
            QuatPlayer { name: "J. Grealish".into(), number: 10, role: QuatRole::FalseNineCreator22V, x_pos: 0.82, y_pos: 0.78, voltage: 2.2 },
            QuatPlayer { name: "E. Haaland".into(), number: 9, role: QuatRole::TotalChaosEmergency33V, x_pos: 0.5, y_pos: 0.88, voltage: 3.3 },
            QuatPlayer { name: "J. Alvarez".into(), number: 19, role: QuatRole::TotalChaosEmergency33V, x_pos: 0.5, y_pos: 0.76, voltage: 3.3 },
        ];

        Self {
            pitch_players: players,
            match_importance: 0.85, // Podrazumevano: Polufinale Lige Šampiona
            pitch_humidity: 62.0,
            hair_loss_index: 0.99,
            overthink_risk: 78.4,
            selected_player_idx: None,
            tactical_logs: vec![
                "[PEP 4^8] Tactical Engine initialized in 65,536 permutations mode.".to_string(),
                "[Juego de Posición] Rest defense stabilized with Q0 (0.0V) inverted pivot.".to_string(),
                "[WARNING] High match importance detected! Overthink algorithm activating...".to_string(),
            ],
        }
    }
}

impl PepOverthinkEngine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Proračunava Overthink Rizik na osnovu parametara okruženja i procesorskih registara
    pub fn calculate_overthink(&mut self, kernel: &QuatKernel4x8) {
        let reg_a_factor = (kernel.reg_a.0 as f32) / 65535.0;
        self.overthink_risk = (self.match_importance * 60.0) 
            + (self.pitch_humidity * 0.2) 
            + (reg_a_factor * 20.0);
        
        self.overthink_risk = self.overthink_risk.clamp(0.0, 100.0);
    }

    /// Renderovanje Pep Tactical UI Tab-a
    pub fn ui(&mut self, ui: &mut egui::Ui, kernel: &mut QuatKernel4x8) {
        self.calculate_overthink(kernel);

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.heading("⚽ Pep Guardiola 4^8 Tactical Overthink Engine");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let risk_color = if self.overthink_risk > 80.0 {
                        egui::Color32::LIGHT_RED
                    } else if self.overthink_risk > 50.0 {
                        egui::Color32::GOLD
                    } else {
                        egui::Color32::GREEN
                    };
                    ui.label(
                        egui::RichText::new(format!("OVERTHINK RIZIK: {:.1}%", self.overthink_risk))
                            .strong()
                            .font(egui::FontId::proportional(14.0))
                            .color(risk_color),
                    );
                });
            });

            ui.label(
                egui::RichText::new("Naponsko mapiranje igrača kroz $4^8$ logička stanja: Q0 (0.0V Inverted) | Q1 (1.1V Half-Space) | Q2 (2.2V False 9) | Q3 (3.3V Chaos)")
                    .font(egui::FontId::proportional(11.0))
                    .color(egui::Color32::GRAY),
            );
        });

        ui.separator();
        ui.add_space(4.0);

        ui.columns(2, |cols| {
            // LEVI PANEL: PARAMEPRI & CONTROLS
            cols[0].vertical(|ui| {
                ui.group(|ui| {
                    ui.label(egui::RichText::new("⚙️ Taktički Parametri Utakmice").strong());
                    ui.separator();

                    ui.add(egui::Slider::new(&mut self.match_importance, 0.0..=1.0).text("Značaj Utakmice (0=Kup, 1=Finale LŠ)"));
                    ui.add(egui::Slider::new(&mut self.pitch_humidity, 0.0..=100.0).text("Vlažnost Trave (%)"));
                    ui.add(egui::Slider::new(&mut self.hair_loss_index, 0.0..=1.0).text("Bald Magic Multiplier (BMM)"));

                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        if ui.button("🚨 DUPLIRAJ ZADNJE VEZNE (Q0)").clicked() {
                            for p in &mut self.pitch_players {
                                if p.role == QuatRole::HalfSpaceOperator11V {
                                    p.role = QuatRole::InvertedFullback0V;
                                    p.voltage = 0.0;
                                }
                            }
                            self.tactical_logs.push("[PEP] Rodri and Stones paired in 0.0V low-voltage double pivot!".to_string());
                        }

                        if ui.button("🔥 UCL FINAL SPECIAL (0 DM / 5 Krila)").clicked() {
                            for p in &mut self.pitch_players {
                                p.role = QuatRole::TotalChaosEmergency33V;
                                p.voltage = 3.3;
                            }
                            self.tactical_logs.push("[OVERTHINK WARNING] Zero holding midfielders! All units set to 3.3V Overdrive!".to_string());
                        }
                    });
                });

                ui.add_space(6.0);

                // INTERAKTIVNA LISTA IGRAČA
                ui.group(|ui| {
                    ui.label(egui::RichText::new("📋 Roster i Naponske Uloge ($4^8$)").strong());
                    ui.separator();

                    egui::ScrollArea::vertical().max_height(180.0).show(ui, |ui| {
                        egui::Grid::new("pep_players_grid").striped(true).min_col_width(70.0).show(ui, |ui| {
                            ui.strong("#");
                            ui.strong("Igrač");
                            ui.strong("Kvat Uloga");
                            ui.strong("Napon");
                            ui.end_row();

                            for (idx, p) in self.pitch_players.iter_mut().enumerate() {
                                ui.label(format!("#{}", p.number));
                                ui.label(&p.name);

                                // Promena uloge u hodnika
                                egui::ComboBox::from_id_salt(format!("combo_{}", idx))
                                    .selected_text(format!("{:?}", p.role))
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut p.role, QuatRole::InvertedFullback0V, "Q0: Inverted Fullback (0.0V)");
                                        ui.selectable_value(&mut p.role, QuatRole::HalfSpaceOperator11V, "Q1: Half-Space Operator (1.1V)");
                                        ui.selectable_value(&mut p.role, QuatRole::FalseNineCreator22V, "Q2: False Nine (2.2V)");
                                        ui.selectable_value(&mut p.role, QuatRole::TotalChaosEmergency33V, "Q3: Chaos/Box Threat (3.3V)");
                                    });

                                p.voltage = match p.role {
                                    QuatRole::InvertedFullback0V => 0.0,
                                    QuatRole::HalfSpaceOperator11V => 1.1,
                                    QuatRole::FalseNineCreator22V => 2.2,
                                    QuatRole::TotalChaosEmergency33V => 3.3,
                                };

                                let v_color = match p.role {
                                    QuatRole::InvertedFullback0V => egui::Color32::GRAY,
                                    QuatRole::HalfSpaceOperator11V => egui::Color32::GREEN,
                                    QuatRole::FalseNineCreator22V => egui::Color32::GOLD,
                                    QuatRole::TotalChaosEmergency33V => egui::Color32::LIGHT_RED,
                                };

                                ui.label(egui::RichText::new(format!("{:.1}V", p.voltage)).color(v_color));
                                ui.end_row();
                            }
                        });
                    });
                });

                ui.add_space(6.0);

                // LOGOVI TAKTIČKOG PROCESORA
                ui.group(|ui| {
                    ui.label(egui::RichText::new("📢 Tactical Engine Console").strong());
                    ui.separator();

                    egui::ScrollArea::vertical().max_height(90.0).show(ui, |ui| {
                        for log in self.tactical_logs.iter().rev() {
                            ui.monospace(egui::RichText::new(log).font(egui::FontId::monospace(10.0)).color(egui::Color32::LIGHT_YELLOW));
                        }
                    });
                });
            });

            // DESNI PANEL: 2D VISUALISERS TERENA
            cols[1].vertical(|ui| {
                ui.group(|ui| {
                    ui.label(egui::RichText::new("🏟️ 4^8 Pitch Visualizer (Naponi na Terenu)").strong());
                    ui.separator();

                    // CRTANJE TERENA U EGUI CANVAS-U
                    let (response, painter) = ui.allocate_painter(
                        egui::vec2(ui.available_width(), 340.0),
                        egui::Sense::hover(),
                    );
                    let rect = response.rect;

                    // Pozadina terena (Trava)
                    painter.rect_filled(rect, 4.0, egui::Color32::from_rgb(20, 80, 35));
                    painter.rect_stroke(
                        rect, 
                        4.0, 
                        egui::Stroke::new(2.0, egui::Color32::WHITE),
                        egui::StrokeKind::Middle
                    );

                    // Srednja linija i centar
                    let mid_y = rect.min.y + rect.height() * 0.5;
                    painter.line_segment(
                        [egui::pos2(rect.min.x, mid_y), egui::pos2(rect.max.x, mid_y)],
                        egui::Stroke::new(1.5, egui::Color32::WHITE),
                    );
                    painter.circle_stroke(
                        egui::pos2(rect.min.x + rect.width() * 0.5, mid_y),
                        35.0,
                        egui::Stroke::new(1.5, egui::Color32::WHITE),
                    );

                    // Iscrtavanje igrača u zavisnosti od X/Y koordinata i Napona
                    for p in &self.pitch_players {
                        let px = rect.min.x + p.x_pos * rect.width();
                        let py = rect.max.y - p.y_pos * rect.height(); // Y obrnuto za fudbalski teren

                        let p_color = match p.role {
                            QuatRole::InvertedFullback0V => egui::Color32::LIGHT_GRAY,
                            QuatRole::HalfSpaceOperator11V => egui::Color32::LIGHT_GREEN,
                            QuatRole::FalseNineCreator22V => egui::Color32::GOLD,
                            QuatRole::TotalChaosEmergency33V => egui::Color32::LIGHT_RED,
                        };

                        // Kvatni krug oko igrača
                        painter.circle_filled(egui::pos2(px, py), 12.0, p_color);
                        painter.circle_stroke(egui::pos2(px, py), 12.0, egui::Stroke::new(1.5, egui::Color32::BLACK));

                        // Broj i ime
                        painter.text(
                            egui::pos2(px, py - 2.0),
                            egui::Align2::CENTER_CENTER,
                            format!("{}", p.number),
                            egui::FontId::proportional(10.0),
                            egui::Color32::BLACK,
                        );

                        painter.text(
                            egui::pos2(px, py + 18.0),
                            egui::Align2::CENTER_CENTER,
                            &p.name,
                            egui::FontId::proportional(9.0),
                            egui::Color32::WHITE,
                        );
                    }
                });
            });
        });
    }
}