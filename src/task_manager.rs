use eframe::egui;
use crate::kernel::QuatKernel4x8;
use crate::osciloskop::Osciloskop;

/// Aktivni tabovi unutar Task Manager-a
#[derive(PartialEq, Clone, Copy)]
pub enum TmTab {
    Overview,
    CpuArch,
    GpuVram,
    MemoryPaging,
    StorageIo,
    Processes,
    BusAnalyzer,
}

/// Kompletan Task Manager sa sopstvenim UI stanjem
pub struct TaskManager {
    pub current_tab: TmTab,
    pub selected_page: usize,
    pub process_search: String,
    pub show_advanced_regs: bool,
    pub selected_pid: Option<u8>,
    pub auto_step: bool,
    pub simulated_clock_mhz: f32,
    pub osciloskop: Osciloskop,
}

impl Default for TaskManager {
    fn default() -> Self {
        Self {
            current_tab: TmTab::Overview,
            selected_page: 0,
            process_search: String::new(),
            show_advanced_regs: true,
            selected_pid: None,
            auto_step: false,
            simulated_clock_mhz: 4.096,
            osciloskop: Osciloskop::new(100),
        }
    }
}

impl TaskManager {
    pub fn new() -> Self {
        Self::default()
        
    }

    /// Glavna render metoda - Poziva se direktno iz tvog main.rs:
    /// `self.task_manager.ui(ui, &mut self.kernel);`
    pub fn ui(&mut self, ui: &mut egui::Ui, kernel: &mut QuatKernel4x8) {
        // 1. AUTOMATSKO IZVRŠAVANJE CIKLUSA
        ui.heading("📊 Task Manager & Sistemske performanse");
        if self.auto_step {
            kernel.step();
            ui.ctx().request_repaint();
        }

        // 2. ZAGLAVLJE SISTEMSKOG MONITOR
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.heading("💻 QuatEngine 4^8 — Advanced Process & Hardware Telemetry");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(if self.auto_step { "⏸ PAUZIRAJ" } else { "⚡ AUTO-RUN" }).clicked() {
                        self.auto_step = !self.auto_step;
                    }
                    if ui.button("⏭ KORAK (Step)").clicked() {
                        kernel.step();
                    }
                    if ui.button("🔄 RESET KERNEL").clicked() {
                        *kernel = QuatKernel4x8::new();

                        self.osciloskop.record(kernel.reg_a.0, kernel.reg_b.0)
                    }
                });
            });
            ui.label(
                egui::RichText::new("Arhitektura: 8-Quat Base-4 | Adresni prostor: 65,536 QWords (4^8) | Parallel 4-Channel Bus")
                    .small()
                    .color(egui::Color32::GRAY),
            );
        });

//Ali kad nesto trebam dodati pa ne znam gde
//Najgori deo

        ui.add_space(8.0);

        // 3. TAB NAVIGACIJA
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.current_tab, TmTab::Overview, "📊 Overview");
            ui.selectable_value(&mut self.current_tab, TmTab::CpuArch, "🖥️ CPU Core Architecture");
            ui.selectable_value(&mut self.current_tab, TmTab::GpuVram, "🎨 GPU & VRAM");
            ui.selectable_value(&mut self.current_tab, TmTab::MemoryPaging, "💾 RAM & Paging (4^8)");
            ui.selectable_value(&mut self.current_tab, TmTab::StorageIo, "💽 NVMe Storage I/O");
            ui.selectable_value(&mut self.current_tab, TmTab::Processes, "⚙️ Process Explorer");
            ui.selectable_value(&mut self.current_tab, TmTab::BusAnalyzer, "📈 Bus Signal Analyzer");
        });

        ui.separator();
        ui.add_space(5.0);

        // Ekstrakcija kvatnih vrednosti iz Registra A za proračun dynamic telemetrije
        let q = kernel.reg_a.to_quats();
        let pc = kernel.pc;

        // 4. RENDEROVANJE IZRABRANOG TABA
        match self.current_tab {
            TmTab::Overview => self.render_overview(ui, kernel, &q, pc),
            TmTab::CpuArch => self.render_cpu_arch(ui, kernel, &q, pc),
            TmTab::GpuVram => self.render_gpu_vram(ui, kernel, &q, pc),
            TmTab::MemoryPaging => self.render_memory_paging(ui, kernel, &q, pc),
            TmTab::StorageIo => self.render_storage_io(ui, kernel, &q, pc),
            TmTab::Processes => self.render_processes(ui, kernel, &q),
            TmTab::BusAnalyzer => self.render_bus_analyzer(ui, kernel, &q),
        }
    }

    // --- TAB 1: OVERVIEW ---
    fn render_overview(&mut self, ui: &mut egui::Ui, kernel: &QuatKernel4x8, q: &[u8; 8], pc: usize) {
        ui.columns(2, |cols| {
            cols[0].group(|ui| {
                ui.label(egui::RichText::new("🖥️ CPU & ALU STATUS").strong().size(15.0));
                ui.separator();
                let avg_load = (q[0] as f32 + q[1] as f32 + q[2] as f32 + q[3] as f32) / 12.0;
                ui.label(format!("Ukupno Opterećenje CPU: {:.1}%", avg_load * 100.0));
                ui.add(egui::ProgressBar::new(avg_load).text(format!("{:.1}%", avg_load * 100.0)));
                
                ui.add_space(8.0);
                ui.label(format!("Takt Procesora: {:.3} MQHz", self.simulated_clock_mhz));
                ui.label(format!("Program Counter (PC): 0x{:04X} ({})", pc, pc));
                ui.label(format!("Carry Flag Status: {}", if kernel.carry_flag { "SET (Q3)" } else { "CLEAR (Q0)" }));
            });

            cols[1].group(|ui| {
                ui.label(egui::RichText::new("💾 SISTEMSKA MEMORIJA & BUS").strong().size(15.0));
                ui.separator();
                let used_ram = (20 + (pc * 3)) % 256;
                let ram_perc = used_ram as f32 / 256.0;
                ui.label(format!("RAM Zauzetost: {} / 256 QWords", used_ram));
                ui.add(egui::ProgressBar::new(ram_perc).text(format!("{:.1}%", ram_perc * 100.0)));

                ui.add_space(8.0);
                ui.label(format!("Širina Kvatne Magistrale: 8 Kvata (Base-4 Parallel)"));
                ui.label(format!("Aktivni I/O Kanal: Q{}", q[6]));
                ui.label(format!("VRAM Registar Slike: {}", kernel.reg_a.to_quat_str()));
            });
        });

        ui.add_space(10.0);
        ui.group(|ui| {
            ui.label(egui::RichText::new("⚡ BRZE SISTEMSKE AKCIJE I DRAJVERI").strong());
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(format!("Display Buffer Dirty: {}", kernel.drivers.display_dirty));
                if kernel.drivers.display_dirty {
                    if ui.button("Bistri ekran (Clear Display)").clicked() {
                        // Čišćenje ekrana se odvaja ovde
                    }
                }
                ui.separator();
                let (mx, my) = kernel.drivers.mouse_coords;
                ui.label(format!("Miš Koordinate: X: {:.1}, Y: {:.1}", mx, my));
            });
        });
    }

    // --- TAB 2: CPU ARCHITECTURE ---
    fn render_cpu_arch(&mut self, ui: &mut egui::Ui, kernel: &QuatKernel4x8, q: &[u8; 8], pc: usize) {
        ui.heading("🖥️ 4-Core Quaternary Microarchitecture Telemetry");
        ui.separator();

        // Prikaz 4 Jezgra
        ui.columns(4, |cols| {
            for i in 0..4 {
                cols[i].group(|ui| {
                    ui.label(egui::RichText::new(format!("CORE {}", i)).strong().color(egui::Color32::LIGHT_BLUE));
                    ui.separator();
                    let load = (q[i] as f32 / 3.0) * 100.0;
                    ui.label(format!("Nivo Stanja: Q{}", q[i]));
                    ui.add(egui::ProgressBar::new(load / 100.0).text(format!("{:.0}%", load)));
                    ui.add_space(5.0);
                    ui.label(format!("Napon: {:.1}V", q[i] as f32 * 1.1));
                    ui.label(format!("L1 Cache Hit: {:.1}%", 90.0 + (q[i] as f32 * 3.0)));
                    ui.label(format!("Takt Core-a: {:.2} MQHz", 4.0 + (q[i] as f32 * 0.25)));
                });
            }
        });

        ui.add_space(10.0);

        // Microarchitecture Pipeline & Vector Units
        ui.columns(2, |cols| {
            cols[0].group(|ui| {
                ui.label(egui::RichText::new("⚙️ PIPELINE & EXECUTION STAGES").strong());
                ui.separator();
                let stage = pc % 4;
                ui.horizontal(|ui| {
                    ui.label("Pipeline Stage:");
                    ui.label(egui::RichText::new(match stage {
                        0 => "[ FETCH (Q0) ]",
                        1 => "[ DECODE (Q1) ]",
                        2 => "[ EXECUTE (Q2) ]",
                        _ => "[ WRITEBACK (Q3) ]",
                    }).strong().color(egui::Color32::GREEN));
                });
                ui.label(format!("Instruction Queue: {} / 16 instrukcija", (q[0] + 1) * 4));
                ui.label(format!("Branch Predictor Accuracy: {:.1}%", 88.5 + (q[1] as f32 * 3.2)));
                ui.label(format!("Speculative Execution: Enabled (Q{})", q[2]));
            });

            cols[1].group(|ui| {
                ui.label(egui::RichText::new("📐 Q-AVX VECTOR REGISTERS (Baza 4)").strong());
                ui.separator();
                ui.monospace(format!("VREG0: [Q{}, Q{}, Q{}, Q{}]", q[0], q[1], q[2], q[3]));
                ui.monospace(format!("VREG1: [Q{}, Q{}, Q{}, Q{}]", q[4], q[5], q[6], q[7]));
                ui.monospace(format!("ALU Output: {} (Dec: {})", kernel.reg_a.to_quat_str(), kernel.reg_a.0));
                ui.monospace(format!("Flags: [Carry: {}, Parity: Q{}, Zero: {}]", 
                    kernel.carry_flag, q[3], kernel.reg_a.0 == 0));
            });
        });
    }

    // --- TAB 3: GPU & VRAM ---
    fn render_gpu_vram(&mut self, ui: &mut egui::Ui, kernel: &QuatKernel4x8, q: &[u8; 8], pc: usize) {
        ui.heading("🎨 Quat-GPU Render Engine & VRAM Subsystem");
        ui.separator();

        ui.columns(2, |cols| {
            cols[0].group(|ui| {
                ui.label(egui::RichText::new("🖼️ DISPLAY & FRAMEBUFFER").strong());
                ui.separator();
                ui.label(format!("Rezolucija Simulacije: 256 x 256 QuatPixels"));
                ui.label(format!("Dubina Boja: 4 Stanja po Kanalu (2-bit per Quat)"));
                ui.label(format!("Frame Buffer Dirty Flag: {}", kernel.drivers.display_dirty));
                ui.label(format!("Ciljani Refresh Rate: {} Hz", 60 + (q[1] as i32 * 15)));
                ui.label(format!("VSYNC Status: Locked (Q3)"));

                ui.add_space(8.0);
                ui.label(egui::RichText::new("16 Shader Cores Activity:").strong());
                egui::Grid::new("shader_grid").min_col_width(20.0).show(ui, |ui| {
                    for core in 0..16 {
                        let active = (q[core % 8] + (core as u8 / 4)) % 4;
                        let color = match active {
                            0 => egui::Color32::DARK_GRAY,
                            1 => egui::Color32::BLUE,
                            2 => egui::Color32::GOLD,
                            _ => egui::Color32::GREEN,
                        };
                        ui.label(egui::RichText::new(format!("S{:02}", core)).color(color));
                        if (core + 1) % 8 == 0 { ui.end_row(); }
                    }
                });
            });

            cols[1].group(|ui| {
                ui.label(egui::RichText::new("💾 VRAM ADRESNI PROSTOR ($4^8 = 65,536$)").strong());
                ui.separator();
                let vram_used = (q[4] as usize * 16384) + (pc * 64);
                let vram_total = 65536;
                let vram_perc = vram_used as f32 / vram_total as f32;

                ui.label(format!("Zauzetost VRAM: {} / {} QWords", vram_used, vram_total));
                ui.add(egui::ProgressBar::new(vram_perc).text(format!("{:.2}%", vram_perc * 100.0)));

                ui.add_space(8.0);
                ui.label(format!("VRAM Propusni Opseg: {:.1} QMB/s", 128.0 + (q[5] as f32 * 32.0)));
                ui.label(format!("Quadrant Render Mode: Quadrant-Q{}", q[4]));
                ui.label(format!("Raytracing BVT Nodes: {} aktivno", (q[7] as u16 + 1) * 128));
            });
        });
    }

    // --- TAB 4: MEMORY & PAGING ---
    fn render_memory_paging(&mut self, ui: &mut egui::Ui, _kernel: &QuatKernel4x8, q: &[u8; 8], pc: usize) {
        ui.heading("💾 System RAM & Quat-Paging Engine (Base-4 Memory Mapping)");
        ui.separator();

        let total_ram = 256;
        let heap_used = (20 + (pc * 3)) % total_ram;
        let stack_used = (q[3] as usize * 15) + 10;
        let total_used = heap_used + stack_used;
        let progress = total_used as f32 / total_ram as f32;

        ui.group(|ui| {
            ui.label(egui::RichText::new(format!("RAM Zauzetost: {} / {} QWords ({:.1}%)", total_used, total_ram, progress * 100.0)).strong());
            ui.add(egui::ProgressBar::new(progress).text(format!("{:.1}%", progress * 100.0)));
            ui.columns(3, |cols| {
                cols[0].label(format!("Heap Segment: {} QW", heap_used));
                cols[1].label(format!("Stack Segment: {} QW", stack_used));
                cols[2].label(format!("Fragmentacija: Q{}", q[2]));
            });
        });

        ui.add_space(8.0);
        ui.label(egui::RichText::new("🔍 Visual Page Inspector (64 Stranica u Bazi 4):").strong());

        // Selectable page preview
        ui.horizontal(|ui| {
            ui.label("Prikaz Stranice:");
            for p in 0..8 {
                if ui.selectable_label(self.selected_page == p, format!("Page Q{}", p)).clicked() {
                    self.selected_page = p;
                }
            }
        });

        ui.group(|ui| {
            ui.label(format!("Detalji Stranice Q{} (Adrese 0x{:04X} - 0x{:04X}):", 
                self.selected_page, self.selected_page * 32, (self.selected_page + 1) * 32 - 1));
            
            egui::Grid::new("page_grid_inspector").min_col_width(45.0).show(ui, |ui| {
                for cell in 0..32 {
                    let addr = self.selected_page * 32 + cell;
                    let val = (q[cell % 8] + (cell as u8 / 4)) % 4;
                    let color = match val {
                        0 => egui::Color32::GRAY,
                        1 => egui::Color32::LIGHT_BLUE,
                        2 => egui::Color32::LIGHT_YELLOW,
                        _ => egui::Color32::LIGHT_RED,
                    };
                    ui.label(egui::RichText::new(format!("{:02X}:Q{}", addr, val)).color(color).monospace());
                    if (cell + 1) % 8 == 0 { ui.end_row(); }
                }
            });
        });
    }

    // --- TAB 5: STORAGE I/O ---
    fn render_storage_io(&mut self, ui: &mut egui::Ui, _kernel: &QuatKernel4x8, q: &[u8; 8], _pc: usize) {
        ui.heading("💽 Quat-NVMe Storage Controller & 4-Channel I/O");
        ui.separator();

        ui.columns(2, |cols| {
            cols[0].group(|ui| {
                ui.label(egui::RichText::new("🚀 PROPUSNI OPSEG & BRZINA").strong());
                ui.separator();
                let read_qps = (q[6] as u16 + 1) * 256;
                let write_qps = (q[7] as u16 + 1) * 128;

                ui.label(format!("Read Transfer Rate:  {} QPS (Quat Ops/s)", read_qps));
                ui.label(format!("Write Transfer Rate: {} QPS (Quat Ops/s)", write_qps));
                ui.label(format!("Prosečna Latencija: {:.2} μs", 10.5 - (q[0] as f32 * 1.5)));
                ui.label(format!("DMA Controller State: Active (IRQ Q{})", q[1]));
            });

            cols[1].group(|ui| {
                ui.label(egui::RichText::new("⚡ STATUS 4 PHISICAL KANALA").strong());
                ui.separator();
                for ch in 0..4 {
                    let state = q[ch];
                    let status_str = match state {
                        0 => "IDLE (Q0)",
                        1 => "READING (Q1)",
                        2 => "WRITING (Q2)",
                        _ => "BUSY/FLUSH (Q3)",
                    };
                    ui.label(format!("Channel {}: {}", ch, status_str));
                }
            });
        });

        ui.add_space(8.0);
        ui.group(|ui| {
            ui.label(egui::RichText::new("📊 NVMe Queue Depth in Base-4:").strong());
            egui::Grid::new("queue_grid").striped(true).min_col_width(120.0).show(ui, |ui| {
                ui.strong("Queue ID"); ui.strong("Kanal"); ui.strong("Popunjenost"); ui.strong("Status Reda"); ui.end_row();
                ui.label("QUEUE_00"); ui.label("Channel 0"); ui.label(format!("{} / 16", q[0] * 4)); ui.label("OK"); ui.end_row();
                ui.label("QUEUE_01"); ui.label("Channel 1"); ui.label(format!("{} / 16", q[1] * 4)); ui.label("OK"); ui.end_row();
                ui.label("QUEUE_02"); ui.label("Channel 2"); ui.label(format!("{} / 16", q[2] * 4)); ui.label("ACTIVE"); ui.end_row();
                ui.label("QUEUE_03"); ui.label("Channel 3"); ui.label(format!("{} / 16", q[3] * 4)); ui.label("WAITING"); ui.end_row();
            });
        });
    }

    // --- TAB 6: PROCESS EXPLORER ---
    fn render_processes(&mut self, ui: &mut egui::Ui, _kernel: &QuatKernel4x8, q: &[u8; 8]) {
        ui.heading("⚙️ QuatOS Process Explorer & Thread Scheduler");
        ui.separator();

        ui.horizontal(|ui| {
            ui.label("🔍 Pretraga Procesa:");
            ui.text_edit_singleline(&mut self.process_search);
            if ui.button("Bistri").clicked() {
                self.process_search.clear();
            }
        });

        ui.add_space(5.0);

        egui::Grid::new("proc_table_full")
            .striped(true)
            .min_col_width(85.0)
            .show(ui, |ui| {
                ui.strong("PID");
                ui.strong("Naziv Procesa");
                ui.strong("Prioritet");
                ui.strong("Stanje");
                ui.strong("RAM Set");
                ui.strong("Kvatni Registar Niti");
                ui.strong("Akcija");
                ui.end_row();

                let procs = [
                    ("01", "QuatOS_Kernel.sys", "Q3 (CRITICAL)", "RUNNING", "16 QW", format!("Q3310-Q{}", q[0])),
                    ("02", "DisplayDriver.dll", "Q2 (HIGH)", "ACTIVE", "32 QW", format!("Q2210-Q{}", q[1])),
                    ("03", "Raytracer_4^8.apk", "Q1 (NORMAL)", "COMPUTING", "64 QW", format!("Q1320-Q{}", q[2])),
                    ("04", "AudioSynth_4State.exe", "Q0 (LOW)", "SLEEP", "8 QW", format!("Q0010-Q{}", q[3])),
                    ("05", "NVMe_Storage_Daemon", "Q2 (HIGH)", "I/O WAIT", "12 QW", format!("Q2011-Q{}", q[4])),
                ];

                for proc in procs.iter() {
                    if !self.process_search.is_empty() && !proc.1.to_lowercase().contains(&self.process_search.to_lowercase()) {
                        continue;
                    }

                    ui.label(proc.0);
                    ui.label(proc.1);
                    ui.label(proc.2);
                    ui.label(proc.3);
                    ui.label(proc.4);
                    ui.monospace(&proc.5);
                    if ui.button("Inspekcija").clicked() {
                        self.selected_pid = proc.0.parse::<u8>().ok();
                    }
                    ui.end_row();
                }
            });

        if let Some(pid) = self.selected_pid {
            ui.add_space(8.0);
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(format!("🔎 Detaljna Inspekcija Procesa PID: {:02X}", pid)).strong());
                    if ui.button("❌ Zatvori").clicked() {
                        self.selected_pid = None;
                    }
                });
                ui.label("Thread Context Switch Count: 1,420 cycles");
                ui.label("Virtual Memory Page Base: 0x00A0");
                ui.label("Quat Signal Handler: Active (SigQ2)");
            });
        }
    }

    // --- TAB 7: BUS ANALYZER & LOGS ---
    fn render_bus_analyzer(&mut self, ui: &mut egui::Ui, kernel: &QuatKernel4x8, q: &[u8; 8]) {
        ui.heading("📈 Quat-Bus Logic Signal Analyzer & System Terminal");
        ui.separator();

        ui.group(|ui| {
            ui.label(egui::RichText::new("Naponski Nivoi 8-Kvatne Magistrale (Volts):").strong());
            ui.separator();
            ui.columns(8, |cols| {
                for i in 0..8 {
                    let volt = q[i] as f32 * 1.1;
                    cols[i].label(format!("Q{}", i));
                    cols[i].label(egui::RichText::new(format!("{:.1}V", volt)).strong());
                    cols[i].add(egui::ProgressBar::new(q[i] as f32 / 3.0).show_percentage());
                }
            });
        });

        ui.add_space(8.0);

        ui.group(|ui| {
            ui.label(egui::RichText::new("📜 TERMINAL KERNEL LOGOVI (Real-Time)").strong());
            ui.separator();
            egui::ScrollArea::vertical().max_height(140.0).show(ui, |ui| {
                for log in &kernel.logs {
                    ui.monospace(log);
                }
            });
        });
        self.osciloskop.ui(ui);
    }
}