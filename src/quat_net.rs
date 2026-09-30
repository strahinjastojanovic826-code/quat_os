use eframe::egui;
use crate::kernel::{Quat, QuatKernel4x8};

/// Tipovi QuatNet mrežnih paketa / komandi (Header Q0..Q3)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PacketType {
    Ping = 0,             // Q00: Mrežna dijagnostika
    DataTransfer = 1,     // Q01: Prenos podataka
    CourtSubpoena = 2,    // Q02: Zahtev za sudsku zaplenu memorije (Subpoena)
    VoltageOverload = 3,  // Q03: Network Flood na 3.3V
}

/// Strukturisani 8-Kvatni Mrežni Paket
#[derive(Clone, Debug)]
pub struct QuatPacket {
    pub header_type: PacketType,
    pub src_addr: u16,  // 4^4 max range
    pub dst_addr: u16,  // 4^4 max range
    pub payload: u16,   // 4^4 podaci
    pub raw_quat_string: String,
    pub voltage_level: f32,
}

/// QuatNet Base-4 Protocol & Ethernet Simulator
pub struct QuatNetEngine {
    pub connection_state: Quat, // Q0..Q3
    pub local_mac_quat: u16,    // Moja mrežna adresa (npr. 0x00FF)
    pub target_mac_quat: u16,   // Adresa odredišta (npr. 0x03FF - Louisiana Court Server)
    pub packet_queue: Vec<QuatPacket>,
    pub latency_ms: u32,
    pub packet_loss_rate: f32,
    pub total_bytes_sent: u32,
    pub network_logs: Vec<String>,
}

impl Default for QuatNetEngine {
    fn default() -> Self {
        Self {
            connection_state: Quat::Q0,
            local_mac_quat: 0x00FF,  // Base-4 Local Node
            target_mac_quat: 0x03FF, // Remote Node
            packet_queue: Vec::new(),
            latency_ms: 12,
            packet_loss_rate: 0.5,
            total_bytes_sent: 0,
            network_logs: vec![
                "[QUATNET] Base-4 Ethernet / OS Stack Initialized.".to_string(),
                "[NET_PHY] Physical Layer tied to 4^8 Voltage Bus.".to_string(),
            ],
        }
    }
}

impl QuatNetEngine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Glavni UI Render za QuatNet Mrežni Modul
    pub fn ui(&mut self, ui: &mut egui::Ui, kernel: &mut QuatKernel4x8) {
        let q = kernel.reg_a.to_quats();

        // 1. ZAGLAVLJE MREŽNOG STEKA
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.heading("🌐 QuatNet: Base-4 Network Protocol Simulator ($4^8$)");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let (status_text, color) = match self.connection_state {
                        Quat::Q0 => ("Q0: DISCONNECTED (0.0V)", egui::Color32::GRAY),
                        Quat::Q1 => ("Q1: HANDSHAKE SYN (1.1V)", egui::Color32::LIGHT_BLUE),
                        Quat::Q2 => ("Q2: STREAMING DATA (2.2V)", egui::Color32::GREEN),
                        Quat::Q3 => ("Q3: SUBPOENA FLOOD ATTACK (3.3V MAX)", egui::Color32::LIGHT_RED),
                    };
                    ui.label(
                        egui::RichText::new(status_text)
                            .strong()
                            .color(egui::Color32::BLACK)
                            .background_color(color),
                    );
                });
            });

            ui.label(
                egui::RichText::new(format!(
                    "Lokalna MAC Adresa: 0x{:04X} | Odredište: 0x{:04X} | Odziv (Latency): {}ms | Gubitak Paketa: {:.1}%",
                    self.local_mac_quat, self.target_mac_quat, self.latency_ms, self.packet_loss_rate
                ))
                .font(egui::FontId::proportional(13.0))
                .color(egui::Color32::LIGHT_GREEN),
            );
        });

        ui.separator();
        ui.add_space(4.0);

        // 2. KONTROLNI PANELI I GENERATOR PAKETA
        ui.columns(2, |cols| {
            // LEVI PANEL: SLANJE PAKETA I MREŽNE KOMANDE
            cols[0].group(|ui| {
                ui.label(egui::RichText::new("📡 GENERATOR $8$-KVATNIH PAKETA").strong());
                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Target Node Address:");
                    ui.add(egui::DragValue::new(&mut self.target_mac_quat).hexadecimal(4, false, true));
                });

                ui.add_space(6.0);

                if ui.button("📡 Pošalji Q0 PING Paket (0.0V Diagnostics)").clicked() {
                    self.send_packet(PacketType::Ping, 0x1234, kernel);
                }

                ui.add_space(4.0);

                if ui.button("📦 Prenesi Registar A kroz Mrežu (Q1 Data)").clicked() {
                    self.send_packet(PacketType::DataTransfer, kernel.reg_a.0, kernel);
                }

                ui.add_space(4.0);

                if ui.button("📜 Pošalji Lujzijana Subpoena Paket (Q2 Legal Packet)").clicked() {
                    self.send_packet(PacketType::CourtSubpoena, 40000, kernel);
                }

                ui.add_space(4.0);

                if ui.button("💥 POKRENI SUBPOENA FLOOD ATTACK (Q3 3.3V Overload)").clicked() {
                    self.connection_state = Quat::Q3;
                    kernel.drivers.set_audio_state(Quat::Q3);
                    for _ in 0..5 {
                        self.send_packet(PacketType::VoltageOverload, 0xFFFF, kernel);
                    }
                }
            });

            // DESNI PANEL: STANJE MREŽNE FIZIKE I BAFERA
            cols[1].group(|ui| {
                ui.label(egui::RichText::new("⚙️ KANALI I STATUS MREŽNOG DRAVERA").strong());
                ui.separator();

                ui.label(format!("Ukupno poslato: {} Bajtova ({}) Quat-Word-a", self.total_bytes_sent, self.total_bytes_sent / 2));
                ui.label(format!("Napon Mrežnog Čipa: {:.1}V", (q[0] as f32 + 1.0) * 0.825));

                ui.add_space(8.0);
                ui.label("Upravljanje Mrežnom Vezom:");
                ui.horizontal(|ui| {
                    if ui.button("🔌 Connect (Handshake)").clicked() {
                        self.connection_state = Quat::Q1;
                        self.network_logs.push("[NET] 3-Way Base-4 Handshake Initiated...".to_string());
                    }
                    if ui.button("❌ Disconnect").clicked() {
                        self.connection_state = Quat::Q0;
                        self.network_logs.push("[NET] Socket closed. Bus idle at 0.0V.".to_string());
                    }
                });

                ui.add_space(8.0);
                if ui.button("🗑️ Očisti Red Paketa").clicked() {
                    self.packet_queue.clear();
                    self.network_logs.push("[NET] Packet Queue Flushed.".to_string());
                }
            });
        });

        ui.add_space(8.0);

        // 3. TABELA POSLATIH PAKETA UNUTAR CIKLUSA
        ui.group(|ui| {
            ui.label(egui::RichText::new("📦 ACTIVE PACKET QUEUE & WIRE FRAME INSPECTOR").strong());
            ui.separator();

            egui::ScrollArea::vertical().max_height(120.0).show(ui, |ui| {
                egui::Grid::new("quatnet_packet_grid")
                    .striped(true)
                    .min_col_width(90.0)
                    .show(ui, |ui| {
                        ui.strong("Tip Paketa");
                        ui.strong("SRC Adresa");
                        ui.strong("DST Adresa");
                        ui.strong("Payload");
                        ui.strong("8-Kvatni Mrežni Kod");
                        ui.strong("Napon");
                        ui.end_row();

                        for pkt in self.packet_queue.iter().rev() {
                            ui.label(format!("{:?}", pkt.header_type));
                            ui.monospace(format!("0x{:04X}", pkt.src_addr));
                            ui.monospace(format!("0x{:04X}", pkt.dst_addr));
                            ui.monospace(format!("0x{:04X}", pkt.payload));
                            ui.monospace(egui::RichText::new(&pkt.raw_quat_string).strong());
                            ui.label(format!("{:.1}V", pkt.voltage_level));
                            ui.end_row();
                        }
                    });
            });
        });

        ui.add_space(8.0);

        // 4. MREŽNI DNEVNIK (NETWORK LOGS)
        ui.group(|ui| {
            ui.label(egui::RichText::new("📜 NETWORKING DIAGNOSTIC CONSOLE").strong());
            ui.separator();
            egui::ScrollArea::vertical().max_height(100.0).show(ui, |ui| {
                for log in self.network_logs.iter().rev() {
                    let color = if log.contains("FLOOD") || log.contains("OVERLOAD") {
                        egui::Color32::LIGHT_RED
                    } else if log.contains("SUBPOENA") {
                        egui::Color32::GOLD
                    } else {
                        egui::Color32::LIGHT_GREEN
                    };

                    ui.monospace(egui::RichText::new(log).color(color));
                }
            });
        });
    }

    /// Funkcija za konstrukciju i slanje 8-kvatnog mrežnog paketa
    pub fn send_packet(&mut self, pkt_type: PacketType, payload_val: u16, kernel: &mut QuatKernel4x8) {
        let volts = match pkt_type {
            PacketType::Ping => 0.0,
            PacketType::DataTransfer => 1.1,
            PacketType::CourtSubpoena => 2.2,
            PacketType::VoltageOverload => 3.3,
        };

        // Generisanje 8-kvatnog mašinskog formata paketa
        let raw_quat = format!(
            "Q{:02b}SRC{:02x}DST{:02x}PL{:02x}",
            pkt_type as u8,
            (self.local_mac_quat & 0xFF) as u8,
            (self.target_mac_quat & 0xFF) as u8,
            (payload_val & 0xFF) as u8
        );

        let packet = QuatPacket {
            header_type: pkt_type,
            src_addr: self.local_mac_quat,
            dst_addr: self.target_mac_quat,
            payload: payload_val,
            raw_quat_string: raw_quat.clone(),
            voltage_level: volts,
        };

        self.packet_queue.push(packet);
        self.total_bytes_sent += 2; // 8 quats = 16 bits = 2 bytes
        kernel.step(); // Okida rad procesora

        self.network_logs.push(format!(
            "[TX] Sent {:?} Packet to 0x{:04X} | Raw: {} | {:.1}V",
            pkt_type, self.target_mac_quat, raw_quat, volts
        ));
    }
}