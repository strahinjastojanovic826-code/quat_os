use std::collections::VecDeque;

#[derive(Clone, Copy, Debug)]
pub struct BusFrame {
    pub cycle: u64,
    pub addr: u16,     // 8 kvatova adrese
    pub data: u16,     // 8 kvatova podataka
    pub control: u8,   // 0=IDLE, 1=READ, 2=WRITE, 3=EXEC
}

pub struct BusAnalyzer {
    pub history: VecDeque<BusFrame>,
    pub max_history: usize,
    pub current_frame: BusFrame,
    pub total_cycles: u64,
}

impl BusAnalyzer {
    pub fn new(max_history: usize) -> Self {
        Self {
            history: VecDeque::with_capacity(max_history),
            max_history,
            current_frame: BusFrame {
                cycle: 0,
                addr: 0,
                data: 0,
                control: 0,
            },
            total_cycles: 0,
        }
    }

    /// Zabeleži novi ciklus na magistrali (Poziva se kad god procesor uradi korak ili pristup RAM-u)
    pub fn record(&mut self, addr: u16, data: u16, control: u8) {
        self.total_cycles += 1;
        
        let frame = BusFrame {
            cycle: self.total_cycles,
            addr,
            data,
            control: control & 0b11,
        };

        self.current_frame = frame;

        if self.history.len() >= self.max_history {
            self.history.pop_front();
        }
        self.history.push_back(frame);
    }

    /// Pomoćna funkcija za izvlačenje 8 kvatova iz 16-bitne reči
    fn to_quats(val: u16) -> [u8; 8] {
        let mut quats = [0u8; 8];
        for i in 0..8 {
            quats[i] = ((val >> (i * 2)) & 0b11) as u8;
        }
        quats
    }

    /// Vraća odgovarajuću egui boju za naponsko stanje kvata (Q0-Q3)
    fn quat_color(quat: u8) -> egui::Color32 {
        match quat {
            0 => egui::Color32::from_rgb(40, 40, 40),    // Q0 (0.0V) - Siva
            1 => egui::Color32::from_rgb(0, 150, 255),   // Q1 (1.1V) - Plava
            2 => egui::Color32::from_rgb(240, 200, 0),   // Q2 (2.2V) - Žuta
            3 => egui::Color32::from_rgb(255, 60, 60),   // Q3 (3.3V) - Crvena
            _ => egui::Color32::BLACK,
        }
    }

    /// Crtanje UI interfejsa Logičkog Analizatora
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.heading("🔬 4^8 Bus Signal Analyzer (Logički Analizator Magistrale)");
            ui.label(format!("Ukupno magistralnih ciklusa: {}", self.total_cycles));

            ui.add_space(8.0);

            // --- 1. TRENUTNI SIGNALI NA MAGISTRALI (UŽIVO) ---
            ui.group(|ui| {
                ui.label(egui::RichText::new("⚡ Trenutno Stanje Linija (Live Hardware State)").strong());
                ui.add_space(4.0);

                // --- UPRAVLJAČKA MAGISTRALA (CONTROL BUS) ---
                ui.horizontal(|ui| {
                    ui.label("Control Bus (1 Q):");
                    let (ctrl_text, ctrl_color) = match self.current_frame.control {
                        1 => (" [Q1] READ (Čitanje) ", egui::Color32::from_rgb(0, 150, 255)),
                        2 => (" [Q2] WRITE (Upis) ", egui::Color32::from_rgb(240, 200, 0)),
                        3 => (" [Q3] EXEC / INT (Izvršavanje) ", egui::Color32::from_rgb(255, 60, 60)),
                        _ => (" [Q0] IDLE (Mirovanje) ", egui::Color32::GRAY),
                    };
                    ui.colored_label(ctrl_color, egui::RichText::new(ctrl_text).strong());
                });

                ui.add_space(4.0);

                // --- ADRESNA MAGISTRALA (ADDRESS BUS - 8 KVATOVA) ---
                let addr_quats = Self::to_quats(self.current_frame.addr);
                ui.horizontal(|ui| {
                    ui.monospace(format!("Address Bus (0x{:04X}): ", self.current_frame.addr));
                    for (i, &q) in addr_quats.iter().enumerate().rev() {
                        let (rect, response) = ui.allocate_exact_size(egui::vec2(22.0, 20.0), egui::Sense::hover());
                        ui.painter().rect_filled(rect, 3.0, Self::quat_color(q));
                        ui.painter().text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            format!("Q{}", q),
                            egui::FontId::monospace(10.0),
                            egui::Color32::WHITE,
                        );
                        response.on_hover_text(format!("Kvat {}: Q{} ({:.1}V)", i, q, q as f32 * 1.1));
                    }
                });

                ui.add_space(4.0);

                // --- MAGISTRALA PODATAKA (DATA BUS - 8 KVATOVA) ---
                let data_quats = Self::to_quats(self.current_frame.data);
                ui.horizontal(|ui| {
                    ui.monospace(format!("Data Bus    (0x{:04X}): ", self.current_frame.data));
                    for (i, &q) in data_quats.iter().enumerate().rev() {
                        let (rect, response) = ui.allocate_exact_size(egui::vec2(22.0, 20.0), egui::Sense::hover());
                        ui.painter().rect_filled(rect, 3.0, Self::quat_color(q));
                        ui.painter().text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            format!("Q{}", q),
                            egui::FontId::monospace(10.0),
                            egui::Color32::WHITE,
                        );
                        response.on_hover_text(format!("Kvat {}: Q{} ({:.1}V)", i, q, q as f32 * 1.1));
                    }
                });
            });

            ui.add_space(8.0);

            // --- 2. ISTORIJA CIKLUSA (LOGIC ANALYZER TRACE TABLE) ---
            ui.label(egui::RichText::new("📜 Istorija Signala (Signal Trace)").strong());
            
            egui::ScrollArea::vertical().max_height(180.0).show(ui, |ui| {
                egui::Grid::new("bus_trace_grid").striped(true).spacing([12.0, 4.0]).show(ui, |ui| {
                    ui.label("Ciklus");
                    ui.label("Tip Operacije");
                    ui.label("Adresa (Hex)");
                    ui.label("Podatak (Hex)");
                    ui.label("Kvat Prikaz (Q7..Q0)");
                    ui.end_row();

                    for frame in self.history.iter().rev() {
                        ui.monospace(format!("#{}", frame.cycle));

                        let op_str = match frame.control {
                            1 => "READ",
                            2 => "WRITE",
                            3 => "EXEC",
                            _ => "IDLE",
                        };
                        ui.monospace(op_str);

                        ui.monospace(format!("0x{:04X}", frame.addr));
                        ui.monospace(format!("0x{:04X}", frame.data));

                        // 8-kvatni tekstualni prikaz
                        let q = Self::to_quats(frame.data);
                        ui.monospace(format!("Q{}Q{}Q{}Q{} Q{}Q{}Q{}", q[7], q[6], q[5], q[4], q[3], q[2], q[1]));

                        ui.end_row();
                    }
                });
            });
        });
    }
}