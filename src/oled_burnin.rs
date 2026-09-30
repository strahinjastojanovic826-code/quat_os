use eframe::egui;
use crate::kernel::{Quat, QuatKernel4x8};

/// OLED Display Hardware Burn-In & Sub-Pixel Stress Test Simulator
pub struct OledBurnInEngine {
    pub pixel_degradation: [f32; 64], // Matrica 8x8 sub-piksela (degradacija 0.0% do 100.0%)
    pub pixel_voltages: [u8; 64],     // Trenutni naponski nivo po pikselu (0..=3)
    pub panel_temp_celsius: f32,       // Temperatura panela u °C
    pub pixel_shift_active: bool,      // Anti-burn-in zaštitni mehanizam
    pub stress_test_running: bool,     // Status testa opterećenja
    pub total_stress_cycles: u32,
    pub log_history: Vec<String>,
}

impl Default for OledBurnInEngine {
    fn default() -> Self {
        Self {
            pixel_degradation: [0.0; 64],
            pixel_voltages: [0; 64],
            panel_temp_celsius: 35.0,
            pixel_shift_active: true,
            stress_test_running: false,
            total_stress_cycles: 0,
            log_history: vec![
                "[OLED] Sub-Pixel Thermal & Luminance Decay Engine initialized.".to_string(),
                "[HARDWARE] Organic Emitter Panel mapped to Base-4 Voltage Lines.".to_string(),
            ],
        }
    }
}

impl OledBurnInEngine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Takt simulacije - preračunava starenje organskog sloja na osnovu napona i toplote
    pub fn tick(&mut self, kernel: &mut QuatKernel4x8) {
        let q = kernel.reg_a.to_quats();
        let base_quat_volts = q[0] as u8; // Koristimo napon iz Registra A kao referencu

        let mut sum_deg = 0.0;

        for i in 0..64 {
            let current_quat = if self.stress_test_running {
                3 // Tokom stress testa gurnemo sve piksele na MAX 3.3V
            } else {
                (self.pixel_voltages[i] + base_quat_volts) % 4
            };

            self.pixel_voltages[i] = current_quat;

            // Degradacija raste ubrzano na višem naponu
            let deg_delta = match current_quat {
                0 => 0.0,   // 0.0V (Q0) - OLED True Black (nema starenja)
                1 => 0.02,  // 1.1V (Q1) - Minimalno starenje
                2 => 0.08,  // 2.2V (Q2) - Umereno starenje
                _ => 0.38,  // 3.3V (Q3) - MAX NAPON (Ekstremni burn-in pritisak)
            };

            // Pixel Shift redukuje lokalizovani burn-in
            let shift_factor = if self.pixel_shift_active { 0.65 } else { 1.0 };
            self.pixel_degradation[i] = (self.pixel_degradation[i] + deg_delta * shift_factor).min(100.0);
            sum_deg += self.pixel_degradation[i];
        }

        let avg_deg = sum_deg / 64.0;
        self.panel_temp_celsius = 35.0 + (avg_deg * 0.48);
        self.total_stress_cycles += 1;

        if avg_deg > 75.0 && self.total_stress_cycles % 10 == 0 {
            self.log_history.push("[CRITICAL] Permanent Organic Sub-Pixel Burn-In Detected!".to_string());
        }
    }

    /// Glavni UI Render za OLED Stress Test Simulator
    pub fn ui(&mut self, ui: &mut egui::Ui, kernel: &mut QuatKernel4x8) {
        // Ako je stress test aktivan, osvežavamo simulaciju svaki frejm
        if self.stress_test_running {
            self.tick(kernel);
            ui.ctx().request_repaint();
        }

        let avg_degradation = self.pixel_degradation.iter().sum::<f32>() / 64.0;
        let health_percentage = (100.0 - avg_degradation).max(0.0);

        // 1. ZAGLAVLJE MODULA
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.heading("🖥️ OLED & Display Hardware Burn-In Simulator ($4^8$)");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let (status_text, bg_color) = if self.panel_temp_celsius > 65.0 {
                        ("CRITICAL OVERHEAT (>65°C)", egui::Color32::LIGHT_RED)
                    } else if self.stress_test_running {
                        ("STRESS TEST ACTIVE (3.3V BLAST)", egui::Color32::GOLD)
                    } else {
                        ("PANEL NOMINAL", egui::Color32::GREEN)
                    };

                    ui.label(
                        egui::RichText::new(status_text)
                            .strong()
                            .color(egui::Color32::BLACK)
                            .background_color(bg_color),
                    );
                });
            });

            ui.label(
                egui::RichText::new(format!(
                    "Zdravlje Panela: {:.1}% | Temp Panela: {:.1}°C | Ciklusa Testiranja: {}",
                    health_percentage, self.panel_temp_celsius, self.total_stress_cycles
                ))
                .font(egui::FontId::proportional(13.0))
                .color(egui::Color32::LIGHT_BLUE),
            );
        });

        ui.separator();
        ui.add_space(4.0);

        // 2. INTERFEJS I KONTROLE
        ui.columns(2, |cols| {
            // LEVI PANEL: 8x8 HEATMAP MATRICA
            cols[0].group(|ui| {
                ui.label(egui::RichText::new("🔥 HEATMAP & DEGRADACIJA PIKSELA (8x8 Panel)").strong());
                ui.label("Klikni na piksel da ga ručno spržiš na 3.3V:");
                ui.add_space(6.0);

                egui::Grid::new("oled_pixel_grid")
                    .spacing([4.0, 4.0])
                    .show(ui, |ui| {
                        for y in 0..8 {
                            for x in 0..8 {
                                let idx = y * 8 + x;
                                let deg = self.pixel_degradation[idx];
                                let volt = self.pixel_voltages[idx];

                                // Izračunavanje boje piksela na osnovu nivoa degradacije
                                let color = if deg < 20.0 {
                                    egui::Color32::from_rgb(0, (255.0 - deg * 5.0) as u8, 120)
                                } else if deg < 60.0 {
                                    egui::Color32::from_rgb(255, 200, 0)
                                } else {
                                    egui::Color32::from_rgb(255, (255.0 - (deg - 60.0) * 6.0).max(0.0) as u8, 0)
                                };

                                let btn_text = format!("Q{}\n{:.0}%", volt, deg);
                                if ui.add_sized(
                                    [36.0, 36.0],
                                    egui::Button::new(
                                        egui::RichText::new(btn_text)
                                            .font(egui::FontId::proportional(9.0))
                                            .color(egui::Color32::BLACK)
                                    ).fill(color)
                                ).clicked() {
                                    self.pixel_voltages[idx] = 3;
                                    self.pixel_degradation[idx] = (self.pixel_degradation[idx] + 20.0).min(100.0);
                                    self.log_history.push(format!("[MANUAL] Sub-pixel ({},{}) blast to Q3 (3.3V)!", x, y));
                                }
                            }
                            ui.end_row();
                        }
                    });
            });

            // DESNI PANEL: KONTROLE TESTIRANJA I PREVENCIJA
            cols[1].group(|ui| {
                ui.label(egui::RichText::new("⚡ KONTROLA STRESS TESTA I ZAŠTITE").strong());
                ui.separator();

                if self.stress_test_running {
                    if ui.button("⏹️ Zaustavi OLED Stress Test").clicked() {
                        self.stress_test_running = false;
                        self.log_history.push("[OLED] Stress test manual stop.".to_string());
                    }
                } else {
                    if ui.button("🚀 Pokreni Continuous Q3 (3.3V) Burn-In Test").clicked() {
                        self.stress_test_running = true;
                        kernel.reg_a.write_quat(0, Quat::Q3);
                        self.log_history.push("[OLED] Continuous 3.3V Overvoltage Stress Test started!".to_string());
                    }
                }

                ui.add_space(8.0);
                ui.checkbox(&mut self.pixel_shift_active, "🛡️ Aktiviraj Pixel Shift / Anti-Burn-In Zaštitu");

                ui.add_space(8.0);
                if ui.button("🔄 Pokreni Pixel Refresh Ciklus (Oporavak)").clicked() {
                    for deg in self.pixel_degradation.iter_mut() {
                        *deg = (*deg - 25.0).max(0.0);
                    }
                    self.panel_temp_celsius = 35.0;
                    self.log_history.push("[MAINTENANCE] Panel Refresh executed. Sub-pixel stress relieved.".to_string());
                }

                ui.add_space(12.0);
                ui.label(egui::RichText::new("Naponske Legende Baze-4:").strong());
                ui.label("• Q0 (0.0V): Isključeno (OLED True Black - 0% starenja)");
                ui.label("• Q1 (1.1V): Niski napon (Minimalno starenje)");
                ui.label("• Q2 (2.2V): Standardni rad (Umereno starenje)");
                ui.label("• Q3 (3.3V): Max svetlosni pritisak (Zagrevanje & Burn-In)");
            });
        });

        ui.add_space(8.0);

        // 3. HARDVERSKI DNEVNIK
        ui.group(|ui| {
            ui.label(egui::RichText::new("📜 HARDWARE DIAGNOSTIC LOG").strong());
            ui.separator();
            egui::ScrollArea::vertical().max_height(100.0).show(ui, |ui| {
                for log in self.log_history.iter().rev() {
                    ui.monospace(log);
                }
            });
        });
    }
}