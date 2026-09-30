use eframe::egui;
use crate::kernel::QuatKernel4x8;

/// Tipovi linija u terminalu za razlikovanje boja
#[derive(Clone)]
pub enum LineType {
    Command,
    Output,
    Error,
    Success,
    SystemHeader,
}

#[derive(Clone)]
pub struct TerminalLine {
    pub text: String,
    pub line_type: LineType,
}

/// Kompletan interaktivni Terminal za 4^8 sistem
pub struct QuatTerminal {
    input_buffer: String,
    history: Vec<TerminalLine>,
    command_history: Vec<String>,
    cmd_history_idx: usize,
    auto_scroll: bool,
}

impl Default for QuatTerminal {
    fn default() -> Self {
        let mut term = Self {
            input_buffer: String::new(),
            history: Vec::new(),
            command_history: Vec::new(),
            cmd_history_idx: 0,
            auto_scroll: true,
        };

        // Inicijalni sistemski "banner"
        term.history.push(TerminalLine {
            text: "QuatOS 4^8 Interactive Command Shell v1.0".to_string(),
            line_type: LineType::SystemHeader,
        });
        term.history.push(TerminalLine {
            text: "Arhitektura: 8-Quat Base-4 Processor | 65,536 Address Space".to_string(),
            line_type: LineType::SystemHeader,
        });
        term.history.push(TerminalLine {
            text: "Ukucaj 'help' ili '?' za prikaz dostupnih komandi.\n".to_string(),
            line_type: LineType::SystemHeader,
        });

        term
    }
}

impl QuatTerminal {
    pub fn new() -> Self {
        Self::default()
    }

    /// Glavna UI metoda za prikaz Terminala
    pub fn ui(&mut self, ui: &mut egui::Ui, kernel: &mut QuatKernel4x8) {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.heading("🖥️ QuatEngine 4^8 — Interactive Terminal Shell");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("🗑️ Očisti Logs").clicked() {
                        self.history.clear();
                    }
                    if ui.button("📋 Status Registara").clicked() {
                        self.execute_cmd("peek", kernel);
                    }
                    if ui.button("❓ Help").clicked() {
                        self.execute_cmd("help", kernel);
                    }
                });
            });

            ui.separator();

            // 1. OBLAST ISPISA TERMILANCA (CONSOLE OUTPUT)
            let text_style = egui::TextStyle::Monospace;
            let _row_height = ui.text_style_height(&text_style);

            egui::ScrollArea::vertical()
                .max_height(320.0)
                .min_scrolled_height(200.0)
                .stick_to_bottom(self.auto_scroll)
                .show(ui, |ui| {
                    for line in &self.history {
                        let color = match line.line_type {
                            LineType::Command => egui::Color32::LIGHT_BLUE,
                            LineType::Output => egui::Color32::LIGHT_GRAY,
                            LineType::Error => egui::Color32::LIGHT_RED,
                            LineType::Success => egui::Color32::GREEN,
                            LineType::SystemHeader => egui::Color32::GOLD,
                        };

                        ui.label(
                            egui::RichText::new(&line.text)
                                .font(egui::FontId::monospace(13.0))
                                .color(color),
                        );
                    }
                });

            ui.separator();

            // 2. KOMANDNA LINIJA (PROMPT)
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("QuatOS 4^8 >")
                        .font(egui::FontId::monospace(14.0))
                        .color(egui::Color32::GREEN)
                        .strong(),
                );

                let response = ui.add(
                    egui::TextEdit::singleline(&mut self.input_buffer)
                        .desired_width(f32::INFINITY)
                        .font(egui::FontId::monospace(13.0))
                        .hint_text("Unesi komandu (npr. help, peek, step 5, convert 255)..."),
                );

                // Provera za Enter key
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    let cmd = self.input_buffer.trim().to_string();
                    if !cmd.is_empty() {
                        self.history.push(TerminalLine {
                            text: format!("QuatOS 4^8 > {}", cmd),
                            line_type: LineType::Command,
                        });
                        self.command_history.push(cmd.clone());
                        self.cmd_history_idx = self.command_history.len();

                        self.execute_cmd(&cmd, kernel);
                        self.input_buffer.clear();
                    }
                    response.request_focus();
                }
            });
        });
    }

    /// PARSER I EXECUTOR ZA KVATNE KOMANDE
    fn execute_cmd(&mut self, cmd_raw: &str, kernel: &mut QuatKernel4x8) {
        let parts: Vec<&str> = cmd_raw.trim().split_whitespace().collect();
        if parts.is_empty() {
            return;
        }

        let main_cmd = parts[0].to_lowercase();

        match main_cmd.as_str() {
            "help" | "?" => {
                self.print_out("--- QUATOS 4^8 SHELL KOMANDE ---", LineType::SystemHeader);
                self.print_out("  help / ?              - Prikazuje ovo uputstvo", LineType::Output);
                self.print_out("  peek / regs           - Prikazuje registre A, B, PC i Flags", LineType::Output);
                self.print_out("  step [n]              - Izvršava n taktnih ciklusa (default: 1)", LineType::Output);
                self.print_out("  convert <dec_broj>    - Prevara dekadni broj u 8-kvatni Base-4 format", LineType::Output);
                self.print_out("  set rega <Q0123... >  - Postavlja vrednost Registra A", LineType::Output);
                self.print_out("  sysinfo               - Specifikacija 4^8 hardverske arhitekture", LineType::Output);
                self.print_out("  clear / cls           - Čisti ekran terminala", LineType::Output);
            }

            "peek" | "regs" => {
                let q_a = kernel.reg_a.to_quat_str();
                let q_b = kernel.reg_b.to_quat_str();
                self.print_out(&format!("REG A: {} (Dekadno: {})", q_a, kernel.reg_a.0), LineType::Success);
                self.print_out(&format!("REG B: {} (Dekadno: {})", q_b, kernel.reg_b.0), LineType::Success);
                self.print_out(&format!("Program Counter (PC): 0x{:04X} ({})", kernel.pc, kernel.pc), LineType::Output);
                self.print_out(&format!("Carry Flag: {}", kernel.carry_flag), LineType::Output);
            }

            "step" => {
                let steps = if parts.len() > 1 {
                    parts[1].parse::<usize>().unwrap_or(1)
                } else {
                    1
                };

                for _ in 0..steps {
                    kernel.step();
                }
                self.print_out(&format!("Izvršeno {} taktnih ciklusa. Novi PC: 0x{:04X}", steps, kernel.pc), LineType::Success);
            }

            "convert" => {
                if parts.len() < 2 {
                    self.print_out("Greška: Nedostaje dekadni broj! Primer: convert 150", LineType::Error);
                    return;
                }

                if let Ok(num) = parts[1].parse::<u16>() {
                    let mut temp = num;
                    let mut quats = [0u8; 8];
                    for i in (0..8).rev() {
                        quats[i] = (temp % 4) as u8;
                        temp /= 4;
                    }
                    let quat_str: String = quats.iter().map(|q| format!("Q{}", q)).collect();
                    self.print_out(&format!("Dekadno {} = Base-4 8-Quat: {}", num, quat_str), LineType::Success);
                } else {
                    self.print_out("Greška: Neispravan broj! Unesi celobrojnu vrednost 0-65535.", LineType::Error);
                }
            }

            "sysinfo" => {
                self.print_out("--- HARDVERSKA SPECIFIKACIJA ---", LineType::SystemHeader);
                self.print_out("  Sistem: QuatEngine 8-Quat Architecture", LineType::Output);
                self.print_out("  Kvatna Stranica: Baza 4 (Stanja: Q0=0V, Q1=1.1V, Q2=2.2V, Q3=3.3V)", LineType::Output);
                self.print_out("  Ukupan Adresni Prostor: 4^8 = 65,536 QWords", LineType::Output);
                self.print_out("  CPU Jezgra: 4 Četverna Jezgra (4 Parallel Core Units)", LineType::Output);
            }

            "clear" | "cls" => {
                self.history.clear();
            }

            _ => {
                self.print_out(
                    &format!("Nepoznata komanda: '{}'. Ukucaj 'help' za spisak komandi.", main_cmd),
                    LineType::Error,
                );
            }
        }
    }

    fn print_out(&mut self, msg: &str, ltype: LineType) {
        self.history.push(TerminalLine {
            text: msg.to_string(),
            line_type: ltype,
        });
    }
}