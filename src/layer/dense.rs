pub struct Dense<
    const _IN : usize,
    const _OUT: usize > where
    [(); 1 + _IN]: {
    bw_  : [[f32; 1 + _IN]; _OUT],
    outs_: [ f32          ; _OUT],
}

impl<
    const _IN : usize,
    const _OUT: usize >
Dense< _IN, _OUT > where
    [(); 1 + _IN]: {
    pub fn new() -> Self {
        Self {
            bw_  : [[0.0; 1 + _IN]; _OUT],
            outs_: [ 0.0          ; _OUT],
        }
    }
}
