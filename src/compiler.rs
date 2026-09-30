pub struct CodeCompiler {
    pub source_code: String,
    pub target_addr: u16,
    pub reset_pc_on_flash: bool,
    pub last_status: Result<usize, String>, // Broj upisanih reči ili greška
}

impl CodeCompiler {
    pub fn new() -> Self {
        let default_program = r#"// --- 4^8 Live Flash Demo ---
// Svaka reč ima 8 kvatova (16 bita)
MOV A, 3
MOV B, 2
ADD A, B
STORE A, 0xE000
JMP 0x0000
"#
        .to_string();

        Self {
            source_code: default_program,
            target_addr: 0x0000,
            reset_pc_on_flash: true,
            last_status: Ok(0),
        }
    }

    /// Pretvara 16-bitni broj u 8-kvatni string (npr. Q0Q1Q2Q3 Q0Q0Q3Q3)
    pub fn to_quat_str(val: u16) -> String {
        let mut q_str = String::with_capacity(9);
        for i in (0..8).rev() {
            let q = (val >> (i * 2)) & 0b11;
            q_str.push_str(&format!("Q{}", q));
            if i == 4 {
                q_str.push(' ');
            }
        }
        q_str
    }

    /// Kompajlira tekstualni asembler u 16-bitne $4^8$ opkodove
    pub fn compile(&self) -> Result<Vec<u16>, String> {
        let mut bytecode = Vec::new();

        for (line_num, line) in self.source_code.lines().enumerate() {
            // Skini komentare (označene sa // ili ;)
            let clean_line = line.split("//").next().unwrap().split(';').next().unwrap().trim();
            if clean_line.is_empty() {
                continue;
            }

            let parts: Vec<&str> = clean_line.split_whitespace().collect();
            let op = parts[0].to_uppercase();

            match op.as_str() {
                "NOP" => bytecode.push(0x0000), // Q0Q0Q0Q0 Q0Q0Q0Q0
                "HALT" => bytecode.push(0xFFFF), // Q3Q3Q3Q3 Q3Q3Q3Q3

                "MOV" => {
                    if parts.len() < 3 {
                        return Err(format!("Linija {}: MOV traži registar i vrednost (npr. MOV A, 3)", line_num + 1));
                    }
                    let reg = parts[1].trim_matches(',').to_uppercase();
                    let val_str = parts[2].trim();
                    let val = parse_num(val_str).ok_or_else(|| format!("Linija {}: Nevaljala vrednost '{}'", line_num + 1, val_str))?;

                    match reg.as_str() {
                        "A" => bytecode.push(0x1000 | (val & 0x0FFF)), // Opkod 0x1... za Reg A
                        "B" => bytecode.push(0x2000 | (val & 0x0FFF)), // Opkod 0x2... za Reg B
                        _ => return Err(format!("Linija {}: Nepoznat registar '{}'", line_num + 1, reg)),
                    }
                }

                "ADD" => bytecode.push(0x3000), // ADD A, B
                "SUB" => bytecode.push(0x4000), // SUB A, B

                "STORE" => {
                    if parts.len() < 3 {
                        return Err(format!("Linija {}: STORE traži reg i adresu (npr. STORE A, 0xE000)", line_num + 1));
                    }
                    let addr_str = parts[2].trim();
                    let addr = parse_num(addr_str).ok_or_else(|| format!("Linija {}: Nevaljala adresa '{}'", line_num + 1, addr_str))?;
                    
                    bytecode.push(0x5000); // Opkod za STORE
                    bytecode.push(addr);  // Druga reč je ciljna adresa u RAM-u
                }

                "JMP" => {
                    if parts.len() < 2 {
                        return Err(format!("Linija {}: JMP traži adresu", line_num + 1));
                    }
                    let addr = parse_num(parts[1]).ok_or_else(|| format!("Linija {}: Nevaljala adresa", line_num + 1))?;
                    
                    bytecode.push(0x6000); // Opkod za JMP
                    bytecode.push(addr);
                }

                _ => return Err(format!("Linija {}: Nepoznata instrukcija '{}'", line_num + 1, op)),
            }
        }

        Ok(bytecode)
    }

    /// Crtanje UI-ja za Live Flash Compiler
    pub fn ui(&mut self, ui: &mut egui::Ui, ram: &mut [u16], pc: &mut u16) {
        ui.group(|ui| {
            ui.heading("📝 4^8 Code Editor & Live RAM Compiler");
            ui.label("Pisi asemblerski kôd koji se direktno prevodi u kvatove i ucitava u RAM.");

            ui.add_space(6.0);

            // --- KONTROLNI PANEL ---
            ui.horizontal(|ui| {
                ui.label("Ciljna RAM adresa:");
                let mut addr_hex = format!("0x{:04X}", self.target_addr);
                if ui.add(egui::TextEdit::singleline(&mut addr_hex).desired_width(70.0)).changed() {
                    if let Some(parsed) = parse_num(&addr_hex) {
                        self.target_addr = parsed;
                    }
                }

                ui.checkbox(&mut self.reset_pc_on_flash, "Resetuj PC nakon upisa");

                // --- ⚡ DUGME ZA LIVE FLASH ---
                if ui.button("⚡ FLASH TO RAM").clicked() {
                    match self.compile() {
                        Ok(words) => {
                            let start = self.target_addr as usize;
                            let mut written = 0;

                            for (i, &word) in words.iter().enumerate() {
                                if start + i < ram.len() {
                                    ram[start + i] = word;
                                    written += 1;
                                }
                            }

                            if self.reset_pc_on_flash {
                                *pc = self.target_addr;
                            }

                            self.last_status = Ok(written);
                        }
                        Err(err) => {
                            self.last_status = Err(err);
                        }
                    }
                }
            });

            ui.add_space(6.0);

            // --- ISPRIS STATUS COMPILERA ---
            match &self.last_status {
                Ok(count) if *count > 0 => {
                    ui.colored_label(
                        egui::Color32::GREEN,
                        format!("✅ Uspešno upisano {} reči na adresu 0x{:04X}!", count, self.target_addr),
                    );
                }
                Err(err_msg) => {
                    ui.colored_label(egui::Color32::RED, format!("❌ Greška: {}", err_msg));
                }
                _ => {}
            }

            ui.add_space(6.0);

            // --- TEKSTUALNI EDITOR I BINARNI INSPEKTOR ---
            ui.columns(2, |columns| {
                // Leva kolona: Tekstualni Kôd Editor
                columns[0].vertical(|ui| {
                    ui.label(egui::RichText::new("Asemblerski Kôd:").strong());
                    egui::ScrollArea::vertical().max_height(250.0).show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::multiline(&mut self.source_code)
                                .font(egui::TextStyle::Monospace)
                                .desired_rows(12)
                                .desired_width(f32::INFINITY),
                        );
                    });
                });

                // Desna kolona: Kompajlirani $4^8$ Kvatovi (Preview)
                columns[1].vertical(|ui| {
                    ui.label(egui::RichText::new("Pregled 4^8 Opkodova (Kvatovi):").strong());
                    egui::ScrollArea::vertical().max_height(250.0).show(ui, |ui| {
                        if let Ok(words) = self.compile() {
                            egui::Grid::new("compiler_preview_grid").striped(true).show(ui, |ui| {
                                ui.label("Off");
                                ui.label("Hex");
                                ui.label("8-Quat Representation");
                                ui.end_row();

                                for (i, &w) in words.iter().enumerate() {
                                    ui.monospace(format!("+{:02X}", i));
                                    ui.monospace(format!("0x{:04X}", w));
                                    ui.monospace(Self::to_quat_str(w));
                                    ui.end_row();
                                }
                            });
                        } else {
                            ui.label("Ispravi sintaksu u editoru za pregled kvatova.");
                        }
                    });
                });
            });
        });
    }
}

/// Pomoćna funkcija za parsiranje običnih brojeva i Hex formata (0x...)
fn parse_num(s: &str) -> Option<u16> {
    let s = s.trim();
    if let Some(hex_str) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u16::from_str_radix(hex_str, 16).ok()
    } else {
        s.parse::<u16>().ok()
    }
}