mod kernel;
mod clock;
mod loeb_law_school;
mod oled_burnin;
mod pep_overthink;
mod quat_fs;
mod quat_net;
mod quat_strategy;
mod quat_lang;
mod quat_synth;
mod task_manager;
mod terminal;
mod spanish_tutor;
mod sojic_bureaucracy;
mod osciloskop;
mod compiler;
mod bus_analyzer;

use eframe::egui;
use kernel::{Quat, QuatKernel4x8};
use clock::DigitalClock;
use loeb_law_school::LoebLawSchool;
use oled_burnin::OledBurnInEngine;
use pep_overthink::PepOverthinkEngine;
use quat_fs::QuatFSEngine;
use quat_lang::QuatLangEngine;
use quat_net::QuatNetEngine;
use quat_strategy::GrandStrategyEngine;
use quat_synth::QuatSynthEngine;
use sojic_bureaucracy::SojicBureaucracyEngine;
use spanish_tutor::SpanishTutor;
use task_manager::TaskManager;
use terminal::QuatTerminal;
use osciloskop::Osciloskop;
use compiler::CodeCompiler;
use bus_analyzer::BusAnalyzer;

// Nesto sam se olenjio oko 3 projekta
//Nadam se da mi ne zamerate

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Tab {
    Net,
    PepOverthink,
    Terminal,
    TaskManager,
    Spanish,
    Sojic,
    Synth,
    Strategy,
    Lang,
    FS,
    Oled,
    LawSchool,
    Clock,
    Bus,
    
}

 struct QuatApp {
    kernel: QuatKernel4x8,
    clock: DigitalClock,
    loeb_law_school: LoebLawSchool,
    oled_burnin: OledBurnInEngine,
    pep_overthink: PepOverthinkEngine,
    quat_fs: QuatFSEngine,
    quat_lang: QuatLangEngine,
    quat_net: QuatNetEngine,
    quat_strategy: GrandStrategyEngine,
    quat_synth: QuatSynthEngine,
    sojic_bureaucracy: SojicBureaucracyEngine,
    spanish_tutor: SpanishTutor,
    task_manager: TaskManager,
    terminal: QuatTerminal,
    compiler: CodeCompiler,
    bus_analyzer: BusAnalyzer,

    active_tab: Option<Tab>,
     
}

//Ostavljaj Tab takav kakav je
//Ni sam ne znam sto treba

impl Default for QuatApp {
    fn default() -> Self {
        Self {
            kernel: QuatKernel4x8::new(),
            clock: DigitalClock::new(),
            loeb_law_school: LoebLawSchool::new(),
            oled_burnin: OledBurnInEngine::new(),
            pep_overthink: PepOverthinkEngine::new(),
            quat_fs: QuatFSEngine::new(),
            quat_lang: QuatLangEngine::new(),
            quat_net: QuatNetEngine::new(),
            quat_strategy: GrandStrategyEngine::new(),
            quat_synth: QuatSynthEngine::new(),
            sojic_bureaucracy: SojicBureaucracyEngine::new(),
            spanish_tutor: SpanishTutor::new(),
            task_manager: TaskManager::new(),
            terminal: QuatTerminal::new(),
            compiler: CodeCompiler::new(),
            bus_analyzer: BusAnalyzer::new(100),

            active_tab: None,
        }
    }
}

//Ne brisi <> 
//Inace kompajler vristi

impl eframe::App for QuatApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("⚡ Četverni Procesor ⬟ 4^8 Kernel & Hardverski Drajveri");
        ui.label("Arhitektura: 4 stanja (Q0-Q3 / 0.0V - 3.3V) | 8 kvata (65.536 adresa)");
        ui.separator();

        // --- 1. OSNOVNE KONTROLE PROCESORA ---
        ui.horizontal(|ui| {
            if ui.button("▶ Step (Jedan korak)").clicked() {
                self.kernel.step();
            }
            if ui.button("🔄 Reset").clicked() {
                self.kernel = QuatKernel4x8::new();
            }
        });

        ui.add_space(8.0);

        // --- 2. PRIKAZ REGISTARA (8 Kvata / 4 Stanja) ---
        ui.group(|ui| {
            ui.label(egui::RichText::new("REGISTRI PROCESORA (8 Kvata)").strong());
            ui.label(format!("Reg A: {} (Dekadno: {})", self.kernel.reg_a.to_quat_str(), self.kernel.reg_a.0));
            ui.label(format!("Reg B: {} (Dekadno: {})", self.kernel.reg_b.to_quat_str(), self.kernel.reg_b.0));
            ui.label(format!("Program Counter (PC): {:02X}", self.kernel.pc));
            ui.label(format!("Carry Flag: {}", self.kernel.carry_flag));
        });

        ui.add_space(8.0);

        // --- 3. HARDVERSKI DRAJVERI (Baza 4) ---
        ui.group(|ui| {
            ui.label(egui::RichText::new("HARDVERSKI DRAJVERI (Baza 4)").strong());

            // Displej Drajver
            ui.label(format!("Display Buffer Dirty: {}", self.kernel.drivers.display_dirty));
            if self.kernel.drivers.display_dirty {
                if ui.button("Bistri ekran (Clear Display)").clicked() {
                    self.kernel.drivers.display_dirty = false;
                }
            }

            // Miš Drajver
            let (mx, my) = self.kernel.drivers.mouse_coords;
            ui.label(format!("Koordinate miša: X: {:.1}, Y: {:.1}", mx, my));

            // Audio Drajver
            ui.horizontal(|ui| {
                ui.label(format!("Audio Driver Stanje: {:?}", self.kernel.drivers.audio_driver_state));
                if ui.button("Postavi Audio Q0").clicked() {
                    self.kernel.drivers.set_audio_state(Quat::Q0);
                }
                if ui.button("Postavi Audio Q3").clicked() {
                    self.kernel.drivers.set_audio_state(Quat::Q3);
                }
            });
        });

        ui.add_space(10.0);

        // --- 4. TRAKA SA TABOVIMA (Izbor Aktivne Aplikacije) ---
        ui.separator();
        egui::ScrollArea::horizontal().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.active_tab, Some(Tab::PepOverthink), "⚽ Pep 4^8 Overthink");
                ui.selectable_value(&mut self.active_tab, Some(Tab::Synth), "🎵 Quat Chiptune Synth");
                ui.selectable_value(&mut self.active_tab, Some(Tab::FS), "📁 QuatFS");
                ui.selectable_value(&mut self.active_tab, Some(Tab::Lang), "🔤 QuatLang");
                ui.selectable_value(&mut self.active_tab, Some(Tab::Net), "🌐 QuatNet");
                ui.selectable_value(&mut self.active_tab, Some(Tab::Strategy), "⚔️ Strategy Engine");
                ui.selectable_value(&mut self.active_tab, Some(Tab::Terminal), "💻 Terminal");
                ui.selectable_value(&mut self.active_tab, Some(Tab::TaskManager), "📊 Task Manager");
                ui.selectable_value(&mut self.active_tab, Some(Tab::Clock), "⏰ Quat Clock");
                ui.selectable_value(&mut self.active_tab, Some(Tab::LawSchool), "⚖️ Loeb Law School");
                ui.selectable_value(&mut self.active_tab, Some(Tab::Sojic), "💼 Šojić Bureaucracy");
                ui.selectable_value(&mut self.active_tab, Some(Tab::Spanish), "🇪🇸 Spanish Tutor");
                ui.selectable_value(&mut self.active_tab, Some(Tab::Oled), "🖥️ OLED Burn-In Test");
                ui.selectable_value(&mut self.active_tab, Some(Tab::Bus), "🚌 Quat_Bus");
            });
        });
        ui.separator();
        ui.add_space(8.0);

        // --- 5. EXECUTION BLOCK: Prikazuje se SAMO TAB koji je izabran ---
        match self.active_tab {
            Some(Tab::PepOverthink) => self.pep_overthink.ui(ui, &mut self.kernel),
            Some(Tab::Synth) => self.quat_synth.ui(ui, &mut self.kernel),
            Some(Tab::FS) => self.quat_fs.ui(ui, &mut self.kernel),
            Some(Tab::Lang) => self.quat_lang.ui(ui, &mut self.kernel),
            Some(Tab::Net) => self.quat_net.ui(ui, &mut self.kernel),
            Some(Tab::Strategy) => self.quat_strategy.ui(ui, &mut self.kernel),
            Some(Tab::Terminal) => self.terminal.ui(ui, &mut self.kernel),
            Some(Tab::TaskManager) => self.task_manager.ui(ui, &mut self.kernel),
            Some(Tab::Clock) => self.clock.ui(ui),
            Some(Tab::LawSchool) => self.loeb_law_school.ui(ui, &mut self.kernel),
            Some(Tab::Sojic) => self.sojic_bureaucracy.ui(ui, &mut self.kernel),
            Some(Tab::Spanish) => self.spanish_tutor.ui(ui, &mut self.kernel),
            Some(Tab::Oled) => self.oled_burnin.ui(ui, &mut self.kernel),
            Some(Tab::Bus) => self.bus_analyzer.ui(ui),
           None => {
        }
    }
        ui.add_space(10.0);

        // --- 6. TERMINAL LOGOVI PROCESORA ---
        ui.group(|ui| {
            ui.label(egui::RichText::new("TERMINAL LOGS").strong());
            egui::ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
                for log in &self.kernel.logs {
                    ui.monospace(log);
                }
            });
        });

        // --- 7. HVATANJE POZICIJE MIŠA ---
        if let Some(pos) = ui.ctx().pointer_latest_pos() {
            self.kernel.drivers.update_mouse(pos.x, pos.y);
        }
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Moja Simulacija",
        options,
        Box::new(|_cc| Ok(Box::new(QuatApp::default()))),
    )
}