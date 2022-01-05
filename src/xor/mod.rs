pub trait XOR {
    fn xor(&self, _: &Self) -> Vec<u8>;
    // fn xor_inplace(&mut self, _: &Self);
}

impl XOR for [u8] {
    fn xor(&self, key: &[u8]) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.len());

        for chunk in self.chunks(key.len()) {
            for (idx, plaintext_byte) in chunk.iter().enumerate() {
                result.push(plaintext_byte ^ key[idx])
            }
        }

        result
    }
    // fn xor(&self, t: &[u8]) -> Vec<u8> {
    //     let mut result = self.to_vec();
    //     result[..].xor_inplace(t);
    //     result
    // }

    // fn xor_inplace(&mut self, t: &[u8]) {
    //     for chunk in self.chunks_mut(t.len()) {
    //         let len = chunk.len();
    //         for (c, &d) in chunk.iter_mut().zip(t[..len].iter()) {
    //             *c ^= d;
    //         }
    //     }
    // }
}
