use crate::{tuple::Tuple, color::Color};

pub struct PointLight {
    pub position: Tuple,
    pub intensity: Color
}

#[derive(Copy, Clone)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64
}

impl Material {
    pub const DEFAULT: Material = Material {
        color: Color { red: 1.0, green: 1.0, blue: 1.0 },
        ambient: 0.1,
        diffuse: 0.9,
        specular: 0.9,
        shininess: 200.0
    };
}

pub fn lighting(material: &Material, light: &PointLight, position: &Tuple, eye_vector: &Tuple, normal_vector: &Tuple) -> Color {
    let effective_color = material.color.mul(&light.intensity);
    let light_vector = light.position.subtract(&position).normalize();
    let ambient = effective_color.scalar_mul(material.ambient);

    let light_dot_normal = light_vector.dot(&normal_vector);

    let mut diffuse  = Color::new(0.0, 0.0, 0.0);
    let mut specular = Color::new(0.0, 0.0, 0.0);

    if light_dot_normal >= 0.0 {
        diffuse = effective_color.scalar_mul(material.diffuse).scalar_mul(light_dot_normal);

        let reflect_vector = light_vector.negate().reflect(&normal_vector);
        let reflect_dot_eye = reflect_vector.dot(&eye_vector);

        if reflect_dot_eye > 0.0  {
            let factor = f64::powf(reflect_dot_eye, material.shininess);
            specular = light.intensity.scalar_mul(material.specular).scalar_mul(factor);
        }
    }
    ambient.add(&diffuse).add(&specular)
}

#[cfg(test)]
mod tests {
    use crate::{tuple::Tuple, color::Color};
    use super::{Material, PointLight, lighting};

    const m: Material = Material::DEFAULT;
    const p: Tuple = Tuple::point(0.0, 0.0, 0.0);

    #[test]
    fn lighting_eye_between_light_surface() {
        let eye_vector = Tuple::vector(0.0, 0.0, -1.0);
        let normal_vector  = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight { position: Tuple::point(0.0, 0.0, -10.0), intensity: Color::new(1.0, 1.0, 1.0) };
        assert_abs_diff_eq!(lighting(&m, &light, &p, &eye_vector, &normal_vector), Color::new(1.9, 1.9, 1.9));
    }
    #[test]
    fn lighting_eye_between_light_surface_eye_offset_45_deg() {
        let eye_vector = Tuple::vector(0.0, f64::sqrt(2.0)/2.0, -f64::sqrt(2.0)/2.0);
        let normal_vector  = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight { position: Tuple::point(0.0, 0.0, -10.0), intensity: Color::new(1.0, 1.0, 1.0) };
        assert_abs_diff_eq!(lighting(&m, &light, &p, &eye_vector, &normal_vector), Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_eye_opposite_light_surface_eye_offset_45_deg() {
        let eye_vector = Tuple::vector(0.0, 0.0, -1.0);
        let normal_vector  = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight { position: Tuple::point(0.0, 10.0, -10.0), intensity: Color::new(1.0, 1.0, 1.0) };
        assert_abs_diff_eq!(lighting(&m, &light, &p, &eye_vector, &normal_vector), Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_eye_in_reflection_path() {
        let eye_vector = Tuple::vector(0.0, -f64::sqrt(2.0)/2.0, -f64::sqrt(2.0)/2.0);
        let normal_vector  = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight { position: Tuple::point(0.0, 10.0, -10.0), intensity: Color::new(1.0, 1.0, 1.0) };
        assert_abs_diff_eq!(lighting(&m, &light, &p, &eye_vector, &normal_vector), Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_behind_surface() {
        let eye_vector = Tuple::vector(0.0, 0.0, -1.0);
        let normal_vector  = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight { position: Tuple::point(0.0, 0.0, 10.0), intensity: Color::new(1.0, 1.0, 1.0) };
        assert_abs_diff_eq!(lighting(&m, &light, &p, &eye_vector, &normal_vector), Color::new(0.1, 0.1, 0.1));
    }
}