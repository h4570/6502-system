pub struct Ram {
    pub(crate) data: [u8; 65536],
}

impl Ram {
    pub fn new() -> Self {
        let mem = Ram { data: [0; 65536] };
        mem
    }
}
