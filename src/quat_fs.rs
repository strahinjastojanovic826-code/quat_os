use eframe::egui;
use crate::kernel::{Quat, QuatKernel4x8};

/// Fizički status fajla u zavisnosti od napona
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileVoltageState {
    ReadOnly0V = 0,      // Q0: 0.0V - Zaštićeno od pisanja
    Normal11V = 1,       // Q1: 1.1V - Običan fajl (Read/Write)
    Encrypted22V = 2,    // Q2: 2.2V - Kriptovan Quat-XOR algoritmom
    SystemLocked33V = 3, // Q3: 3.3V - Sistemski fajl jezgra / Zabrana pristupa
}

/// Virtuelni fajl unutar QuatFS-a ($4^8$ adresni prostor)
#[derive(Clone, Debug)]
pub struct QuatFile {
    pub name: String,
    pub start_addr: u16, // 0x0000..0xFFFF
    pub size: u16,       // Veličina u bajtovima
    pub state: Quat,     // Q0..Q3
    pub content: Vec<u8>,
}

/// QuatFS: $4^8$ Virtual File System & Hex Editor Engine
pub struct QuatFSEngine {
    pub memory: Vec<u8>,           // 65536 bajtova (4^8)
    pub files: Vec<QuatFile>,
    pub selected_file_idx: Option<usize>,
    pub hex_view_offset: u16,      // Početak prikazane stranice u Hex Editoru
    pub hex_edit_addr: u16,        // Adresa koja se trenutno menja
    pub input_hex_val: String,     // Unos nove hex vrednosti
    pub cli_input: String,         // Unos komande u QuatShell terminalu
    pub cli_logs: Vec<String>,     // Istorija terminala
    pub new_file_name: String,     // Naziv za kreiranje fajla
    pub new_file_content: String,  // Sadržaj novog fajla
}

impl Default for QuatFSEngine {
    fn default() -> Self {
        let mut mem = vec![0u8; 65536];
        
        // Pred-popunjavanje nekih sistemskih sektora sa Quat kodom
        let sample_text = b"QUAT_FS_V1.0_KERNEL_BASE4_BOOT_SECTOR";
        for (i, &b) in sample_text.iter().enumerate() {
            mem[i] = b;
        }

        // Podrazumevani fajlovi u sistemu
        let files = vec![
            QuatFile {
                name: "boot_sector.sys".to_string(),
                start_addr: 0x0000,
                size: sample_text.len() as u16,
                state: Quat::Q3, // System Locked (3.3V)
                content: sample_text.to_vec(),
            },
            QuatFile {
                name: "sojic_promena_ustava.q4".to_string(),
                start_addr: 0x0100,
                size: 30,
                state: Quat::Q1, // Normal (1.1V)
                content: b"Zahtevam direktan prenos u 4K!".to_vec(),
            },
            QuatFile {
                name: "subpoena_louisiana.key".to_string(),
                start_addr: 0x0200,
                size: 31,
                state: Quat::Q2, // Encrypted (2.2V)
                content: b"RESTRICTED_COURT_DOCUMENT_Q2222".to_vec(),
            },
        ];

        Self {
            memory: mem,
            files,
            selected_file_idx: Some(0),
            hex_view_offset: 0x0000,
            hex_edit_addr: 0x0000,
            input_hex_val: "00".to_string(),
            cli_input: String::new(),
            cli_logs: vec![
                "[QuatFS] $4^8$ Virtual File System initialized (65,536 Bytes / Quat-Words).".to_string(),
                "[QuatFS] QuatFAT table mounted. Address range: 0x0000 (Q00000000) - 0xFFFF (Q33333333).".to_string(),
                "Type 'help' for available QuatShell commands.".to_string(),
            ],
            new_file_name: "pantic_zalba.txt".to_string(),
            new_file_content: "Strogo poverljivo za direktora!".to_string(),
        }
    }
}

impl QuatFSEngine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Konvertuje 8-bitni bajt u 4 Quat znaka (baza 4: 0, 1, 2, 3)
    pub fn byte_to_quat_str(val: u8) -> String {
        let q3 = (val >> 6) & 3;
        let q2 = (val >> 4) & 3;
        let q1 = (val >> 2) & 3;
        let q0 = val & 3;
        format!("Q{}{}{}{}", q3, q2, q1, q0)
    }

    /// Konvertuje u16 adresu u 8 Quat reprezentaciju ($4^8$)
    pub fn addr_to_quat_8_str(addr: u16) -> String {
        let mut q_str = String::with_capacity(8);
        for shift in (0..8).rev() {
            let digit = (addr >> (shift * 2)) & 0x03;
            q_str.push_str(&digit.to_string());
        }
        format!("Q{}", q_str)
    }

    /// Glavni UI Render za QuatFS i Hex Editor
    pub fn ui(&mut self, ui: &mut egui::Ui, kernel: &mut QuatKernel4x8) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.heading("💾 QuatFS: $4^8$ Virtual File System & Hex Memory Inspector");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let used_bytes: usize = self.files.iter().map(|f| f.size as usize).sum();
                    ui.label(
                        egui::RichText::new(format!("Kapacitet: {} / 65536 B ({:.1}% popunjeno)", used_bytes, (used_bytes as f32 / 65536.0) * 100.0))
                            .strong()
                            .color(egui::Color32::LIGHT_BLUE),
                    );
                });
            });

            ui.label(
                egui::RichText::new("Adresni prostor: 0x0000 - 0xFFFF | Quat Opseg: Q00000000 do Q33333333 | 4 Naponska Nivoa (0.0V - 3.3V)")
                    .font(egui::FontId::proportional(12.0))
                    .color(egui::Color32::GRAY),
            );
        });

        ui.separator();
        ui.add_space(4.0);

        // GLAVNI RASPORED: LEVO FILE SYSTEM & CLI, DESNO HEX EDITOR
        ui.columns(2, |cols| {
            // LEVI PANEL: FAJLOVI + SHELL TERMINAL
            cols[0].vertical(|ui| {
                ui.group(|ui| {
                    ui.label(egui::RichText::new("📂 QuatFAT Virtuelni Fajlovi").strong());
                    ui.separator();

                    egui::ScrollArea::vertical().max_height(140.0).show(ui, |ui| {
                        egui::Grid::new("quatfs_files_grid")
                            .striped(true)
                            .min_col_width(75.0)
                            .show(ui, |ui| {
                                ui.strong("Naziv");
                                ui.strong("Adresa");
                                ui.strong("Veličina");
                                ui.strong("Napon / Status");
                                ui.strong("Akcija");
                                ui.end_row();

                                for (idx, file) in self.files.iter().enumerate() {
                                    let is_selected = self.selected_file_idx == Some(idx);
                                    let (status_str, status_color) = match file.state {
                                        Quat::Q0 => ("Q0 (0.0V RO)", egui::Color32::GRAY),
                                        Quat::Q1 => ("Q1 (1.1V RW)", egui::Color32::GREEN),
                                        Quat::Q2 => ("Q2 (2.2V ENC)", egui::Color32::GOLD),
                                        Quat::Q3 => ("Q3 (3.3V LOCK)", egui::Color32::LIGHT_RED),
                                    };

                                    if ui.selectable_label(is_selected, &file.name).clicked() {
                                        self.selected_file_idx = Some(idx);
                                        self.hex_view_offset = file.start_addr;
                                    }
                                    ui.monospace(format!("0x{:04X}", file.start_addr));
                                    ui.label(format!("{} B", file.size));
                                    ui.label(egui::RichText::new(status_str).color(status_color));

                                    if ui.button("Inspect").clicked() {
                                        self.hex_view_offset = file.start_addr;
                                        self.hex_edit_addr = file.start_addr;
                                    }
                                    ui.end_row();
                                }
                            });
                    });
                });

                ui.add_space(6.0);

                // FORMULAR ZA KREIRANJE NOVOG FAJLA
                ui.group(|ui| {
                    ui.label(egui::RichText::new("➕ Kreiraj Novi QuatFS Fajl").strong());
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Ime:");
                        ui.text_edit_singleline(&mut self.new_file_name);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Sadržaj:");
                        ui.text_edit_singleline(&mut self.new_file_content);
                    });
                    ui.horizontal(|ui| {
                        if ui.button("💾 Zapiši u QuatFS").clicked() {
                            self.create_file_from_input();
                        }
                        if ui.button("🧹 Formatiraj FS").clicked() {
                            self.memory.fill(0);
                            self.files.clear();
                            self.cli_logs.push("[QUATFS] File system formatted. All 65,536 bytes zeroed out.".to_string());
                        }
                    });
                });

                ui.add_space(6.0);

                // TERMINAL INTERFEJS (QuatShell)
                ui.group(|ui| {
                    ui.label(egui::RichText::new("💻 QuatShell Terminal (CLI)").strong());
                    ui.separator();

                    egui::ScrollArea::vertical().max_height(100.0).show(ui, |ui| {
                        for log in self.cli_logs.iter().rev() {
                            ui.monospace(egui::RichText::new(log).font(egui::FontId::monospace(11.0)).color(egui::Color32::LIGHT_GREEN));
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label(">");
                        let response = ui.text_edit_singleline(&mut self.cli_input);
                        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            self.execute_cli_command(kernel);
                        }
                    });
                });
            });

            // DESNI PANEL: QUAT-HEX & VOLTAGE EDITOR
            cols[1].vertical(|ui| {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("🔍 4^8 Hex & Quat Voltage Memory Editor").strong());
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.monospace(format!("Stranica: 0x{:04X}", self.hex_view_offset));
                        });
                    });
                    ui.separator();

                    // KONTROLE NAVIGACIJE KROZ MEMORIJU
                    ui.horizontal(|ui| {
                        if ui.button("⏮️ -256B").clicked() {
                            self.hex_view_offset = self.hex_view_offset.saturating_sub(256);
                        }
                        if ui.button("◀️ -16B").clicked() {
                            self.hex_view_offset = self.hex_view_offset.saturating_sub(16);
                        }
                        if ui.button("▶️ +16B").clicked() {
                            if self.hex_view_offset as usize + 16 < 65536 {
                                self.hex_view_offset += 16;
                            }
                        }
                        if ui.button("⏭️ +256B").clicked() {
                            if self.hex_view_offset as usize + 256 < 65536 {
                                self.hex_view_offset += 256;
                            }
                        }
                        ui.label("Skok:");
                        ui.add(egui::DragValue::new(&mut self.hex_view_offset).hexadecimal(4, false, true));
                    });

                    ui.add_space(6.0);

                    // PRIKAZ MEMORIJSKE MREŽE (16 Bajtova po redu)
                    egui::ScrollArea::vertical().max_height(220.0).show(ui, |ui| {
                        egui::Grid::new("quat_hex_grid")
                            .striped(true)
                            .min_col_width(26.0)
                            .show(ui, |ui| {
                                ui.strong("Adresa (Hex/Quat)");
                                for col in 0..16 {
                                    ui.strong(format!("{:X}", col));
                                }
                                ui.strong("ASCII");
                                ui.end_row();

                                let start_row = self.hex_view_offset as usize;
                                let end_row = (start_row + 160).min(65536); // Prikazujemo 10 redova (160 B)

                                for row_addr in (start_row..end_row).step_by(16) {
                                    let q_addr_str = Self::addr_to_quat_8_str(row_addr as u16);
                                    ui.monospace(egui::RichText::new(format!("0x{:04X}\n{}", row_addr, q_addr_str)).font(egui::FontId::monospace(9.0)));

                                    let mut ascii_repr = String::new();

                                    for col in 0..16 {
                                        let addr = row_addr + col;
                                        if addr < 65536 {
                                            let val = self.memory[addr];
                                            let q_code = Self::byte_to_quat_str(val);
                                            let ch = if val >= 32 && val <= 126 { val as char } else { '.' };
                                            ascii_repr.push(ch);

                                            // Bojenje napona u zavisnosti od prva dva kvata bajta
                                            let top_quat = (val >> 6) & 3;
                                            let cell_color = match top_quat {
                                                0 => egui::Color32::LIGHT_GRAY,
                                                1 => egui::Color32::LIGHT_GREEN,
                                                2 => egui::Color32::GOLD,
                                                _ => egui::Color32::LIGHT_RED,
                                            };

                                            let is_selected = self.hex_edit_addr == addr as u16;
                                            let btn_text = format!("{:02X}\n{}", val, q_code);

                                            if ui.add(egui::Button::new(
                                                egui::RichText::new(btn_text)
                                                    .font(egui::FontId::monospace(9.0))
                                                    .color(if is_selected { egui::Color32::WHITE } else { cell_color })
                                                    .background_color(if is_selected { egui::Color32::BLUE } else { egui::Color32::TRANSPARENT })
                                            )).clicked() {
                                                self.hex_edit_addr = addr as u16;
                                                self.input_hex_val = format!("{:02X}", val);
                                            }
                                        }
                                    }

                                    ui.monospace(egui::RichText::new(ascii_repr).color(egui::Color32::YELLOW));
                                    ui.end_row();
                                }
                            });
                    });

                    ui.separator();

                    // PANEL ZA DIREKTNU IZMENU BAJTA
                    ui.horizontal(|ui| {
                        ui.label(format!("Izmena Bajta na 0x{:04X} ({}):", self.hex_edit_addr, Self::addr_to_quat_8_str(self.hex_edit_addr)));
                        ui.text_edit_singleline(&mut self.input_hex_val);
                        if ui.button("✏️ Upisi u Memoriju").clicked() {
                            if let Ok(val) = u8::from_str_radix(self.input_hex_val.trim(), 16) {
                                self.memory[self.hex_edit_addr as usize] = val;
                                // Sinhronizuj sa Kernel Registrom A ako je na početnoj adresi
                                if self.hex_edit_addr == 0x0000 {
                                    kernel.reg_a.0 = val as u16;
                                }
                                self.cli_logs.push(format!("[HEX] Address 0x{:04X} set to 0x{:02X} ({})", self.hex_edit_addr, val, Self::byte_to_quat_str(val)));
                            }
                        }
                    });
                });
            });
        });
    }

    /// Pomoćna funkcija za kreiranje fajla i pisanje u QuatFS memoriju
    fn create_file_from_input(&mut self) {
        if self.new_file_name.is_empty() { return; }

        let bytes = self.new_file_content.as_bytes();
        let size = bytes.len() as u16;

        // Pronađi prvu slobodnu adresu iza poslednjeg fajla
        let start_addr = self.files.iter().map(|f| f.start_addr + f.size).max().unwrap_or(0x0300);

        if start_addr as usize + size as usize >= 65536 {
            self.cli_logs.push("[QUATFS ERROR] Storage full! Unable to write file.".to_string());
            return;
        }

        // Pisanje u simuliranu RAM memoriju
        for (i, &b) in bytes.iter().enumerate() {
            self.memory[start_addr as usize + i] = b;
        }

        let new_file = QuatFile {
            name: self.new_file_name.clone(),
            start_addr,
            size,
            state: Quat::Q1,
            content: bytes.to_vec(),
        };

        self.files.push(new_file);
        self.cli_logs.push(format!("[QUATFS] File '{}' saved at 0x{:04X} ({} bytes).", self.new_file_name, start_addr, size));
        self.hex_view_offset = start_addr;
        self.new_file_name.clear();
        self.new_file_content.clear();
    }

    /// Interpretator QuatShell Terminal Komandi
    fn execute_cli_command(&mut self, kernel: &mut QuatKernel4x8) {
        let cmd = self.cli_input.trim().to_string();
        if cmd.is_empty() { return; }

        self.cli_logs.push(format!("> {}", cmd));
        let parts: Vec<&str> = cmd.split_whitespace().collect();

        match parts[0] {
            "help" => {
                self.cli_logs.push("Dostupne komande: ls, cat <file>, rega, clear".to_string());
            }
            "ls" => {
                for f in &self.files {
                    self.cli_logs.push(format!("- {} | 0x{:04X} | {} B | {:?}", f.name, f.start_addr, f.size, f.state));
                }
            }
            "cat" => {
                if parts.len() > 1 {
                    if let Some(f) = self.files.iter().find(|x| x.name == parts[1]) {
                        let text = String::from_utf8_lossy(&f.content);
                        self.cli_logs.push(format!("Content: {}", text));
                    } else {
                        self.cli_logs.push(format!("File '{}' not found.", parts[1]));
                    }
                } else {
                    self.cli_logs.push("Usage: cat <file_name>".to_string());
                }
            }
            "rega" => {
                self.cli_logs.push(format!("Reg-A Value: 0x{:04X} | Quat: {:?}", kernel.reg_a.0, kernel.reg_a.to_quats()));
            }
            "clear" => {
                self.cli_logs.clear();
            }
            _ => {
                self.cli_logs.push(format!("Unknown command: '{}'. Type 'help' for info.", parts[0]));
            }
        }

        self.cli_input.clear();
    }
}