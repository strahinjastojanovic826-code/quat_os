use eframe::egui;
use crate::kernel::{Quat, QuatKernel4x8};

/// Tabovi unutar Strateškog Modula
#[derive(PartialEq, Clone, Copy)]
pub enum StrategyTab {
    MacroEconomy,
    IndustryAndFactories,
    MilitaryLogistics,
    DiplomacyAndInfamy,
    FrontlineSimulation,
}

/// QuatOS Grand Strategy & Macro-Logistics Engine
pub struct GrandStrategyEngine {
    pub current_tab: StrategyTab,
    pub coal_stock: u16,        // Q0 Resurs
    pub steel_stock: u16,       // Q1 Resurs
    pub fuel_stock: u16,        // Q2 Resurs
    pub ammo_stock: u16,        // Q3 Resurs
    pub mobilization_level: u8, // 0..=3 (Q0 - Q3)
    pub inflation_rate_base4: f32,
    pub infamy_points: u8,      // Badboy / Infamy metar (0..255)
    pub active_divisions: u16,
    pub supply_efficiency: f32,
    pub log_history: Vec<String>,
}

impl Default for GrandStrategyEngine {
    fn default() -> Self {
        Self {
            current_tab: StrategyTab::MacroEconomy,
            coal_stock: 16384, // 4^7
            steel_stock: 4096,  // 4^6
            fuel_stock: 1024,   // 4^5
            ammo_stock: 256,    // 4^4
            mobilization_level: 0,
            inflation_rate_base4: 2.1,
            infamy_points: 5,
            active_divisions: 12,
            supply_efficiency: 94.5,
            log_history: vec![
                "[STRATEGY] Macro-Strategy & Logistics Core Initialized.".to_string(),
                "[ECONOMY] National Treasury mapped to 4^8 Memory Space.".to_string(),
            ],
        }
    }
}

impl GrandStrategyEngine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Glavni UI Render za Strateški Engine
    pub fn ui(&mut self, ui: &mut egui::Ui, kernel: &mut QuatKernel4x8) {
        let q = kernel.reg_a.to_quats();

        // 1. ZAGLAVLJE MODULA
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.heading("🌐 QuatOS Grand Strategy & Macro-Logistics Engine ($4^8$)");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let mob_text = match self.mobilization_level {
                        0 => "Q0: PEACETIME",
                        1 => "Q1: INDUSTRIALIZING",
                        2 => "Q2: PARTIAL MOBILIZATION",
                        _ => "Q3: TOTAL WAR ECONOMY (3.3V)",
                    };
                    ui.label(
                        egui::RichText::new(mob_text)
                            .strong()
                            .color(egui::Color32::BLACK)
                            .background_color(if self.mobilization_level == 3 {
                                egui::Color32::LIGHT_RED
                            } else {
                                egui::Color32::GREEN
                            }),
                    );
                });
            });

            ui.label(
                egui::RichText::new(format!(
                    "Aktivne Divizije: {} | Efikasnost Mreže Snabdevanja: {:.1}% | Infamy: {}/100",
                    self.active_divisions, self.supply_efficiency, self.infamy_points
                ))
                .font(egui::FontId::proportional(13.0))
                .color(egui::Color32::LIGHT_BLUE),
            );
        });

        ui.add_space(6.0);

        // 2. TAB NAVIGACIJA
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.current_tab, StrategyTab::MacroEconomy, "📊 Makroekonomija");
            ui.selectable_value(&mut self.current_tab, StrategyTab::IndustryAndFactories, "🏭 Industrijski Kompleks");
            ui.selectable_value(&mut self.current_tab, StrategyTab::MilitaryLogistics, "🚚 Mreža Snabdevanja");
            ui.selectable_value(&mut self.current_tab, StrategyTab::DiplomacyAndInfamy, "🕊️ Diplomatija & Tenzija");
            ui.selectable_value(&mut self.current_tab, StrategyTab::FrontlineSimulation, "⚔️ Frontalna Simulacija");
        });

        ui.separator();
        ui.add_space(4.0);

        // 3. RENDEROVANJE TABA
        match self.current_tab {
            StrategyTab::MacroEconomy => self.render_economy(ui, kernel, &q),
            StrategyTab::IndustryAndFactories => self.render_industry(ui, kernel, &q),
            StrategyTab::MilitaryLogistics => self.render_logistics(ui, kernel, &q),
            StrategyTab::DiplomacyAndInfamy => self.render_diplomacy(ui, kernel, &q),
            StrategyTab::FrontlineSimulation => self.render_frontline(ui, kernel, &q),
        }
    }

    // --- TAB 1: MAKROEKONOMIJA & TRŽIŠTE ---
    fn render_economy(&mut self, ui: &mut egui::Ui, _kernel: &QuatKernel4x8, q: &[u8; 8]) {
        ui.heading("📊 Baza-4 Državne Rezerve i Tržišni Indeksi");
        ui.separator();

        ui.columns(2, |cols| {
            cols[0].group(|ui| {
                ui.label(egui::RichText::new("📦 STRATEŠKE REZERVE RESURSA").strong());
                ui.separator();

                ui.label(format!("• Q0 Ugalj i Žito: {} QWords", self.coal_stock));
                ui.add(egui::ProgressBar::new(self.coal_stock as f32 / 65535.0).text("Sirovine"));

                ui.add_space(4.0);
                ui.label(format!("• Q1 Čelik i Metal: {} QWords", self.steel_stock));
                ui.add(egui::ProgressBar::new(self.steel_stock as f32 / 65535.0).text("Prerađevine"));

                ui.add_space(4.0);
                ui.label(format!("• Q2 Sintetičko Gorivo: {} QWords", self.fuel_stock));
                ui.add(egui::ProgressBar::new(self.fuel_stock as f32 / 65535.0).text("Energija"));

                ui.add_space(4.0);
                ui.label(format!("• Q3 Artiljerijska Municija: {} QWords", self.ammo_stock));
                ui.add(egui::ProgressBar::new(self.ammo_stock as f32 / 65535.0).text("Ratne Rezerve"));
            });

            cols[1].group(|ui| {
                ui.label(egui::RichText::new("📈 PARAMETRI TRŽIŠTA I INFLACIJE").strong());
                ui.separator();
                ui.label(format!("Godišnja Inflacija: {:.2}%", self.inflation_rate_base4));
                ui.label(format!("Kvatni Napon Tržišta: {:.1}V", (q[0] as f32 + 1.0) * 0.8));

                ui.add_space(8.0);
                ui.label("Upravljanje Budžetom:");
                if ui.button("💵 Povećaj Poreze na Q0 (Agrar)").clicked() {
                    self.coal_stock = self.coal_stock.saturating_sub(500);
                    self.inflation_rate_base4 += 0.4;
                    self.log_history.push("[ECONOMY] Taxes increased on Q0 Sector.".to_string());
                }
                if ui.button("🏭 Kupi Čelik na Svetskom Tržištu").clicked() {
                    self.steel_stock = self.steel_stock.saturating_add(1000);
                    self.log_history.push("[MARKET] Purchased 1000 Steel QWords.".to_string());
                }
            });
        });
    }

    // --- TAB 2: INDUSTRIJA I FABRIKE ---
    fn render_industry(&mut self, ui: &mut egui::Ui, kernel: &mut QuatKernel4x8, _q: &[u8; 8]) {
        ui.heading("🏭 Baza-4 Industrijski Kompleks i Fabrike");
        ui.separator();

        ui.group(|ui| {
            ui.label(egui::RichText::new("⚙️ NIVOI MOBILIZACIJE INDUSTRIJE").strong());
            ui.horizontal(|ui| {
                if ui.selectable_label(self.mobilization_level == 0, "Q0: Mirovna Privreda (0.0V)").clicked() {
                    self.mobilization_level = 0;
                }
                if ui.selectable_label(self.mobilization_level == 1, "Q1: Industrializacija (1.1V)").clicked() {
                    self.mobilization_level = 1;
                }
                if ui.selectable_label(self.mobilization_level == 2, "Q2: Delimična Mobilizacija (2.2V)").clicked() {
                    self.mobilization_level = 2;
                }
                if ui.selectable_label(self.mobilization_level == 3, "Q3: Totalni Rat (3.3V MAX)").clicked() {
                    self.mobilization_level = 3;
                    kernel.drivers.set_audio_state(Quat::Q3);
                }
            });
        });

        ui.add_space(8.0);

        ui.columns(2, |cols| {
            cols[0].group(|ui| {
                ui.label(egui::RichText::new("🏢 CIVILNE FABRIKE").strong());
                ui.separator();
                ui.label("• Proizvodnja potrošačkih dobara");
                ui.label("• Izgradnja infrastrukture i pruga");
                if ui.button("🔨 Izgradi Novu Prugu za Snabdevanje").clicked() {
                    self.supply_efficiency = (self.supply_efficiency + 2.5).min(100.0);
                    self.log_history.push("[INFRA] Railway infrastructure expanded.".to_string());
                }
            });

            cols[1].group(|ui| {
                ui.label(egui::RichText::new("⚔️ VOJNE FABRIKE").strong());
                ui.separator();
                ui.label("• Prerada Čelika u Municiju i Tenkove");
                ui.label("• Povećava borbenu gotovost");
                if ui.button("💣 Preusmeri Čelik u Proizvodnju Municije").clicked() {
                    if self.steel_stock >= 500 {
                        self.steel_stock -= 500;
                        self.ammo_stock = self.ammo_stock.saturating_add(1500);
                        self.log_history.push("[INDUSTRY] Steel converted into Munitions.".to_string());
                    }
                }
            });
        });
    }

    // --- TAB 3: LOGISTIKA I MREŽA SNABDEVANJA ---
    fn render_logistics(&mut self, ui: &mut egui::Ui, _kernel: &QuatKernel4x8, q: &[u8; 8]) {
        ui.heading("🚚 Mreža Snabdevanja i Analiza Uskih Grla (Logistics Core)");
        ui.separator();

        ui.group(|ui| {
            ui.label(egui::RichText::new("📊 PROTOCI KROZ MEMORIJSKE STRANICE ($4^8$ Supply Hubs)").strong());
            ui.separator();

            egui::Grid::new("logistics_grid")
                .striped(true)
                .min_col_width(110.0)
                .show(ui, |ui| {
                    ui.strong("Hub Adresa");
                    ui.strong("Regija");
                    ui.strong("Kapacitet Pruge");
                    ui.strong("Zagušenje");
                    ui.strong("Napon Hub-a");
                    ui.end_row();

                    let hubs = [
                        ("0x0010 / Q0010", "Glavni Grad (Central Hub)", "100%", "0% (Optimalno)", "3.3V (Q3)"),
                        ("0x0100 / Q0100", "Industrijska Zona Zapad", "85%", "12% (Nisko)", "2.2V (Q2)"),
                        ("0x0250 / Q0220", "Istočni Koridor", "60%", "38% (Umereno)", "1.1V (Q1)"),
                        ("0x03FE / Q0332", "Frontalna Linija 1", "30%", "72% (CRITICAL)", "0.0V (Q0)"),
                    ];

                    for item in hubs.iter() {
                        ui.monospace(item.0);
                        ui.label(item.1);
                        ui.label(item.2);
                        ui.label(egui::RichText::new(item.3).color(if item.3.contains("CRITICAL") {
                            egui::Color32::LIGHT_RED
                        } else {
                            egui::Color32::LIGHT_GREEN
                        }));
                        ui.label(item.4);
                        ui.end_row();
                    }
                });
        });

        ui.add_space(8.0);
        ui.label(format!("Aktivno opterećenje sabirnice snabdevanja: {}%", q[2] * 25));
    }

    // --- TAB 4: DIPLOMATIJA I TENZIJA ---
    fn render_diplomacy(&mut self, ui: &mut egui::Ui, _kernel: &QuatKernel4x8, _q: &[u8; 8]) {
        ui.heading("🕊️ Baza-4 Diplomatija, Tenzija i Badboy / Infamy Metar");
        ui.separator();

        ui.columns(2, |cols| {
            cols[0].group(|ui| {
                ui.label(egui::RichText::new("⚠️ MEĐUNARODNA ZLA VOLJA (INFAMY)").strong());
                ui.separator();
                ui.label(format!("Nivo Zle Volje: {} / 100", self.infamy_points));
                ui.add(egui::ProgressBar::new(self.infamy_points as f32 / 100.0).text("Infamy Meter"));

                ui.add_space(6.0);
                if self.infamy_points > 25 {
                    ui.label(egui::RichText::new("🚨 UPOZORENJE: Velike sile mogu objaviti Rat za Otuđenje!").color(egui::Color32::LIGHT_RED));
                } else {
                    ui.label("Diplomatska pozicija je stabilna.");
                }
            });

            cols[1].group(|ui| {
                ui.label(egui::RichText::new("📜 DIPLOMATSKE AKCIJE").strong());
                ui.separator();

                if ui.button("🕊️ Potpiši Trgovinski Sporazum (Smanjuje Infamy)").clicked() {
                    self.infamy_points = self.infamy_points.saturating_sub(3);
                    self.log_history.push("[DIPLOMACY] Commercial treaty signed.".to_string());
                }

                if ui.button("⚔️ Polaži Pravo na Adresni Prostor (Opravdanje Rata)").clicked() {
                    self.infamy_points = self.infamy_points.saturating_add(10);
                    self.log_history.push("[WAR GOAL] Justifying war claim on memory sector 0x0300.".to_string());
                }
            });
        });
    }

    // --- TAB 5: FRONTALNA SIMULACIJA I BITKE ---
    fn render_frontline(&mut self, ui: &mut egui::Ui, kernel: &mut QuatKernel4x8, _q: &[u8; 8]) {
        ui.heading("⚔️ Frontalna Simulacija i Okršaji u Realnom Vremenu");
        ui.separator();

        ui.group(|ui| {
            ui.label(egui::RichText::new("🎯 STATISTIKA FRONTA").strong());
            ui.horizontal(|ui| {
                ui.label(format!("Broj Divizija na Frontu: {}", self.active_divisions));
                if ui.button("➕ Regrutuj Novu Diviziju").clicked() {
                    if self.steel_stock >= 200 && self.ammo_stock >= 100 {
                        self.steel_stock -= 200;
                        self.ammo_stock -= 100;
                        self.active_divisions += 1;
                        self.log_history.push("[MILITARY] New Division mobilized into action.".to_string());
                    }
                }
            });
        });

        ui.add_space(8.0);

        ui.group(|ui| {
            ui.label(egui::RichText::new("⚡ SIMULIRAJ TAKTNI IMPULS BITKE (1 TICK)").strong());
            if ui.button("💥 Naredi Opšti Napad (Consumes Ammo & Fuel)").clicked() {
                if self.ammo_stock >= 50 && self.fuel_stock >= 20 {
                    self.ammo_stock -= 50;
                    self.fuel_stock -= 20;
                    kernel.step(); // Pokreće takt procesora
                    self.log_history.push(format!(
                        "[BATTLE] Offensive pushed! CPU PC: 0x{:04X} | Enemy positions shelled.",
                        kernel.pc
                    ));
                } else {
                    self.log_history.push("[LOGISTICS ERROR] Out of Ammunition or Fuel for offensive!".to_string());
                }
            }
        });

        ui.add_space(8.0);

        // DNEVNIK STRATEŠKIH DOGAĐAJA
        ui.group(|ui| {
            ui.label(egui::RichText::new("📜 DNEVNIK STRATEŠKIH OPERACIJA").strong());
            ui.separator();
            egui::ScrollArea::vertical().max_height(120.0).show(ui, |ui| {
                for log in self.log_history.iter().rev() {
                    ui.monospace(log);
                }
            });
        });
    }
}