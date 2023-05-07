pub struct WriteStream {
    pub bytes: Vec<u8>,
    pub size_t_size: u8,
    pub int_size: u8,
}

impl WriteStream {
    pub fn new() -> Self {
        Self {
            bytes: vec![],
            size_t_size: 4,
            int_size: 4,
        }
    }

    pub fn from_size(size_t_size: u8, int_size: u8) -> Self {
        Self {
            bytes: vec![],
            size_t_size,
            int_size,
        }
    }

    pub fn write(&mut self, bytes: &mut Vec<u8>) {
        self.bytes.append(bytes);
    }

    pub fn write_int8(&mut self, num: u8) {
        self.write(&mut vec![num]);
    }

    pub fn write_int16(&mut self, num: u16) {
        self.write(&mut vec![
            (num & 0xff).try_into().expect("Failed to convert number"),
            ((num >> 8) & 0xff)
                .try_into()
                .expect("Failed to convert number"),
        ]);
    }

    pub fn write_int32(&mut self, num: u32) {
        self.write(&mut vec![
            (num & 0xff).try_into().expect("Failed to convert number"),
            ((num >> 8) & 0xff)
                .try_into()
                .expect("Failed to convert number"),
            ((num >> 16) & 0xff)
                .try_into()
                .expect("Failed to convert number"),
            ((num >> 24) & 0xff)
                .try_into()
                .expect("Failed to convert number"),
        ]);
    }

    pub fn write_int64(&mut self, num: u64) {
        self.write_int32(
            (num & 0xffffffff)
                .try_into()
                .expect("Failed to convert number"),
        );
        self.write_int32(
            ((num >> 32) & 0xffffffff)
                .try_into()
                .expect("Failed to convert number"),
        );
    }

    pub fn write_double(&mut self, num: f64) {
        self.write(&mut f64::to_le_bytes(num).to_vec());
    }

    pub fn write_size_t(&mut self, num: u64) {
        if self.size_t_size == 4 {
            self.write_int32(num as u32);
        } else {
            self.write_int64(num);
        }
    }

    pub fn write_int(&mut self, num: u64) {
        if self.int_size == 4 {
            self.write_int32(num as u32);
        } else {
            self.write_int64(num);
        }
    }

    pub fn write_string_len(&mut self, str: &String) {
        self.write(&mut str.as_bytes().to_vec());
    }

    pub fn write_string(&mut self, str: &String) {
        self.write_size_t(str.len() as u64);
        self.write_string_len(str);
    }
}
