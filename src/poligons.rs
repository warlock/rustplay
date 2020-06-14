pub struct Rectangulo {
  pub ancho: u32,
  pub alto: u32,
}

pub fn area(reactangulo: Rectangulo) -> u32 {
  reactangulo.ancho * reactangulo.alto
}

pub fn area_pointer(reactangulo: &Rectangulo) -> u32 {
  reactangulo.ancho * reactangulo.alto
}
