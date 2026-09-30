use eframe::egui;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

/// Digitronski Sat sa podrškom za Real-Time i Kvatno Vreme (Base-4)
pub struct DigitalClock {
    start_time: Instant,
}

impl Default for DigitalClock {
    fn default() -> Self {
        Self {
            start_time: Instant::now(),
        }
    }
}

impl DigitalClock {
    pub fn new() -> Self {
        Self::default()
    }

    /// Glavna render metoda - Otkucava glatko u realnom vremenu
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        // Tražimo od egui-ja repaint u svakom frejmu da bi sat otkucavao bez zastoja
        ui.ctx().request_repaint();

        // 1. Proračun Sistemskog Vremena (Sati, Minuti, Sekunde)
        let now_secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Korekcija za vremensku zonu (UTC + 2 CEST)
        let hours = ((now_secs / 3600) + 2) % 24;
        let mins = (now_secs / 60) % 60;
        let secs = now_secs % 60;

        // 2. Proračun Uptime-a i Milisekundi (za digitronski efekat brzo menjajućih cifara)
        let elapsed = self.start_time.elapsed();
        let millis = (elapsed.as_millis() % 1000) / 10; // 00 - 99

        // 3. Proračun Kvatnog Vremena (Sekunde preveđene u 8-Kvatni Baza-4 format)
        let up_secs = (elapsed.as_secs() % 65536) as u16;
        let mut q_str = String::new();
        let mut temp = up_secs;
        for _ in 0..8 {
            let quat_digit = temp % 4;
            q_str.insert_0(&format!("Q{}", quat_digit));
            temp /= 4;
        }

        // --- DIGITRONSKI ZELENI DISPLAY EKRAN ---
        ui.group(|ui| {
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new("⏱️ QUAT-ENGINE RTC DIGITRON CLOCK")
                        .strong()
                        .small()
                        .color(egui::Color32::GRAY),
                );

                ui.add_space(3.0);

                // Digitronsko crno-zeleno kućište ekrana
                egui::Frame::canvas(ui.style())
                    .fill(egui::Color32::from_rgb(10, 20, 10)) // Tamna digitronska podloga
                    .stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 180, 80))) // Svetleći okvir
                    .inner_margin(10.0)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            // A) Real-Time Sistemski Sat (HH:MM:SS.ms)
                            let time_formatted = format!("{:02}:{:02}:{:02}.{:02}", hours, mins, secs, millis);
                            ui.label(
                                egui::RichText::new(time_formatted)
                                    .font(egui::FontId::monospace(26.0))
                                    .color(egui::Color32::from_rgb(50, 255, 100)) // Zeleni fluorescentni LED
                                    .strong(),
                            );

                            ui.add_space(15.0);
                            ui.separator();
                            ui.add_space(15.0);

                            // B) Base-4 Kvatno Vreme (4^8 Ticks)
                            ui.vertical(|ui| {
                                ui.label(
                                    egui::RichText::new("UPTIME (Base-4)")
                                        .font(egui::FontId::monospace(10.0))
                                        .color(egui::Color32::from_rgb(0, 200, 150)),
                                );
                                ui.label(
                                    egui::RichText::new(q_str)
                                        .font(egui::FontId::monospace(13.0))
                                        .color(egui::Color32::from_rgb(0, 255, 220))
                                        .strong(),
                                );
                            });
                        });
                    });
            });
        });
    }
}

// Pomoćna ekstenzija za dodavanje stringa na početak
trait InsertStart {
    fn insert_0(&mut self, s: &str);
}
impl InsertStart for String {
    fn insert_0(&mut self, s: &str) {
        self.insert_str(0, s);
    }
}