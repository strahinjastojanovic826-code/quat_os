use eframe::egui;
use crate::kernel::{Quat, QuatKernel4x8};

/// Tabovi unutar Sofía Vergara Španskog Modula
#[derive(PartialEq, Clone, Copy)]
pub enum TutorTab {
    Vocabulary,
    GrammarBase4,
    AccentAudioSynth,
    SofiaQuiz,
    TotySpecialMode,
}

pub struct SpanishTutor {
    pub current_tab: TutorTab,
    pub input_query: String,
    pub selected_quat_level: u8, // 0..=3 (Q0 - Q3)
    pub toty_easter_egg_unlocked: bool,
    pub quiz_score_base4: usize,
    pub active_quote: String,
    pub pitch_volts: f32,
}

impl Default for SpanishTutor {
    fn default() -> Self {
        Self {
            current_tab: TutorTab::Vocabulary,
            input_query: String::new(),
            selected_quat_level: 0,
            toty_easter_egg_unlocked: false,
            quiz_score_base4: 0,
            active_quote: "¡Hola, papi! Dobrodošao u 4^8 Španski sa Sofijom!".to_string(),
            pitch_volts: 2.2,
        }
    }
}

impl SpanishTutor {
    pub fn new() -> Self {
        Self::default()
    }

    /// Glavni UI Render za Sofía Vergara Tutor Modul
    pub fn ui(&mut self, ui: &mut egui::Ui, kernel: &mut QuatKernel4x8) {
        let q = kernel.reg_a.to_quats();

        // 1. PROVERA PROCESORSKIH REGISTARA ZA "TOTY" EASTER EGG (Q3 Q3 Q3 Q3)
        if q[0] == 3 && q[1] == 3 && q[2] == 3 && q[3] == 3 {
            self.toty_easter_egg_unlocked = true;
        }

        // 2. ZAGLAVLJE MODULA SA SOFÍINIM CITATOM
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.heading("💃 Aprende Español con Sofía Vergara — 4^8 QuatEngine");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if self.toty_easter_egg_unlocked {
                        ui.label(
                            egui::RichText::new("🇨🇴 TOTY MODE ACTIVE!")
                                .strong()
                                .color(egui::Color32::YELLOW)
                                .background_color(egui::Color32::RED),
                        );
                    }
                });
            });

            ui.label(
                egui::RichText::new(format!("\"{}\"", self.active_quote))
                    .italics()
                    .font(egui::FontId::proportional(14.0))
                    .color(egui::Color32::from_rgb(255, 180, 100)),
            );
        });

        ui.add_space(6.0);

        // 3. TAB NAVIGACIJA
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.current_tab, TutorTab::Vocabulary, "📖 4^8 Rečnik");
            ui.selectable_value(&mut self.current_tab, TutorTab::GrammarBase4, "📐 4-Stanja Gramatika");
            ui.selectable_value(&mut self.current_tab, TutorTab::AccentAudioSynth, "🎙️ Sofía Accent Synth");
            ui.selectable_value(&mut self.current_tab, TutorTab::SofiaQuiz, "🏆 Kvatni Kviz");

            if self.toty_easter_egg_unlocked {
                ui.selectable_value(&mut self.current_tab, TutorTab::TotySpecialMode, "🇨🇴 TOTY / Barranquilla");
            }
        });

        ui.separator();
        ui.add_space(4.0);

        // 4. PRETRAGA I TRIGGER ZA "TOTY"
        ui.horizontal(|ui| {
            ui.label("🔍 Pretraži fraze / Kvatni kod:");
            let resp = ui.text_edit_singleline(&mut self.input_query);

            if resp.changed() {
                let clean = self.input_query.trim().to_lowercase();
                if clean == "toty" || clean == "toty!" || clean == "barranquilla" {
                    self.toty_easter_egg_unlocked = true;
                    self.current_tab = TutorTab::TotySpecialMode;
                    self.active_quote = "¡Ay, Dios mío! Otključao si moj nadimak iz detinjstva! Welcome to Barranquilla!".to_string();
                }
            }

            if ui.button("Bistri").clicked() {
                self.input_query.clear();
            }
        });

        ui.add_space(8.0);

        // 5. RENDEROVANJE TABOVA
        match self.current_tab {
            TutorTab::Vocabulary => self.render_vocabulary(ui, kernel, &q),
            TutorTab::GrammarBase4 => self.render_grammar(ui, kernel, &q),
            TutorTab::AccentAudioSynth => self.render_audio_synth(ui, kernel, &q),
            TutorTab::SofiaQuiz => self.render_quiz(ui, kernel),
            TutorTab::TotySpecialMode => self.render_toty_mode(ui, kernel, &q),
        }
    }

    // --- TAB 1: 4^8 REČNIK (VOCABULARY) ---
    fn render_vocabulary(&mut self, ui: &mut egui::Ui, _kernel: &QuatKernel4x8, _q: &[u8; 8]) {
        ui.heading("📖 Sofía's Quaternary Vocabulary ($4^8 = 65,536$ Words)");
        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Nivo Kvatne Težine:");
            for level in 0..4 {
                let label = match level {
                    0 => "Q0 (Principiante)",
                    1 => "Q1 (Intermedio)",
                    2 => "Avanzado (Q2)",
                    _ => "Q3 (¡Gloria Fury!)",
                };
                if ui.selectable_label(self.selected_quat_level == level, label).clicked() {
                    self.selected_quat_level = level;
                }
            }
        });

        ui.add_space(6.0);

        egui::Grid::new("spanish_vocab_grid")
            .striped(true)
            .min_col_width(110.0)
            .show(ui, |ui| {
                ui.strong("Kvatna Adresa");
                ui.strong("Španski izraz");
                ui.strong("Prevod na Srpski");
                ui.strong("Sofíin Komentar");
                ui.strong("Napon (V)");
                ui.end_row();

                let vocab_data = [
                    ("0x0001 / Q0001", "¡Hola, papi!", "Zdravo, lutko!", "Tako pozdravljam Jaya!", "1.1V (Q1)"),
                    ("0x00FE / Q0332", "¡Ay, por Dios!", "O Bože moj!", "Kada Manny uradi nešto čudno.", "2.2V (Q2)"),
                    ("0x01A0 / Q1220", "¡Escúchame!", "Slušaj me!", "Kada Jay ne nosi slušni aparat.", "3.3V (Q3)"),
                    ("0x02FF / Q2333", "Vergüenza", "Sramota", "Kada neko ne zna kolumbijsku kuhinju.", "2.2V (Q2)"),
                    ("0x0300 / Q3000", "¡Sinvergüenza!", "Bezobraznik!", "Upućeno Stella-i (psu).", "3.3V (Q3)"),
                    ("0x03FF / Q3333", "¡JAAAAY!", "DŽEEEEEJ!", "Glasnoća probija zvučni zid!", "3.3V (Q3 MAX)"),
                ];

                for item in vocab_data.iter() {
                    if !self.input_query.is_empty()
                        && !item.1.to_lowercase().contains(&self.input_query.to_lowercase())
                        && !item.2.to_lowercase().contains(&self.input_query.to_lowercase())
                    {
                        continue;
                    }

                    ui.monospace(item.0);
                    ui.label(egui::RichText::new(item.1).strong().color(egui::Color32::LIGHT_GREEN));
                    ui.label(item.2);
                    ui.label(egui::RichText::new(item.3).small().italics());
                    ui.label(item.4);
                    ui.end_row();
                }
            });
    }

    // --- TAB 2: GRAMATIKA U 4 STANJA ---
    fn render_grammar(&mut self, ui: &mut egui::Ui, _kernel: &QuatKernel4x8, q: &[u8; 8]) {
        ui.heading("📐 Baza-4 Kvatna Gramatika (4 States Engine)");
        ui.separator();

        ui.columns(2, |cols| {
            cols[0].group(|ui| {
                ui.label(egui::RichText::new("⚡ STANJA GLAGOLA U BAZI 4").strong());
                ui.separator();
                ui.label("Q0 (0.0V): Presente (Sadašnje vreme) — Ejemplo: Yo hablo");
                ui.label("Q1 (1.1V): Pasado (Prošlo vreme) — Ejemplo: Yo hablé");
                ui.label("Q2 (2.2V): Futuro (Buduće vreme) — Ejemplo: Yo hablaré");
                ui.label("Q3 (3.3V): Subjuntivo / Gritos — Ejemplo: ¡QUE HABLES!");

                ui.add_space(8.0);
                ui.label(format!("Trenutno stanje procesorskog registra: Q{}", q[0]));
            });

            cols[1].group(|ui| {
                ui.label(egui::RichText::new("🔥 SOFÍA'S ACCENT RULES").strong());
                ui.separator();
                ui.label("1. Slovo 'J' se uvek izgovara sa 3.3V napona (JAY = HAEEY).");
                ui.label("2. Ako nema emocije u rečenici, izbaci Carry Flag!");
                ui.label("3. Pridjevi idu iza imenice sa Q3 prioritetom.");
            });
        });
    }

    // --- TAB 3: SOFÍA ACCENT AUDIO SYNTHESIS ---
    fn render_audio_synth(&mut self, ui: &mut egui::Ui, kernel: &mut QuatKernel4x8, q: &[u8; 8]) {
        ui.heading("🎙️ Sofía Vergara Voice Pitch & Hardware Modulator");
        ui.separator();

        ui.group(|ui| {
            ui.label(egui::RichText::new("🎛️ QUAT-AUDIO MODULATOR (Hardware Drivers)").strong());
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Sintetizovani Napon Glasa:");
                ui.add(egui::Slider::new(&mut self.pitch_volts, 0.0..=3.3).text("Volti (V)"));
            });

            let state_code = match self.pitch_volts {
                v if v < 0.8 => Quat::Q0,
                v if v < 1.8 => Quat::Q1,
                v if v < 2.8 => Quat::Q2,
                _ => Quat::Q3,
            };

            ui.label(format!("Kvatno Stanje Zvuka: {:?} ({:.1}V)", state_code, self.pitch_volts));

            if ui.button("🔊 Testiraj Sofíin Uzvik (\"¡JAY!\")").clicked() {
                kernel.drivers.set_audio_state(state_code);
                self.active_quote = "¡JAAAAY! Look at the processor registers!".to_string();
            }
        });

        ui.add_space(8.0);

        // Spektrogram vizuelizacija
        ui.group(|ui| {
            ui.label(egui::RichText::new("📊 Voice Spectrum Wave (Base-4 Real-Time Waveform)").strong());
            ui.horizontal(|ui| {
                for i in 0..16 {
                    let h = (q[i % 8] as f32 + 1.0) * 12.0 * (self.pitch_volts + 0.5);
                    ui.add(egui::ProgressBar::new(h / 150.0).text(format!("{}", q[i % 8])));
                }
            });
        });
    }

    // --- TAB 4: SOFÍA KVIZ ---
    fn render_quiz(&mut self, ui: &mut egui::Ui, _kernel: &QuatKernel4x8) {
        ui.heading("🏆 Sofía Vergara Quaternary Spanish Quiz");
        ui.separator();

        ui.group(|ui| {
            ui.label(egui::RichText::new("Pitanje 1: Kako Sofía izgovara ime svog muža Jay-a kada je ljuta?").strong());
            ui.add_space(4.0);

            if ui.button("A) Jay (Normalno, 1.1V)").clicked() {
                self.active_quote = "Wrong, papi! Nemamo mi vremena za tišinu!".to_string();
            }
            if ui.button("B) ¡JAAAAAAAY! (Sa Q3 max naponom 3.3V)").clicked() {
                self.quiz_score_base4 += 1;
                self.active_quote = "¡CORRECTO! Tako se to radi u Barranquilli!".to_string();
            }
            if ui.button("C) Jason").clicked() {
                self.active_quote = "Ko je uopšte Jason?!".to_string();
            }

            ui.add_space(8.0);
            ui.label(format!("Tvoj Rezultat u Bazi 4: Q{}", self.quiz_score_base4 % 4));
        });
    }

    // --- TAB 5: 🐣 SECRET EASTER EGG — TOTY MODE ---
    fn render_toty_mode(&mut self, ui: &mut egui::Ui, kernel: &mut QuatKernel4x8, q: &[u8; 8]) {
        ui.group(|ui| {
            ui.vertical_centered(|ui| {
                ui.heading("🐣 EASTER EGG UNLOCKED: \"TOTY\" MODE!");
                ui.label(
                    egui::RichText::new("Sofía Vergara's Childhood Nickname: TOTY")
                        .strong()
                        .color(egui::Color32::GOLD)
                        .size(16.0),
                );
                ui.label("Dobrodošli u Barranquilla, Kolumbija! Procesor je na maksimalnom naponu Q3 (3.3V)!");
            });

            ui.add_space(8.0);
            ui.separator();

            ui.columns(2, |cols| {
                cols[0].group(|ui| {
                    ui.label(egui::RichText::new("🇨🇴 BARRANQUILLA STREET DICTIONARY").strong());
                    ui.separator();
                    ui.label("• ¡No joda!: Ne zezaj! (3.3V Overclock)");
                    ui.label("• ¡Qué bacano!: Kako je ovo dobro!");
                    ui.label("• A la orden: Uvek na usluzi!");
                    ui.label("• Pardo / Ajá: Šta ima / Pa naravno!");
                });

                cols[1].group(|ui| {
                    ui.label(egui::RichText::new("🚨 JAY PRITCHETT HARDWARE WARNING").strong());
                    ui.separator();
                    ui.label("STATUS: Gloria / Toty is yelling at Stella!");
                    ui.label(format!("Hardware Temp: {:.1} °C (OVERHEAT!)", 75.0 + (q[0] as f32 * 5.0)));
                    ui.label("ALU Logic: ALL FLAGS SET TO Q3");

                    if ui.button("💥 FORCE OVERCLOCK ALL QUATS TO Q3").clicked() {
                        kernel.reg_a.0 = 65535; // Sve jedinice na 3 (Q33333333)
                        self.active_quote = "¡PARRANDA EN BARRANQUILLA! All registers MAXED!".to_string();
                    }
                });
            });
        });
    }
}