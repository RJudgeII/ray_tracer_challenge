use ray_tracer_challenge::math::tuple::Tuple;

#[derive(Clone)]
pub struct Projectile {
  pub position: Tuple,
  pub velocity: Tuple,
}

#[derive(Clone)]
pub struct Environment {
  pub gravity: Tuple,
  pub wind: Tuple,
}

pub fn tick(environment: Environment, projectile: Projectile) -> Projectile {
  let mut output = projectile.clone();
  output.position += projectile.velocity;
  output.velocity += environment.gravity + environment.wind;
  output
}

fn main() {
  let mut projectile = Projectile {
    position: Tuple::point(0.0, 1.0, 0.0),
    velocity: Tuple::vector(1.0, 1.0, 0.0).normalize(),
  };
  let environment = Environment {
    gravity: Tuple::vector(0.0, -0.1, 0.0),
    wind: Tuple::vector(-0.01, 0.0, 0.0),
  };

  while projectile.position.y > 0.0 {
    projectile = tick(environment.clone(), projectile);
    println!("Position:\t{:?}", projectile.position);
  }
}
