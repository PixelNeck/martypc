use crate::cpu_808x::*;
use crate::bytequeue::*;

pub struct InstructionQueue {
    size: usize,
    len: usize,
    back: usize,
    front: usize,
    q: [u8; QUEUE_MAX],
    dt: [QueueType; QUEUE_MAX],
    preload: Option<u8>,
    delay_flag: bool,
}

impl Default for InstructionQueue {
    fn default() -> Self {
        Self::new(4)
    }
}

impl InstructionQueue {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            len: 0,
            back: 0,
            front: 0,
            q: [0; QUEUE_MAX],
            dt: [QueueType::First; QUEUE_MAX],
            preload: None,
            delay_flag: false,
        }
    }

    pub fn set_size(&mut self, size: usize) {
        assert!(size <= QUEUE_MAX);
        self.size = size;
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        self.len == self.size
    }

    #[inline]
    pub fn get_preload(&mut self) -> Option<u8> {
        let preload = self.preload;
        self.preload = None;
        preload
    }

    #[inline]
    pub fn has_preload(&self) -> bool {
        if let Some(_) = self.preload {
            true
        }
        else {
            false
        }
    }

    #[inline]
    pub fn set_preload(&mut self) {
        if self.len > 0 {
            let byte = self.pop();
            self.preload = Some(byte);
        }
        else {
            panic!("Tried to preload with empty queue.")
        }
    }

    pub fn push8(&mut self, byte: u8) {
        if self.len < self.size {

            self.q[self.front] = byte;
            //self.dt[self.front] = dtype;

            self.front = (self.front + 1) % self.size;
            self.len += 1;
            self.delay_flag = false;
        }
        else {
            panic!("Queue overrun!");
        }
    }

    pub fn push16(&mut self, word: u16) {

        self.push8((word & 0xFF) as u8);
        self.push8(((word >> 8) & 0xFF) as u8);
    }

    pub fn pop(&mut self) -> u8 {
        if self.len > 0 {
            let byte = self.q[self.back];
            //let dt = self.dt[self.back];

            self.back = (self.back + 1) % self.size;
            self.len -= 1;

            if self.len >= 3 {
                // Queue length of 3 or 4 after pop. Set delay flag.
                // This should cover 8088 and 8086(?)
                self.delay_flag = true;
            }
            else {
                self.delay_flag = false;
            }

            return byte
        }
        panic!("Queue underrun!");
    }

    #[inline]
    pub fn have_delay(&self) -> bool {
        self.delay_flag
    }

    pub fn flush(&mut self) {
        self.len = 0;
        self.back = 0;
        self.front = 0;
        self.preload = None;
        self.delay_flag = false;
    }

    /// Convert the contents of the processor instruction queue to a hexadecimal string.
    pub fn to_string(&self) -> String {

        let mut base_str = "".to_string();

        for i in 0..self.len {
            base_str.push_str(&format!("{:02X}", self.q[(self.back + i) % self.size]));
        }

        base_str
    }

    /// Write the contents of the processor instruction queue in order to the
    /// provided slice of u8. The slice must be the same size as the current piq 
    /// length for the given cpu type.
    pub fn to_slice(&self, slice: &mut [u8]) {

        assert_eq!(self.size, slice.len());

        for i in 0..self.len {
            slice[i] = self.q[(self.back + i) % self.size];
        }
    }
}