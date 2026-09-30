use std::collections::VecDeque;

pub struct Osciloskop {
    voltage_history_a: VecDeque<f32>,
    voltage_history_b: VecDeque<f32>,
    max_samples: usize,
}

impl Osciloskop {
    pub fn new(max_samples: usize) -> Self {
        Self {
            voltage_history_a: VecDeque::with_capacity(max_samples),
            voltage_history_b: VecDeque::with_capacity(max_samples),
            max_samples,
        }
    }

    /// Bilježi trenutni napon registara (Q0-Q3 prevođenje u 0.0V - 3.3V)
    pub fn record(&mut self, reg_a_val: u16, reg_b_val: u16) {
        let v_a = ((reg_a_val & 0b11) as f32) * 1.1;
        let v_b = ((reg_b_val & 0b11) as f32) * 1.1;

        if self.voltage_history_a.len() >= self.max_samples {
            self.voltage_history_a.pop_front();
        }
        self.voltage_history_a.push_back(v_a);

        if self.voltage_history_b.len() >= self.max_samples {
            self.voltage_history_b.pop_front();
        }
        self.voltage_history_b.push_back(v_b);
    }

    /// Crta grafički prikaz osciloskopa
    pub fn ui(&self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.heading("📈 Analogni Osciloskop (Signal Registara)");

            ui.horizontal(|ui| {
                ui.colored_label(egui::Color32::from_rgb(0, 200, 255), "― Reg A Napon");
                ui.label(" | ");
                ui.colored_label(egui::Color32::from_rgb(255, 180, 0), "― Reg B Napon");
            });

            ui.add_space(4.0);

            let canvas_size = egui::vec2(ui.available_width(), 120.0);
            let (rect, _response) = ui.allocate_exact_size(canvas_size, egui::Sense::hover());
            let painter = ui.painter_at(rect);

            // Pozadina i ram
            painter.rect_filled(rect, 4.0, egui::Color32::from_rgb(15, 20, 25));
            painter.rect_stroke(
                rect, 
                4.0, 
                egui::Stroke::new(1.0, egui::Color32::from_rgb(40, 60, 80)),
                egui::StrokeKind::Middle
            );

            // Referentne linije (Q0 - Q3)
            let voltages = [("3.3V (Q3)", 3.3), ("2.2V (Q2)", 2.2), ("1.1V (Q1)", 1.1), ("0.0V (Q0)", 0.0)];
            for (label, v) in voltages {
                let norm_y = 1.0 - (v / 3.3);
                let y = rect.min.y + 10.0 + norm_y * (rect.height() - 20.0);
                
                painter.line_segment(
                    [egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)],
                    egui::Stroke::new(0.5, egui::Color32::from_rgb(50, 70, 90))
                );
                painter.text(
                    egui::pos2(rect.min.x + 5.0, y - 6.0),
                    egui::Align2::LEFT_TOP,
                    label,
                    egui::FontId::monospace(9.0),
                    egui::Color32::from_rgb(100, 130, 160)
                );
            }

            // Linija za Reg A
            if self.voltage_history_a.len() > 1 {
                let points: Vec<egui::Pos2> = self.voltage_history_a.iter().enumerate().map(|(i, &v)| {
                    let x = rect.min.x + (i as f32 / (self.max_samples - 1) as f32) * rect.width();
                    let norm_y = 1.0 - (v / 3.3).clamp(0.0, 1.0);
                    let y = rect.min.y + 10.0 + norm_y * (rect.height() - 20.0);
                    egui::pos2(x, y)
                }).collect();

                painter.line(points, egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 200, 255)));
            }

            // Linija za Reg B
            if self.voltage_history_b.len() > 1 {
                let points: Vec<egui::Pos2> = self.voltage_history_b.iter().enumerate().map(|(i, &v)| {
                    let x = rect.min.x + (i as f32 / (self.max_samples - 1) as f32) * rect.width();
                    let norm_y = 1.0 - (v / 3.3).clamp(0.0, 1.0);
                    let y = rect.min.y + 10.0 + norm_y * (rect.height() - 20.0);
                    egui::pos2(x, y)
                }).collect();

                painter.line(points, egui::Stroke::new(1.5, egui::Color32::from_rgb(255, 180, 0)));
            }
        });
    }
}