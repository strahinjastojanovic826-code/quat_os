use eframe::egui;
use crate::kernel::{Quat, QuatKernel4x8};

/// Tipovi QuatLang instrukcija
#[derive(Debug, Clone, PartialEq)]
pub enum Opcode {
    Nop = 0,
    Set = 1,
    Load = 2,
    Store = 3,
    Add = 4,
    Sub = 5,
    Mul = 6,
    QNot = 7,
    Jmp = 8,
    Jz = 9,
    Jmax = 10,
    Yell = 11,
    Sua = 12,
    Volt = 13,
    Print = 14,
    Halt = 15,
}

/// Parsirana mašinska instrukcija spakovana u 8-kvatnu reč
#[derive(Clone, Debug)]
pub struct QuatInstruction {
    pub opcode: Opcode,
    pub reg_a: u8,
    pub reg_b: u8,
    pub immediate_or_addr: u16,
    pub raw_quat_string: String,
}

//Kad nakupis 200 greske u main gde ti ni bensendin ne pomaze
//Ali posle vidis da je od 200 140 gomila dosadnih promena iz bold u strong

pub struct QuatLangEngine {
    pub source_code: String,
    pub compiled_instructions: Vec<QuatInstruction>,
    pub execution_logs: Vec<String>,
    pub is_running: bool,
    pub program_counter: usize,
    pub selected_preset: usize,
}

impl Default for QuatLangEngine {
    fn default() -> Self {
        let default_script = r#"// QuatLang 4^8 Example: Loeb 40k Audit & Sofía Burst
SET Q0, 40000          // Budžet filma $40,000 (Base-4: Q22100100)
SET Q1, 15000          // Pravni troškovi tužbe
ADD Q0, Q1             // Saberi u Bazi 4
SUA 0x00FF             // Zamrzni stranicu memorije pred sudom u LA
YELL "JAAAAAY!"        // Sofía 3.3V Audio Burst
PRINT Q0               // Prikazi krajnji saldo
HALT                   // Kraj programa"#;

        let mut engine = Self {
            source_code: default_script.to_string(),
            compiled_instructions: Vec::new(),
            execution_logs: vec!["[IDE] QuatLang 4^8 IDE Initialized.".to_string()],
            is_running: false,
            program_counter: 0,
            selected_preset: 0,
        };
        engine.compile_source();
        engine
    }
}

impl QuatLangEngine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Glavni UI IDE-a za QuatLang
    pub fn ui(&mut self, ui: &mut egui::Ui, kernel: &mut QuatKernel4x8) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.heading("⚡ QuatLang 4^8 Compiler & Interactive IDE");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("▶ REKORDI I POKRENI (RUN)").clicked() {
                        self.compile_source();
                        self.run_all(kernel);
                    }
                    if ui.button("⚙️ ASEMBLIRAJ (COMPILE)").clicked() {
                        self.compile_source();
                    }
                    if ui.button("⏭ STEPER (1 STEP)").clicked() {
                        self.step_execution(kernel);
                    }
                });
            });

            ui.separator();

            // PRESET SKRIPTE
            ui.horizontal(|ui| {
                ui.label("📜 Učitaj Primer Skripte:");
                if ui.selectable_label(self.selected_preset == 0, "🧅 Loeb 40k Audit").clicked() {
                    self.selected_preset = 0;
                    self.source_code = "// Loeb $40k Film Audit\nSET Q0, 40000\nSUA 0x00FE\nPRINT Q0\nHALT".to_string();
                    self.compile_source();
                }
                if ui.selectable_label(self.selected_preset == 1, "💃 Sofía 3.3V Scream").clicked() {
                    self.selected_preset = 1;
                    self.source_code = "// Sofía Scream Script\nSET Q0, 65535\nYELL \"¡AY POR DIOS!\"\nPRINT Q0\nHALT".to_string();
                    self.compile_source();
                }
                if ui.selectable_label(self.selected_preset == 2, "🧮 Quat Fibonacci").clicked() {
                    self.selected_preset = 2;
                    self.source_code = "// Base-4 Quat Fibonacci\nSET Q0, 1\nSET Q1, 1\nADD Q0, Q1\nPRINT Q0\nHALT".to_string();
                    self.compile_source();
                }
            });

            ui.add_space(6.0);

            // GLAVNI RADNI PROSTOR (EDITOR + BINARY VIEW + CONSOLE)
            ui.columns(2, |cols| {
                // LEVA KOLONA: SOURCE CODE EDITOR
                cols[0].group(|ui| {
                    ui.label(egui::RichText::new("📝 EDITOVANJE IZVORNOG KODA (QuatLang Syntax)").strong());
                    ui.add_space(4.0);

                    let code_response = ui.add(
                        egui::TextEdit::multiline(&mut self.source_code)
                            .font(egui::FontId::monospace(13.0))
                            .desired_rows(16)
                            .desired_width(f32::INFINITY),
                    );

                    if code_response.changed() {
                        self.compile_source();
                    }
                });

                // DESNA KOLONA: ASEMBLIRANI 8-KVATNI MAŠINSKI KOD
                cols[1].group(|ui| {
                    ui.label(egui::RichText::new("🔩 MAŠINSKI ASEMBLER ($4^8$ Binary Output)").strong());
                    ui.add_space(4.0);

                    egui::ScrollArea::vertical().max_height(250.0).show(ui, |ui| {
                        egui::Grid::new("binary_compiled_grid")
                            .striped(true)
                            .min_col_width(70.0)
                            .show(ui, |ui| {
                                ui.strong("PC");
                                ui.strong("Opcode");
                                ui.strong("8-Kvatni Mašinski Kod");
                                ui.strong("Hex");
                                ui.end_row();

                                for (idx, instr) in self.compiled_instructions.iter().enumerate() {
                                    let is_current = idx == self.program_counter;
                                    let color = if is_current {
                                        egui::Color32::GREEN
                                    } else {
                                        egui::Color32::LIGHT_GRAY
                                    };

                                    ui.monospace(egui::RichText::new(format!("0x{:02X}", idx)).color(color));
                                    ui.label(egui::RichText::new(format!("{:?}", instr.opcode)).color(color));
                                    ui.monospace(egui::RichText::new(&instr.raw_quat_string).strong().color(color));
                                    ui.monospace(egui::RichText::new(format!("0x{:04X}", instr.immediate_or_addr)).color(color));
                                    ui.end_row();
                                }
                            });
                    });
                });
            });

            ui.add_space(6.0);

            // KONZOLA ZA ISPIS I LO GOVE
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("🖥️ QUATLANG EXECUTION CONSOLE").strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("🗑️ Očisti Konzolu").clicked() {
                            self.execution_logs.clear();
                        }
                    });
                });

                ui.separator();

                egui::ScrollArea::vertical().max_height(100.0).show(ui, |ui| {
                    for log in self.execution_logs.iter().rev() {
                        let color = if log.contains("ERROR") || log.contains("SUA") {
                            egui::Color32::LIGHT_RED
                        } else if log.contains("YELL") || log.contains("PRINT") {
                            egui::Color32::GOLD
                        } else {
                            egui::Color32::GREEN
                        };

                        ui.monospace(egui::RichText::new(log).color(color));
                    }
                });
            });
        });
    }

    /// COMPILER: Prevođenje teksta u 8-Kvatne Mašinske Instrukcije
    pub fn compile_source(&mut self) {
        self.compiled_instructions.clear();
        let lines = self.source_code.lines();

        for line in lines {
            let clean_line = line.split("//").next().unwrap_or("").trim();
            if clean_line.is_empty() {
                continue;
            }

            let parts: Vec<&str> = clean_line.split_whitespace().collect();
            let cmd = parts[0].to_uppercase();

            let (opcode, reg_a, reg_b, imm) = match cmd.as_str() {
                "NOP" => (Opcode::Nop, 0, 0, 0),
                "SET" => {
                    let reg = parse_reg(parts.get(1).unwrap_or(&"Q0"));
                    let val = parse_val(parts.get(2).unwrap_or(&"0"));
                    (Opcode::Set, reg, 0, val)
                }
                "ADD" => {
                    let r1 = parse_reg(parts.get(1).unwrap_or(&"Q0"));
                    let r2 = parse_reg(parts.get(2).unwrap_or(&"Q1"));
                    (Opcode::Add, r1, r2, 0)
                }
                "SUB" => {
                    let r1 = parse_reg(parts.get(1).unwrap_or(&"Q0"));
                    let r2 = parse_reg(parts.get(2).unwrap_or(&"Q1"));
                    (Opcode::Sub, r1, r2, 0)
                }
                "SUA" => {
                    let addr = parse_val(parts.get(1).unwrap_or(&"0x0000"));
                    (Opcode::Sua, 0, 0, addr)
                }
                "YELL" => (Opcode::Yell, 3, 3, 65535),
                "PRINT" => {
                    let reg = parse_reg(parts.get(1).unwrap_or(&"Q0"));
                    (Opcode::Print, reg, 0, 0)
                }
                "HALT" => (Opcode::Halt, 0, 0, 0),
                _ => (Opcode::Nop, 0, 0, 0),
            };

            // Enkodiranje u 8-Kvatni Mašinski Format
            let raw_quat = format!(
                "Q{:02b}Q{:02b}Q{:02b}Q{:02b}",
                opcode.clone() as u8, reg_a, reg_b, (imm & 0x03) as u8
            );

            self.compiled_instructions.push(QuatInstruction {
                opcode,
                reg_a,
                reg_b,
                immediate_or_addr: imm,
                raw_quat_string: raw_quat,
            });
        }
    }

    /// EXECUTOR: Izvršavanje trenutne instrukcije
    pub fn step_execution(&mut self, kernel: &mut QuatKernel4x8) {
        if self.program_counter >= self.compiled_instructions.len() {
            self.program_counter = 0;
        }

        if self.compiled_instructions.is_empty() {
            return;
        }

        let instr = self.compiled_instructions[self.program_counter].clone();
        self.execution_logs.push(format!(
            "[EXEC PC:0x{:02X}] Opcode: {:?} | Machine Quat: {}",
            self.program_counter, instr.opcode, instr.raw_quat_string
        ));

        match instr.opcode {
            Opcode::Set => {
                if instr.reg_a == 0 {
                    kernel.reg_a.0 = instr.immediate_or_addr;
                } else {
                    kernel.reg_b.0 = instr.immediate_or_addr;
                }
                self.execution_logs.push(format!("  -> Reg Q{} set to {}", instr.reg_a, instr.immediate_or_addr));
            }
            Opcode::Add => {
                let res = kernel.reg_a.0.wrapping_add(kernel.reg_b.0);
                kernel.reg_a.0 = res;
                self.execution_logs.push(format!("  -> ADD Result in Reg A: {} (0x{:04X})", res, res));
            }
            Opcode::Sua => {
                self.execution_logs.push(format!(
                    "  -> 🚨 NICK LOEB LITIGATION LOCK FILED on address 0x{:04X}! Page Frozen!",
                    instr.immediate_or_addr
                ));
            }
            Opcode::Yell => {
                kernel.drivers.set_audio_state(Quat::Q3);
                self.execution_logs.push("  -> 📢 SOFÍA VERGARA SCREAM BURST (3.3V MAX VOLTS ACTIVATED!)".to_string());
            }
            Opcode::Print => {
                let val = if instr.reg_a == 0 { kernel.reg_a.0 } else { kernel.reg_b.0 };
                self.execution_logs.push(format!("  -> 🖨️ OUT: Reg Q{} = {} (Base-4 Quat String)", instr.reg_a, val));
            }
            Opcode::Halt => {
                self.execution_logs.push("  -> 🛑 PROCESSOR HALTED.".to_string());
            }
            _ => {}
        }

        self.program_counter += 1;
    }

    pub fn run_all(&mut self, kernel: &mut QuatKernel4x8) {
        self.program_counter = 0;
        let len = self.compiled_instructions.len();
        for _ in 0..len {
            self.step_execution(kernel);
        }
    }
}

fn parse_reg(s: &str) -> u8 {
    let clean = s.replace(",", "").replace("Q", "").trim().to_string();
    clean.parse::<u8>().unwrap_or(0) % 4
}

fn parse_val(s: &str) -> u16 {
    let clean = s.replace(",", "").trim().to_string();
    if clean.starts_with("0x") {
        u16::from_str_radix(&clean[2..], 16).unwrap_or(0)
    } else {
        clean.parse::<u16>().unwrap_or(0)
    }
}