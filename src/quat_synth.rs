use eframe::egui;
use crate::kernel::QuatKernel4x8;

/// Tip talasa na osnovu kvatnog naponskog nivoa
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QuatWaveform {
    DcFloor0V = 0,       // Q0: 0.0V - Pulse Baseline
    QuatPulse11V = 1,    // Q1: 1.1V - Pulse Wave (25% Duty)
    StaircaseSaw22V = 2, // Q2: 2.2V - Четворостепена степениasta pila
    QuatNoise33V = 3,    // Q3: 3.3V - Pseudorandom Quat Šum
}

/// Jedan glas/kanal sintetizatora
#[derive(Clone, Debug)]
pub struct QuatVoice {
    pub name: String,
    pub enabled: bool,
    pub freq_hz: f32,       // Frekvencija (Hz)
    pub wave_type: QuatWaveform,
    pub volume: f32,        // 0.0 do 1.0
    pub phase: f32,         // Trenutna faza oscilatora
}

/// Custom 4^8 Chiptune Synth & Audio DSP Subsystem
pub struct QuatSynthEngine {
    pub voices: Vec<QuatVoice>,
    pub master_volume: f32,
    pub quantizer_enabled: bool, // DSP Kvantizacija na 4 napona
    pub bpm: u16,
    pub current_step: usize,
    pub is_playing: bool,
    pub sequencer_grid: [[u8; 16]; 4], // 4 kanala x 16 koraka (Vrednosti 0..3 predstavlja Q0..Q3)
    pub wave_buffer: Vec<f32>,         // Bafer za prikaz na osciloskopu
    pub synth_logs: Vec<String>,
}

impl Default for QuatSynthEngine {
    fn default() -> Self {
        let voices = vec![
            QuatVoice { name: "CH0: Lead Pulse".into(), enabled: true, freq_hz: 440.0, wave_type: QuatWaveform::QuatPulse11V, volume: 0.8, phase: 0.0 },
            QuatVoice { name: "CH1: Bass Saw".into(), enabled: true, freq_hz: 110.0, wave_type: QuatWaveform::StaircaseSaw22V, volume: 0.9, phase: 0.0 },
            QuatVoice { name: "CH2: Arp Step".into(), enabled: false, freq_hz: 880.0, wave_type: QuatWaveform::QuatPulse11V, volume: 0.5, phase: 0.0 },
            QuatVoice { name: "CH3: Noise Perc".into(), enabled: true, freq_hz: 220.0, wave_type: QuatWaveform::QuatNoise33V, volume: 0.6, phase: 0.0 },
        ];

        // Podrazumevana sekvenca za retro melodiju u 16 koraka
        let mut grid = [[0u8; 16]; 4];
        grid[0] = [1, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 2, 1, 0]; // Lead
        grid[1] = [2, 2, 0, 0, 2, 2, 0, 0, 2, 2, 0, 0, 3, 3, 0, 0]; // Bass
        grid[3] = [0, 3, 0, 3, 0, 3, 0, 3, 0, 3, 0, 3, 3, 3, 3, 3]; // Drums

        Self {
            voices,
            master_volume: 0.75,
            quantizer_enabled: true,
            bpm: 128,
            current_step: 0,
            is_playing: true,
            sequencer_grid: grid,
            wave_buffer: vec![0.0; 256],
            synth_logs: vec![
                "[QuatAudio] $4^8$ DSP Audio Engine initialized.".to_string(),
                "[QuatAudio] DAC Output locked to 4 Discrete Voltages (0.0V, 1.1V, 2.2V, 3.3V).".to_string(),
                "[Sequencer] 16-Step Quat Tracker ready.".to_string(),
            ],
        }
    }
}

impl QuatSynthEngine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Generiše sledeći sempl napona u zavisnosti od oblika talasa i kvatne kvantizacije
    pub fn generate_sample(&mut self, kernel_reg_a: u16) -> f32 {
        let mut mixed_sample = 0.0;
        let mut active_count = 0.0;

        for (v_idx, voice) in self.voices.iter_mut().enumerate() {
            if !voice.enabled { continue; }
            active_count += 1.0;

            // Pomeranje faze
            let step_add = voice.freq_hz / 44100.0;
            voice.phase = (voice.phase + step_add) % 1.0;

            // Generisanje napona na osnovu kvatnog oblika talasa
            let raw_voltage = match voice.wave_type {
                QuatWaveform::DcFloor0V => 0.0,
                QuatWaveform::QuatPulse11V => {
                    if voice.phase < 0.25 { 3.3 } else if voice.phase < 0.50 { 1.1 } else { 0.0 }
                }
                QuatWaveform::StaircaseSaw22V => {
                    // Četvorostepena kvatna "pila"
                    if voice.phase < 0.25 { 0.0 }
                    else if voice.phase < 0.50 { 1.1 }
                    else if voice.phase < 0.75 { 2.2 }
                    else { 3.3 }
                }
                QuatWaveform::QuatNoise33V => {
                    // Generisanje šuma iz stanja procesorskog registra A
                    let seed = (kernel_reg_a as usize + (voice.phase * 1000.0) as usize + v_idx) % 4;
                    match seed {
                        0 => 0.0,
                        1 => 1.1,
                        2 => 2.2,
                        _ => 3.3,
                    }
                }
            };

            mixed_sample += raw_voltage * voice.volume;
        }

        if active_count > 0.0 {
            mixed_sample /= active_count;
        }

        // DSP QUAT-QUANTIZER: Svodi proizvoljan napon isključivo na najbliži od 4 kvatna napona
        if self.quantizer_enabled {
            mixed_sample = if mixed_sample < 0.55 {
                0.0
            } else if mixed_sample < 1.65 {
                1.1
            } else if mixed_sample < 2.75 {
                2.2
            } else {
                3.3
            };
        }

        mixed_sample * self.master_volume
    }

    /// Ažurira bafer za osciloskop
    pub fn update_dsp_buffer(&mut self, kernel: &QuatKernel4x8) {
        let sample = self.generate_sample(kernel.reg_a.0);
        self.wave_buffer.remove(0);
        self.wave_buffer.push(sample);

        // Simulacija koraka u sekvenceru
        if self.is_playing {
            self.current_step = (self.current_step + 1) % 16;
            
            // Ažuriraj frekvencije na osnovu sekvencera
            for ch in 0..4 {
                let q_val = self.sequencer_grid[ch][self.current_step];
                let base_freq = match ch {
                    0 => 220.0,
                    1 => 110.0,
                    2 => 440.0,
                    _ => 330.0,
                };
                self.voices[ch].freq_hz = base_freq * (1.0 + q_val as f32 * 0.5);
            }
        }
    }

    /// UI Render za QuatSynth i Audio DSP
    pub fn ui(&mut self, ui: &mut egui::Ui, kernel: &mut QuatKernel4x8) {
        self.update_dsp_buffer(kernel);

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.heading("🎵 Custom 4^8 Chiptune Synth & Audio DSP");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new(format!("MASTER VOL: {:.0}% | DSP: 4-STAGE DAC", self.master_volume * 100.0))
                            .strong()
                            .color(egui::Color32::LIGHT_BLUE),
                    );
                });
            });

            ui.label(
                egui::RichText::new("Zvučni čip baziran na 4 naponska nivoa ($0.0\\text{V} - 3.3\\text{V}$) sa kvatnom kvantizacijom talasa")
                    .font(egui::FontId::proportional(11.0))
                    .color(egui::Color32::GRAY),
            );
        });

        ui.separator();
        ui.add_space(4.0);

        ui.columns(2, |cols| {
            // LEVI PANEL: KANALI OSCILATORA & KONTROLE
            cols[0].vertical(|ui| {
                ui.group(|ui| {
                    ui.label(egui::RichText::new("🎛️ Quat Voice Oscillators (CH0 - CH3)").strong());
                    ui.separator();

                    for (idx, voice) in self.voices.iter_mut().enumerate() {
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut voice.enabled, "");
                            ui.strong(&voice.name);

                            egui::ComboBox::from_id_salt(format!("wave_combo_{}", idx))
                                .selected_text(format!("{:?}", voice.wave_type))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut voice.wave_type, QuatWaveform::DcFloor0V, "Q0: 0.0V DC Floor");
                                    ui.selectable_value(&mut voice.wave_type, QuatWaveform::QuatPulse11V, "Q1: 1.1V Pulse Wave");
                                    ui.selectable_value(&mut voice.wave_type, QuatWaveform::StaircaseSaw22V, "Q2: 2.2V Staircase Saw");
                                    ui.selectable_value(&mut voice.wave_type, QuatWaveform::QuatNoise33V, "Q3: 3.3V Quat Noise");
                                });
                        });

                        ui.horizontal(|ui| {
                            ui.label("Freq:");
                            ui.add(egui::Slider::new(&mut voice.freq_hz, 50.0..=1200.0).suffix(" Hz"));
                            ui.label("Vol:");
                            ui.add(egui::Slider::new(&mut voice.volume, 0.0..=1.0));
                        });
                        ui.separator();
                    }
                });

                ui.add_space(4.0);

                // MASTER DSP KONTROLE
                ui.group(|ui| {
                    ui.label(egui::RichText::new("🎚️ Master Audio & DSP Quantizer").strong());
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Master Vol:");
                        ui.add(egui::Slider::new(&mut self.master_volume, 0.0..=1.0));
                        ui.checkbox(&mut self.quantizer_enabled, "🔒 Lock to 4 Voltages (0..3.3V)");
                    });
                    ui.horizontal(|ui| {
                        if ui.button(if self.is_playing { "⏸️ Pause Sequencer" } else { "▶️ Play Sequencer" }).clicked() {
                            self.is_playing = !self.is_playing;
                        }
                        ui.label("BPM:");
                        ui.add(egui::DragValue::new(&mut self.bpm).range(40..=240));
                    });
                });
            });

            // DESNI PANEL: OSCILOSKOP & TRACKER
            cols[1].vertical(|ui| {
                // REAL-TIME QUAT OSCILOSKOP
                ui.group(|ui| {
                    ui.label(egui::RichText::new("📈 Real-Time Voltage Waveform Oscilloscope").strong());
                    ui.separator();

                    let (response, painter) = ui.allocate_painter(
                        egui::vec2(ui.available_width(), 140.0),
                        egui::Sense::hover(),
                    );
                    let rect = response.rect;

                    // Mreža osciloskopa
                    painter.rect_filled(rect, 4.0, egui::Color32::from_rgb(10, 15, 25));
                    painter.rect_stroke(
                        rect, 
                        4.0, 
                        egui::Stroke::new(1.0, egui::Color32::DARK_GRAY),
                        egui::StrokeKind::Middle
                    );

                    // Iscrtavanje 4 referentne naponske linije
                    let volt_levels = [(0.0, "0.0V (Q0)"), (1.1, "1.1V (Q1)"), (2.2, "2.2V (Q2)"), (3.3, "3.3V (Q3)")];
                    for (v, label) in volt_levels {
                        let y = rect.max.y - (v / 3.3) * (rect.height() - 20.0) - 10.0;
                        painter.line_segment(
                            [egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)],
                            egui::Stroke::new(0.5, egui::Color32::from_white_alpha(40)),
                        );
                        painter.text(
                            egui::pos2(rect.min.x + 5.0, y - 6.0),
                            egui::Align2::LEFT_BOTTOM,
                            label,
                            egui::FontId::monospace(9.0),
                            egui::Color32::GRAY,
                        );
                    }

                    // Iscrtavanje naponskog talasa iz bafera
                    let points_count = self.wave_buffer.len();
                    for i in 0..points_count - 1 {
                        let x1 = rect.min.x + (i as f32 / points_count as f32) * rect.width();
                        let y1 = rect.max.y - (self.wave_buffer[i] / 3.3).clamp(0.0, 1.0) * (rect.height() - 20.0) - 10.0;

                        let x2 = rect.min.x + ((i + 1) as f32 / points_count as f32) * rect.width();
                        let y2 = rect.max.y - (self.wave_buffer[i + 1] / 3.3).clamp(0.0, 1.0) * (rect.height() - 20.0) - 10.0;

                        painter.line_segment(
                            [egui::pos2(x1, y1), egui::pos2(x2, y2)],
                            egui::Stroke::new(2.0, egui::Color32::GREEN),
                        );
                    }
                });

                ui.add_space(4.0);

                // 16-STEP QUAT TRACKER SEQUENCER
                ui.group(|ui| {
                    ui.label(egui::RichText::new("🎼 16-Step Quat Tracker Sequencer").strong());
                    ui.separator();

                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        egui::Grid::new("seq_grid").spacing([3.0, 3.0]).show(ui, |ui| {
                            for step in 0..16 {
                                let is_current = self.current_step == step && self.is_playing;
                                ui.strong(
                                    egui::RichText::new(format!("{:02}", step + 1))
                                        .color(if is_current { egui::Color32::YELLOW } else { egui::Color32::GRAY })
                                );
                            }
                            ui.end_row();

                            for ch in 0..4 {
                                for step in 0..16 {
                                    let val = self.sequencer_grid[ch][step];
                                    let btn_color = match val {
                                        0 => egui::Color32::DARK_GRAY,
                                        1 => egui::Color32::LIGHT_GREEN,
                                        2 => egui::Color32::GOLD,
                                        _ => egui::Color32::LIGHT_RED,
                                    };

                                    let is_current = self.current_step == step && self.is_playing;
                                    let label = format!("Q{}", val);

                                    if ui.add(egui::Button::new(
                                        egui::RichText::new(label)
                                            .font(egui::FontId::monospace(10.0))
                                            .color(egui::Color32::BLACK)
                                            .background_color(if is_current { egui::Color32::WHITE } else { btn_color })
                                    )).clicked() {
                                        // Ciklična promena stanja Q0 -> Q1 -> Q2 -> Q3 -> Q0
                                        self.sequencer_grid[ch][step] = (val + 1) % 4;
                                    }
                                }
                                ui.end_row();
                            }
                        });
                    });
                });
            });
        });
    }
}