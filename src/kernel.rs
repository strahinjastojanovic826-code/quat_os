/// 4-stanjac: Osnovni tip za jedan kvat (0, 1, 2, 3)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Quat {
    Q0 = 0,
    Q1 = 1,
    Q2 = 2,
    Q3 = 3,
}

impl Quat {
    pub fn from_u8(val: u8) -> Self {
        match val % 4 {
            0 => Quat::Q0,
            1 => Quat::Q1,
            2 => Quat::Q2,
            _ => Quat::Q3,
        }
    }

    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

/// QuatWord16 predstavlja 8 kvata (4^8 = 65,536 stanja)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct QuatWord16(pub u16);

impl QuatWord16 {
    pub const ZERO: QuatWord16 = QuatWord16(0);

    pub fn new(val: u16) -> Self {
        QuatWord16(val)
    }

    pub fn write_quat(&mut self, idx: usize, val: Quat) {
        // Svaki kvat (2 bita) upisujemo na odgovarajući offset u u16 reči (8 kvata po reči)
        let mask = !(0b11 << (idx * 2));
        self.0 = (self.0 & mask) | (((val as u16) & 0b11) << (idx * 2));
    }

    pub fn to_quats(&self) -> [u8; 8] {
        let mut q = [0u8; 8];
        let mut temp = self.0;
        for i in (0..8).rev() {
            q[i] = (temp % 4) as u8;
            temp /= 4;
        }
        q
    }

    pub fn from_quats(quats: [u8; 8]) -> Self {
        let mut val: u16 = 0;
        for &q in &quats {
            val = val * 4 + ((q & 3) as u16);
        }
        QuatWord16(val)
    }

    pub fn add(&self, rhs: &QuatWord16) -> (QuatWord16, bool) {
        let a = self.to_quats();
        let b = rhs.to_quats();
        let mut res = [0u8; 8];
        let mut carry = 0u8;

        for i in (0..8).rev() {
            let sum = a[i] + b[i] + carry;
            res[i] = sum % 4;
            carry = sum / 4;
        }

        (QuatWord16::from_quats(res), carry > 0)
    }

    pub fn to_quat_str(&self) -> String {
        let q = self.to_quats();
        format!("Q{}{}{}{}{}{}{}{}", q[0], q[1], q[2], q[3], q[4], q[5], q[6], q[7])
    }
}

/// HARDVERSKI I/O DRAJVERI U BAZI 4
pub struct QuatDrivers {
    pub display_dirty: bool,
    pub mouse_coords: (f32, f32),
    pub audio_driver_state: Quat,
    pub vram: [QuatWord16; 16], // Pomoćna VRAM memorija za displej
}

impl Default for QuatDrivers {
    fn default() -> Self {
        Self {
            display_dirty: false,
            mouse_coords: (0.0, 0.0),
            audio_driver_state: Quat::Q0,
            vram: [QuatWord16::ZERO; 16],
        }
    }
}

impl QuatDrivers {
    pub fn update_mouse(&mut self, x: f32, y: f32) {
        self.mouse_coords = (x, y);
        self.display_dirty = true;
    }

    pub fn set_audio_state(&mut self, state: Quat) {
        self.audio_driver_state = state;
    }
}

/// KERNEL SIMULATOR ZA 4^8 ARHITEKTURU SA DRAJVERIMA
pub struct QuatKernel4x8 {
    pub ram: Vec<QuatWord16>,
    pub reg_a: QuatWord16,
    pub reg_b: QuatWord16,
    pub pc: usize,
    pub carry_flag: bool,
    pub is_running: bool,
    pub drivers: QuatDrivers, // <-- Ubaceni drajveri!
    pub logs: Vec<String>,
}

impl QuatKernel4x8 {
    pub fn new() -> Self {
        let mut kernel = Self {
            ram: vec![QuatWord16::ZERO; 256],
            reg_a: QuatWord16::ZERO,
            reg_b: QuatWord16::ZERO,
            pc: 0,
            carry_flag: false,
            is_running: false,
            drivers: QuatDrivers::default(),
            logs: vec!["Kernel 4^8 inicijalizovan sa I/O drajverima.".to_string()],
        };

        kernel.ram[0] = QuatWord16::from_quats([0, 0, 0, 0, 1, 2, 3, 0]);
        kernel.ram[1] = QuatWord16::from_quats([0, 0, 0, 0, 2, 1, 1, 3]);
        kernel.ram[2] = QuatWord16::from_quats([1, 1, 1, 1, 0, 0, 0, 1]);
        kernel
    }

    pub fn step(&mut self) {
        if self.pc >= self.ram.len() {
            self.is_running = false;
            return;
        }

        let data = self.ram[self.pc];

        // ALU operacija
        let (new_reg_a, carry) = self.reg_a.add(&data);
        self.reg_a = new_reg_a;
        self.carry_flag = carry;

        // Drajverska logika: sinhronizacija sa stanjem registara
        let first_quat = self.reg_a.to_quats()[7];
        self.drivers.audio_driver_state = Quat::from_u8(first_quat);
        self.drivers.display_dirty = true;

        self.log(format!(
            "PC: {:02X} | RegA: {} | Audio State: {:?}",
            self.pc,
            self.reg_a.to_quat_str(),
            self.drivers.audio_driver_state
        ));

        self.pc += 1;
    }

    pub fn log(&mut self, msg: String) {
        self.logs.push(msg);
        if self.logs.len() > 20 {
            self.logs.remove(0);
        }
    }
}

//Jebi se binarni sistemu Jebi se Jebi se Jebi se Jebi se
// JEBIIII SEEEEEEE!!!!!!!