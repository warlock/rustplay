#[derive(Debug)]
pub struct User {
  pub username: String,
  pub password: String,
}

impl User {
  pub fn saluda(&mut self) {
    println!("Soc en {}", self.username)
  }

  pub fn change_password(&mut self, new_password: String) {
    self.password = new_password;
  }
}
