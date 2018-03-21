


// Dumb texture
pub struct Texture {
    tex_id : u32,
    w : usize,
    h : usize,
}

impl Texture {

    pub fn new(w : usize, h : usize) -> Self {

        let tex_id = 0;

        Self { tex_id, w, h }
    }

}

