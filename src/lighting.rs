use crate::{color::Color, object::Object, pattern::Pattern, tuple::Tuple};

#[derive(Copy, Clone)]
pub struct PointLight {
    pub position: Tuple,
    pub intensity: Color,
}

#[derive(Clone, Copy)]
pub struct Material {
    pub pattern: Pattern,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub const DEFAULT_AMBIENT: f64 = 0.1;
    pub const DEFAULT_SPECULAR: f64 = 0.9;
    pub const DEFAULT_DIFFUSE: f64 = 0.9;
    pub const DEFAULT_SHININESS: f64 = 200.0;

    pub const DEFAULT: Material = Material {
        pattern: Pattern::WHITE,
        ambient: Material::DEFAULT_AMBIENT,
        diffuse: Material::DEFAULT_DIFFUSE,
        specular: Material::DEFAULT_SPECULAR,
        shininess: Material::DEFAULT_SHININESS,
    };
}

pub fn lighting(
    object: &Object,
    light: &PointLight,
    position: &Tuple,
    eye_vector: &Tuple,
    normal_vector: &Tuple,
    is_in_shadow: bool,
) -> Color {
    let effective_color = object
        .material
        .pattern
        .color_at(object, &position)
        .mul(&light.intensity);
    let ambient = effective_color.scalar_mul(object.material.ambient);

    if is_in_shadow {
        return ambient;
    }

    let light_vector = light.position.subtract(&position).normalize();
    let light_dot_normal = light_vector.dot(&normal_vector);

    let mut diffuse = Color::new(0.0, 0.0, 0.0);
    let mut specular = Color::new(0.0, 0.0, 0.0);

    if light_dot_normal >= 0.0 {
        diffuse = effective_color
            .scalar_mul(object.material.diffuse)
            .scalar_mul(light_dot_normal);

        let reflect_vector = light_vector.negate().reflect(&normal_vector);
        let reflect_dot_eye = reflect_vector.dot(&eye_vector);

        if reflect_dot_eye > 0.0 {
            let factor = f64::powf(reflect_dot_eye, object.material.shininess);
            specular = light
                .intensity
                .scalar_mul(object.material.specular)
                .scalar_mul(factor);
        }
    }

    ambient.add(&diffuse).add(&specular)
}

#[cfg(test)]
mod tests {
    use super::{lighting, Material, PointLight};
    use crate::{
        color::Color,
        matrix4::Matrix4,
        object::{Object, ObjectType},
        pattern::Pattern,
        tuple::Tuple,
    };

    const POINT: Tuple = Tuple::point(0.0, 0.0, 0.0);

    #[test]
    fn lighting_eye_between_light_surface() {
        let eye_vector = Tuple::vector(0.0, 0.0, -1.0);
        let normal_vector = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight {
            position: Tuple::point(0.0, 0.0, -10.0),
            intensity: Color::WHITE,
        };
        assert_abs_diff_eq!(
            lighting(
                &Object::SPHERE,
                &light,
                &POINT,
                &eye_vector,
                &normal_vector,
                false
            ),
            Color::new(1.9, 1.9, 1.9)
        );
    }

    #[test]
    fn lighting_eye_between_light_surface_shadow() {
        let eye_vector = Tuple::vector(0.0, 0.0, -1.0);
        let normal_vector = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight {
            position: Tuple::point(0.0, 0.0, -10.0),
            intensity: Color::WHITE,
        };
        assert_abs_diff_eq!(
            lighting(
                &Object::SPHERE,
                &light,
                &POINT,
                &eye_vector,
                &normal_vector,
                true
            ),
            Color::new(0.1, 0.1, 0.1)
        );
    }

    #[test]
    fn lighting_eye_between_light_surface_eye_offset_45_deg() {
        let eye_vector = Tuple::vector(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0);
        let normal_vector = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight {
            position: Tuple::point(0.0, 0.0, -10.0),
            intensity: Color::WHITE,
        };
        assert_abs_diff_eq!(
            lighting(
                &Object::SPHERE,
                &light,
                &POINT,
                &eye_vector,
                &normal_vector,
                false
            ),
            Color::WHITE
        );
    }

    #[test]
    fn lighting_eye_opposite_light_surface_eye_offset_45_deg() {
        let eye_vector = Tuple::vector(0.0, 0.0, -1.0);
        let normal_vector = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight {
            position: Tuple::point(0.0, 10.0, -10.0),
            intensity: Color::WHITE,
        };
        assert_abs_diff_eq!(
            lighting(
                &Object::SPHERE,
                &light,
                &POINT,
                &eye_vector,
                &normal_vector,
                false
            ),
            Color::new(0.7364, 0.7364, 0.7364)
        );
    }

    #[test]
    fn lighting_eye_in_reflection_path() {
        let eye_vector = Tuple::vector(0.0, -f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0);
        let normal_vector = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight {
            position: Tuple::point(0.0, 10.0, -10.0),
            intensity: Color::WHITE,
        };
        assert_abs_diff_eq!(
            lighting(
                &Object::SPHERE,
                &light,
                &POINT,
                &eye_vector,
                &normal_vector,
                false
            ),
            Color::new(1.6364, 1.6364, 1.6364)
        );
    }

    #[test]
    fn lighting_behind_surface() {
        let eye_vector = Tuple::vector(0.0, 0.0, -1.0);
        let normal_vector = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight {
            position: Tuple::point(0.0, 0.0, 10.0),
            intensity: Color::WHITE,
        };
        assert_abs_diff_eq!(
            lighting(
                &Object::SPHERE,
                &light,
                &POINT,
                &eye_vector,
                &normal_vector,
                false
            ),
            Color::new(0.1, 0.1, 0.1)
        );
    }

    #[test]
    fn lighting_pattern() {
        let mat = Material {
            pattern: Pattern::Stripe {
                a: Color::WHITE,
                b: Color::BLACK,
                transform: Matrix4::IDENTITY,
            },
            ambient: 1.0,
            diffuse: 0.0,
            specular: 0.0,
            shininess: Material::DEFAULT_SHININESS,
        };

        let s = Object {
            object_type: ObjectType::Sphere,
            material: mat,
            transform: Matrix4::IDENTITY,
        };
        let light = PointLight {
            position: Tuple::point(0.0, 0.0, -10.0),
            intensity: Color::WHITE,
        };

        let eye = Tuple::vector(0.0, 0.0, -1.0);
        let normal = Tuple::vector(0.0, 0.0, -1.0);
        let c1 = lighting(
            &s,
            &light,
            &Tuple::point(0.9, 0.0, 0.0),
            &eye,
            &normal,
            false,
        );
        let c2 = lighting(
            &s,
            &light,
            &Tuple::point(1.0, 0.0, 0.0),
            &eye,
            &normal,
            false,
        );

        assert_abs_diff_eq!(c1, Color::WHITE);
        assert_abs_diff_eq!(c2, Color::BLACK);
    }
}
