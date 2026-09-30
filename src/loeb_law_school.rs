use eframe::egui;
use crate::kernel::{Quat, QuatKernel4x8};

/// Tabovi unutar Loeb Law School Modula
#[derive(PartialEq, Clone, Copy)]
pub enum LawTab {
    TrustAndContracts,
    OnionCrunchEquity,
    JurisdictionMapper,
    CourtroomSynth,
    CaseLawDatabase,
}

/// Nick Loeb's School of Quaternary Jurisprudence Engine
pub struct LoebLawSchool {
    pub current_tab: LawTab,
    pub motion_search: String,
    pub selected_jurisdiction: u8, // 0: CA, 1: LA, 2: FL, 3: Federal Q3
    pub objection_volts: f32,
    pub trust_fund_qwords: usize,
    pub crispy_onion_shares: f32,
    pub motion_logs: Vec<String>,
    pub Active_lawsuit_title: String,
}

impl Default for LoebLawSchool {
    fn default() -> Self {
        Self {
            current_tab: LawTab::TrustAndContracts,
            motion_search: String::new(),
            selected_jurisdiction: 1, // Lujzijana je omiljena (Q1)
            objection_volts: 2.2,
            trust_fund_qwords: 16384, // 4^7 QWords
            crispy_onion_shares: 51.0,
            motion_logs: vec![
                "[SYSTEM] Loeb Law Engine Initialized v4.8".to_string(),
                "[LEGAL] Filed Motion for Emergency Custody of Memory Page 0x00FF".to_string(),
            ],
            Active_lawsuit_title: "Loeb v. Modern Family Trust & QuatEngine Kernel".to_string(),
        }
    }
}

impl LoebLawSchool {
    pub fn new() -> Self {
        Self::default()
    }

    /// Glavni UI Render za Pravni Modul Nick Loeba
    pub fn ui(&mut self, ui: &mut egui::Ui, kernel: &mut QuatKernel4x8) {
        let q = kernel.reg_a.to_quats();
        let pc = kernel.pc;

        // 1. ZAGLAVLJE FAKULTETA & SLUČAJA
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.heading("⚖️ Nick Loeb School of Quaternary Jurisprudence ($4^8$)");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new("📜 JURISDICTION: LOUISIANA STATE COURT (Q1)")
                            .strong()
                            .color(egui::Color32::BLACK)
                            .background_color(egui::Color32::GOLD),
                    );
                });
            });

            ui.label(
                egui::RichText::new(format!("Aktivni Proces: \"{}\" | Status: Q{} (Litigation Active)", self.Active_lawsuit_title, q[0]))
                    .italics()
                    .font(egui::FontId::proportional(13.0))
                    .color(egui::Color32::LIGHT_BLUE),
            );
        });

        ui.add_space(6.0);

        // 2. TAB NAVIGACIJA
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.current_tab, LawTab::TrustAndContracts, "📜 Trust & Contract Law");
            ui.selectable_value(&mut self.current_tab, LawTab::OnionCrunchEquity, "🧅 Loeb's Crunch Corporate Law");
            ui.selectable_value(&mut self.current_tab, LawTab::JurisdictionMapper, "🗺️ Jurisdiction Mapper");
            ui.selectable_value(&mut self.current_tab, LawTab::CourtroomSynth, "👨‍⚖️ Objection & Pitch Synth");
            ui.selectable_value(&mut self.current_tab, LawTab::CaseLawDatabase, "📁 $4^8$ Case Law Database");
        });

        ui.separator();
        ui.add_space(4.0);

        // 3. RENDEROVANJE SELEKTOVANOG TABA
        match self.current_tab {
            LawTab::TrustAndContracts => self.render_trust_law(ui, kernel, &q, pc),
            LawTab::OnionCrunchEquity => self.render_onion_equity(ui, kernel, &q),
            LawTab::JurisdictionMapper => self.render_jurisdiction_mapper(ui, kernel, &q),
            LawTab::CourtroomSynth => self.render_courtroom_synth(ui, kernel, &q),
            LawTab::CaseLawDatabase => self.render_case_database(ui, kernel, &q),
        }
    }

    // --- TAB 1: TRUST & CONTRACT LAW ($4^8$ PAGING) ---
    fn render_trust_law(&mut self, ui: &mut egui::Ui, _kernel: &QuatKernel4x8, q: &[u8; 8], pc: usize) {
        ui.heading("📜 Irrevocable Embryonic Trust & Quaternary Contract Clauses");
        ui.separator();

        ui.columns(2, |cols| {
            cols[0].group(|ui| {
                ui.label(egui::RichText::new("🏛️ TRUST FUND ALOKACIJA (\"Emma & Isabella Trust\")").strong());
                ui.separator();
                ui.label(format!("Alocirana RAM Memorija: {} / 65,536 QWords", self.trust_fund_qwords));
                ui.add(egui::ProgressBar::new(self.trust_fund_qwords as f32 / 65536.0).text("25% Adresnog Prostora"));

                ui.add_space(8.0);
                ui.label("Pravni Status Alokacije:");
                ui.label("• Pravo na rođenje (Right to Life Clause): ENFORCED (Q3)");
                ui.label("• Starateljstvo nad Registrom A: NICK LOEB (PLAINTIFF)");
                ui.label(format!("• Suglasnost Tuženog (Sofía V.): OVERRIDDEN BY COURT (Q{})", q[1]));
            });

            cols[1].group(|ui| {
                ui.label(egui::RichText::new("⚖️ BAZA-4 KLAUZULE O STARATELJSTVU").strong());
                ui.separator();
                ui.label("Q0 (0.0V): Mutual Consent Required (Standardni Ugovor)");
                ui.label("Q1 (1.1V): Louisiana Unborn Child Act (Povoljna Jurisdikcija)");
                ui.label("Q2 (2.2V): Injunction Against Destruction of Memory Pages");
                ui.label("Q3 (3.3V): Supreme Court Appeal & Trust Enforcement");

                ui.add_space(8.0);
                ui.label(format!("Trenutna Klauzula po PC (0x{:04X}): State Q{}", pc, pc % 4));
            });
        });

        ui.add_space(8.0);

        ui.group(|ui| {
            ui.label(egui::RichText::new("📝 GENERIŠI NOVI PODNESAK (Emergency Motion):").strong());
            ui.horizontal(|ui| {
                if ui.button("🚨 Podnesi Zahtev za Zabrana Brisanja Registara").clicked() {
                    self.motion_logs.push(format!("[MOTION] Motion for Preliminary Injunction filed at PC: 0x{:04X}", pc));
                }
                if ui.button("🏛️ Prebaci Slučaj u Lujzijanu (Jurisdiction Shift)").clicked() {
                    self.selected_jurisdiction = 1;
                    self.motion_logs.push("[JURISDICTION] Case transferred to Fourth Circuit Court of Appeal (Louisiana)".to_string());
                }
            });
        });
    }

    // --- TAB 2: LOEB'S CRUNCH CORPORATE LAW ---
    fn render_onion_equity(&mut self, ui: &mut egui::Ui, _kernel: &QuatKernel4x8, q: &[u8; 8]) {
        ui.heading("🧅 Loeb's Onion Crunch Inc. — Corporate IP & Equity Litigation");
        ui.separator();

        ui.columns(2, |cols| {
            cols[0].group(|ui| {
                ui.label(egui::RichText::new("📦 HRNSKAVI LUK (ONION CRUNCH) PATENTI").strong());
                ui.separator();
                ui.label("Naziv Firme: Loeb's Crunch LLC");
                ui.label("Patent Recepture: Crispiness Index #48-QUAT");
                ui.label(format!("Udeo Nick Loeba: {:.1}% (Majority Stake)", self.crispy_onion_shares));
                
                ui.add_space(6.0);
                ui.label("Distribucija po Kanalima I/O:");
                ui.label(format!("• Retail Channel Q0: {} %", q[4] * 25));
                ui.label(format!("• Direct-to-Consumer Q1: {} %", q[5] * 25));
                ui.label(format!("• Patent Royalty Q2: {} %", q[6] * 25));
            });

            cols[1].group(|ui| {
                ui.label(egui::RichText::new("⚖️ PARNICA ZA DIONICE I BREND").strong());
                ui.separator();
                ui.label("Status Tužbe protiv Bivših Partnera:");
                ui.label("• Neovlašćeno korišćenje hrskavog luka: CLAIM FILED");
                ui.label("• Zaštita žiga u bazi 4: APPROVED (Q3)");
                ui.label(format!("• Procenjena Šteta: ${} M", (q[7] as u32 + 1) * 12));

                ui.add_space(6.0);
                if ui.button("💰 Zatraži Isplatu Dividendi iz Registra B").clicked() {
                    self.motion_logs.push("[EQUITY] Dividend Payout Motion Submitted to Delaware Chancery Court".to_string());
                }
            });
        });
    }

    // --- TAB 3: JURISDICTION MAPPER ---
    fn render_jurisdiction_mapper(&mut self, ui: &mut egui::Ui, _kernel: &QuatKernel4x8, q: &[u8; 8]) {
        ui.heading("🗺️ Base-4 Forum Non Conveniens & Jurisdiction Mapper");
        ui.separator();

        ui.group(|ui| {
            ui.label(egui::RichText::new("Izaberi Sudsku Jurisdikciju za Podnošenje Tužbe:").strong());
            ui.horizontal(|ui| {
                if ui.selectable_label(self.selected_jurisdiction == 0, "Q0: California State Court (Nepovoljno)").clicked() {
                    self.selected_jurisdiction = 0;
                }
                if ui.selectable_label(self.selected_jurisdiction == 1, "Q1: Louisiana State Court (Veoma Povoljno)").clicked() {
                    self.selected_jurisdiction = 1;
                }
                if ui.selectable_label(self.selected_jurisdiction == 2, "Q2: Florida Circuit Court (Neutralno)").clicked() {
                    self.selected_jurisdiction = 2;
                }
                if ui.selectable_label(self.selected_jurisdiction == 3, "Q3: US Federal Supreme Court (Maksimalni Napon)").clicked() {
                    self.selected_jurisdiction = 3;
                }
            });
        });

        ui.add_space(8.0);

        ui.group(|ui| {
            ui.label(egui::RichText::new("📊 Verovatnoća Dobijanja Privremene Mere (Base-4 Analysis):").strong());
            let chance = match self.selected_jurisdiction {
                0 => 15.0 + (q[0] as f32 * 2.0),
                1 => 85.0 + (q[1] as f32 * 4.0),
                2 => 50.0 + (q[2] as f32 * 3.0),
                _ => 99.0,
            };

            ui.add(egui::ProgressBar::new(chance / 100.0).text(format!("{:.1}% Šanse za Pobedu", chance)));
            ui.label(format!("Naponski Nivo Sudnice: {:.1}V", self.selected_jurisdiction as f32 * 1.1));
        });
    }

    // --- TAB 4: COURTROOM SYNTH & OBJECTIONS ---
    fn render_courtroom_synth(&mut self, ui: &mut egui::Ui, kernel: &mut QuatKernel4x8, _q: &[u8; 8]) {
        ui.heading("👨‍⚖️ Real-Time Courtroom Telemetry & Objection Synth");
        ui.separator();

        ui.group(|ui| {
            ui.label(egui::RichText::new("🎛️ PRIGOVOR (OBJECTION) PITCH GENERATOR").strong());
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Napon Prigovora (Objection Volts):");
                ui.add(egui::Slider::new(&mut self.objection_volts, 0.0..=3.3).text("Volti (V)"));
            });

            let quat_val = match self.objection_volts {
                v if v < 0.8 => Quat::Q0,
                v if v < 1.8 => Quat::Q1,
                v if v < 2.8 => Quat::Q2,
                _ => Quat::Q3,
            };

            ui.label(format!("Kvatni Nivo Prigovora: {:?} ({:.1}V)", quat_val, self.objection_volts));

            ui.horizontal(|ui| {
                if ui.button("💥 ¡OBJECTION! (Prigovor na Hearsay)").clicked() {
                    kernel.drivers.set_audio_state(quat_val);
                    self.motion_logs.push(format!("[OBJECTION] Sustained! Pitch level: {:.1}V", self.objection_volts));
                }
                if ui.button("🚨 ¡MOTION TO STRIKE!").clicked() {
                    kernel.drivers.set_audio_state(Quat::Q3);
                    self.motion_logs.push("[MOTION] Motion to strike testimony from kernel memory log!".to_string());
                }
            });
        });
        
        //Veoma poucna edukacija za studente prava

        ui.add_space(8.0);

        ui.group(|ui| {
            ui.label(egui::RichText::new("📜 DNEVNIK SUDSKIH PODNESAKA (REAL-TIME LOGS)").strong());
            ui.separator();
            egui::ScrollArea::vertical().max_height(120.0).show(ui, |ui| {
                for log in self.motion_logs.iter().rev() {
                    ui.monospace(log);
                }
            });
        });
    }

    // --- TAB 5: CASE LAW DATABASE ---
    fn render_case_database(&mut self, ui: &mut egui::Ui, _kernel: &QuatKernel4x8, _q: &[u8; 8]) {
        ui.heading("📁 Loeb $4^8$ Case Law Database ($65,536$ Precedents)");
        ui.separator();

        ui.horizontal(|ui| {
            ui.label("🔍 Pretraga Presuda:");
            ui.text_edit_singleline(&mut self.motion_search);
            if ui.button("Bistri").clicked() {
                self.motion_search.clear();
            }
        });

        ui.add_space(6.0);

        egui::Grid::new("loeb_case_grid")
            .striped(true)
            .min_col_width(100.0)
            .show(ui, |ui| {
                ui.strong("Broj Predmeta");
                ui.strong("Naziv Slučaja");
                ui.strong("Sud / Jurisdikcija");
                ui.strong("Kvatni Ischod");
                ui.strong("Status Žalbe");
                ui.end_row();

                let cases = [
                    ("0x0001 / Q0001", "Loeb v. Vergara I", "California Superior Court", "Q0 (DISMISSED)", "Appealed to LA"),
                    ("0x00FE / Q0332", "Loeb v. Vergara II (Embryo Custody)", "Louisiana State Court", "Q1 (PENDING)", "Active Motion"),
                    ("0x01A0 / Q1220", "Loeb's Crunch v. Snack Corp", "Delaware Chancery Court", "Q2 (SETTLED)", "Royalty 15%"),
                    ("0x02FF / Q2333", "In re Emma & Isabella Trust", "Jefferson Parish Court", "Q3 (INJUNCTION)", "Enforced"),
                    ("0x03FF / Q3333", "Loeb v. Modern Family Production", "Federal Circuit Court", "Q3 (OVERRULED)", "Pending Certiorari"),
                ];

                for item in cases.iter() {
                    if !self.motion_search.is_empty()
                        && !item.1.to_lowercase().contains(&self.motion_search.to_lowercase())
                        && !item.2.to_lowercase().contains(&self.motion_search.to_lowercase())
                    {
                        continue;
                    }

                    ui.monospace(item.0);
                    ui.label(egui::RichText::new(item.1).strong().color(egui::Color32::LIGHT_RED));
                    ui.label(item.2);
                    ui.label(item.3);
                    ui.label(egui::RichText::new(item.4).small().italics());
                    ui.end_row();
                }
         });
    }
     pub fn audit_40k_indie_movie_budget(reg_a: u16) -> String {
    if reg_a == 40000 {
        return "🚨 ALARM: Pronađen budžet filma od $40,000! Nick Loeb podnosi hitnu tužbu za povraćaj sredstava u Lujzijani!".to_string();
    }
    "Budžet je stabilan.".to_string()
}

}