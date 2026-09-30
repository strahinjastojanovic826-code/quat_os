use eframe::egui;
use crate::kernel::{Quat, QuatKernel4x8};

/// Stanja malverzacije u Bazi 4 ($Q_0 \dots Q_3$)
#[derive(PartialEq, Clone, Copy)]
pub enum MalversationsLevel {
    Q0CleanState,     // 0.0V - Pantićeva plata (Sirotinja)
    Q1Reprezentacija, // 1.1V - Kafa i viski sa šlagom
    Q2MuckaOOUR,      // 2.2V - Krediti i mućke u Preduzeću
    Q3BeloglaviSup,   // 3.3V - KAPETAN LAĐE / Maksimalna malverzacija
}

/// Šojić & Preduzeće Birokratsko-Knjigovodstveni Engine
pub struct SojicBureaucracyEngine {
    pub state: MalversationsLevel,
    pub pantic_patience: f32,       // Strpljenje Pantića (0.0..100.0)
    pub reprezentacija_budget: u16, // Budžet za viski i kafu (QWords)
    pub loeb_audit_fund: u32,       // Loeb-ov fond ($40.000)
    pub inspection_alert: bool,     // Carry Flag okidač inspekcije
    pub log_history: Vec<String>,
}

impl Default for SojicBureaucracyEngine {
    fn default() -> Self {
        Self {
            state: MalversationsLevel::Q0CleanState,
            pantic_patience: 100.0,
            reprezentacija_budget: 1024,
            loeb_audit_fund: 40_000,
            inspection_alert: false,
            log_history: vec![
                "[PREDUZEĆE] Birokratsko-knjigovodstveni modul inicijalizovan.".to_string(),
                "[ŠOJIĆ] 'Polako sa to plaćanje, nismo mi ovde došli da trošimo pare!'".to_string(),
            ],
        }
    }
}

impl SojicBureaucracyEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, kernel: &mut QuatKernel4x8) {
        let q = kernel.reg_a.to_quats();

        // 1. ZAGLAVLJE MODULA
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.heading("💼 Šojić & Preduzeće: Birokratski Engine ($4^8$)");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let (status_text, color) = match self.state {
                        MalversationsLevel::Q0CleanState => ("Q0: PANTIĆEVA PLATA (0.0V)", egui::Color32::GRAY),
                        MalversationsLevel::Q1Reprezentacija => ("Q1: VISKI SA ŠLAGOM (1.1V)", egui::Color32::LIGHT_BLUE),
                        MalversationsLevel::Q2MuckaOOUR => ("Q2: MUĆKA U PREDUZEĆU (2.2V)", egui::Color32::GOLD),
                        MalversationsLevel::Q3BeloglaviSup => ("Q3: ¡BELOGLAVI SUP / KAPETAN LAĐE! (3.3V)", egui::Color32::LIGHT_RED),
                    };
                    ui.label(
                        egui::RichText::new(status_text)
                            .strong()
                            .color(egui::Color32::BLACK)
                            .background_color(color)
                    );
                });
            });

            ui.label(
                egui::RichText::new(format!(
                    "Loeb Audit Fond: ${} | Budžet Reprezentacije: {} QWords | Strpljenje Pantića: {:.0}%",
                    self.loeb_audit_fund, self.reprezentacija_budget, self.pantic_patience
                ))
                .font(egui::FontId::proportional(13.0))
                .color(egui::Color32::LIGHT_GREEN),
            );
        });

        ui.separator();
        ui.add_space(4.0);

        // 2. KONTROLNI PANELI
        ui.columns(2, |cols| {
            // LEVI PANEL: NAREDBE DIREKTORA
            cols[0].group(|ui| {
                ui.label(egui::RichText::new("👔 NAREDBE GENERALNOG DIREKTORA").strong());
                ui.separator();

                if ui.button("☕ Naruči Viski sa Šlagom (Troši Q1)").clicked() {
                    self.reprezentacija_budget = self.reprezentacija_budget.saturating_add(256);
                    self.state = MalversationsLevel::Q1Reprezentacija;
                    kernel.reg_a.write_quat(0, Quat::Q1);
                    self.log_history.push("[REPREZENTACIJA] Naručena kafa i viski na račun Preduzeća.".to_string());
                }

                ui.add_space(4.0);

                if ui.button("📝 Prebaci Kredit OOUR-a u Bazu 4 (Dizač Napona)").clicked() {
                    self.state = MalversationsLevel::Q2MuckaOOUR;
                    kernel.reg_a.write_quat(0, Quat::Q2);
                    self.pantic_patience -= 15.0;
                    self.log_history.push("[OOUR] Sredstva preusmerena u lični registar A.".to_string());
                }

                ui.add_space(4.0);

                if ui.button("🦅 Aktiviraj Režim 'BELOGLAVI SUP' (Max Overclock 3.3V)").clicked() {
                    self.state = MalversationsLevel::Q3BeloglaviSup;
                    kernel.reg_a.write_quat(0, Quat::Q3);
                    kernel.drivers.set_audio_state(Quat::Q3);
                    self.pantic_patience -= 35.0;
                    self.log_history.push("[KAPETAN LAĐE] Proglašena maksimalna malverzacija na 3.3V!".to_string());
                }

                ui.add_space(6.0);
                if ui.button("💼 Zahtevaj Nick Loeb $40k Pravni Audit").clicked() {
                    if self.loeb_audit_fund >= 10_000 {
                        self.loeb_audit_fund -= 10_000;
                        self.pantic_patience = (self.pantic_patience + 25.0).min(100.0);
                        self.inspection_alert = false;
                        self.log_history.push("[AUDIT] Loeb advokati zataškali izveštaj inspekcije.".to_string());
                    } else {
                        self.log_history.push("[ERROR] Nedovoljno sredstava za pravni audit!".to_string());
                    }
                }
            });

            // DESNI PANEL: INSPEKCIJA I STATUS PANTIĆA
            cols[1].group(|ui| {
                ui.label(egui::RichText::new("📑 INSPEKCIJA & MEEĐUMOMENTALNO STANJE").strong());
                ui.separator();

                ui.label("Strpljenje Referenta Pantića:");
                ui.add(egui::ProgressBar::new(self.pantic_patience / 100.0).text(format!("{:.0}%", self.pantic_patience)));

                if self.pantic_patience <= 20.0 {
                    self.inspection_alert = true;
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new("🚨 PANTIĆ JE DONEO IZVEŠTAJ INSPEKCIJE! CARRY FLAG AKTIVAN!")
                            .strong()
                            .color(egui::Color32::LIGHT_RED),
                    );
                }

                ui.add_space(8.0);
                ui.label("Kvatni Napon Registra A:");
                ui.monospace(format!("Voltaza: {:.1}V | Stat: Q{:?}", (q[0] as f32) * 1.1, q[0]));

                ui.add_space(8.0);
                if ui.button("📄 Izmiri Pantićevu Platu (Smanji Napon na Q0)").clicked() {
                    self.state = MalversationsLevel::Q0CleanState;
                    kernel.reg_a.write_quat(0, Quat::Q0);
                    self.pantic_patience = 100.0;
                    self.inspection_alert = false;
                    self.log_history.push("[PLATA] Pantić primio lični dohodak. Inspekcija obustavljena.".to_string());
                }
            });
        });

        ui.add_space(8.0);

        // DNEVNIK BIROKRATSKIH DOGAĐAJA
        ui.group(|ui| {
            ui.label(egui::RichText::new("📜 DNEVNIK BIROKRATSKIH PROMENA").strong());
            ui.separator();
            egui::ScrollArea::vertical().max_height(100.0).show(ui, |ui| {
                for log in self.log_history.iter().rev() {
                    ui.monospace(log);
                }
            });
        });
    }
}