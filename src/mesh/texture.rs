enum TextureType {
    Diffuse,
    Specular,
}

pub struct Texture {
    id: u32,
    texture_type: TextureType,
}
