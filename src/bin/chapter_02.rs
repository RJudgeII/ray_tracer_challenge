use ray_tracer_challenge::drawing::canvas::Canvas;
use ray_tracer_challenge::drawing::color::Color;
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
    velocity: Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25,
  };
  let environment = Environment {
    gravity: Tuple::vector(0.0, -0.1, 0.0),
    wind: Tuple::vector(-0.01, 0.0, 0.0),
  };

  let mut canvas = Canvas::new(900, 550);

  while projectile.position.y > 0.0 {
    projectile = tick(environment.clone(), projectile);
    canvas.write_pixel(
      projectile.position.x.round() as usize,
      canvas.height - projectile.position.y.round() as usize - 1,
      Color::red(),
    );
  }

  canvas.write_to_png("chapter_02.png");
}
