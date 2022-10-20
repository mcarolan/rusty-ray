use crate::{color::Color, matrix4::Matrix4, tuple::Tuple, object::Object};

#[derive(Clone, Copy)]
pub enum Pattern {
    Stripe {
        a: Color,
        b: Color,
        transform: Matrix4,
    },
    Gradient {
        a: Color,
        b: Color,
        transform: Matrix4,
    },
    Ring {
        a: Color,
        b: Color,
        transform: Matrix4
    },
    Checker {
        a: Color,
        b: Color,
        transform: Matrix4
    },
    Constant {
        value: Color,
    },
}

impl Pattern {
    pub const WHITE: Pattern =  Pattern::Constant { value: Color::WHITE };

    fn pattern_color_at(&self, point: &Tuple) -> Color {
        match self {
            Pattern::Stripe { a, b, transform: _ } => {
                if point.x.floor() % 2.0 == 0.0 {
                    *a
                } else {
                    *b
                }
            }
            Pattern::Gradient { a, b, transform: _ } => {
                let distance = b.subtract(&a);
                let fraction = point.x - point.x.floor();
                a.add(&distance.scalar_mul(fraction))
            },
            Pattern::Constant { value } => *value,
            Pattern::Ring { a, b, transform: _ } => {
                if (point.x.powf(2.0) + point.z.powf(2.0)).floor() % 2.0 == 0.0 {
                    *a
                }
                else {
                    *b
                }
            },
            Pattern::Checker { a, b, transform: _ } => {
                if (point.x.floor() +  point.y.floor() +  point.z.floor()) % 2.0 == 0.0 {
                    *a
                }
                else {
                    *b
                }
            },
        }
    }

    pub fn color_at(&self, object: &Object, point: &Tuple) -> Color {
        let object_point = object.transform.inverse().mul_tuple(point);
        let pattern_transform =
            match self {
                Pattern::Stripe { a: _, b: _, transform } => *transform,
                Pattern::Gradient { a: _, b: _, transform } => *transform,
                Pattern::Constant { value: _ } => Matrix4::IDENTITY,
                Pattern::Ring { a: _, b: _, transform } => *transform,
                Pattern::Checker { a: _, b: _, transform } => *transform,
            };
        let pattern_point = pattern_transform.inverse().mul_tuple(&object_point);
        self.pattern_color_at(&pattern_point)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        color::Color,
        matrix4::Matrix4,
        transform::{scaling, translation},
        tuple::Tuple,
        pattern::Pattern, object::{Object, ObjectType}, lighting::Material
    };

    #[test]
    fn stripe_constant_y() {
        let p = Pattern::Stripe {
            a: Color::WHITE,
            b: Color::BLACK,
            transform: Matrix4::IDENTITY,
        };

        assert_abs_diff_eq!(
            p.pattern_color_at(&Tuple::point(0.0, 0.0, 0.0)),
            Color::WHITE
        );
        assert_abs_diff_eq!(
            p.pattern_color_at(&Tuple::point(0.0, 1.0, 0.0)),
            Color::WHITE
        );
        assert_abs_diff_eq!(
            p.pattern_color_at(&Tuple::point(0.0, 2.0, 0.0)),
            Color::WHITE
        );
    }

    #[test]
    fn stripe_constant_z() {
        let p = Pattern::Stripe {
            a: Color::WHITE,
            b: Color::BLACK,
            transform: Matrix4::IDENTITY,
        };

        assert_abs_diff_eq!(
            p.pattern_color_at(&Tuple::point(0.0, 0.0, 0.0)),
            Color::WHITE
        );
        assert_abs_diff_eq!(
            p.pattern_color_at(&Tuple::point(0.0, 0.0, 1.0)),
            Color::WHITE
        );
        assert_abs_diff_eq!(
            p.pattern_color_at(&Tuple::point(0.0, 0.0, 2.0)),
            Color::WHITE
        );
    }

    #[test]
    fn stripe_alternates_x() {
        let p = Pattern::Stripe {
            a: Color::WHITE,
            b: Color::BLACK,
            transform: Matrix4::IDENTITY,
        };

        assert_abs_diff_eq!(
            p.pattern_color_at(&Tuple::point(0.0, 0.0, 0.0)),
            Color::WHITE
        );
        assert_abs_diff_eq!(
            p.pattern_color_at(&Tuple::point(0.9, 0.0, 0.0)),
            Color::WHITE
        );
        assert_abs_diff_eq!(
            p.pattern_color_at(&Tuple::point(1.0, 0.0, 0.0)),
            Color::BLACK
        );
        assert_abs_diff_eq!(
            p.pattern_color_at(&Tuple::point(-0.1, 0.0, 0.0)),
            Color::BLACK
        );
        assert_abs_diff_eq!(
            p.pattern_color_at(&Tuple::point(-1.0, 0.0, 0.0)),
            Color::BLACK
        );
        assert_abs_diff_eq!(
            p.pattern_color_at(&Tuple::point(-1.1, 0.0, 0.0)),
            Color::WHITE
        );
    }

    #[test]
    fn stripe_object_transform() {
        let p = Pattern::Stripe {
            a: Color::WHITE,
            b: Color::BLACK,
            transform: Matrix4::IDENTITY,
        };
        let s = Object {
            object_type: ObjectType::Sphere,
            material: Material::DEFAULT,
            transform: scaling(2.0, 2.0, 2.0)
        };
        assert_abs_diff_eq!(
            p.color_at(&s, &Tuple::point(1.5, 0.0, 0.0)),
            Color::WHITE
        );
    }

    #[test]
    fn stripe_pattern_transform() {
        let p = Pattern::Stripe {
            a: Color::WHITE,
            b: Color::BLACK,
            transform: scaling(2.0, 2.0, 2.0),
        };

        assert_abs_diff_eq!(
            p.color_at(&Object::SPHERE, &Tuple::point(1.5, 0.0, 0.0)),
            Color::WHITE
        );
    }

    #[test]
    fn stripe_object_pattern_transform() {
        let p = Pattern::Stripe {
            a: Color::WHITE,
            b: Color::BLACK,
            transform: translation(0.5, 2.0, 2.0),
        };
        let s = Object {
            object_type: ObjectType::Sphere,
            material: Material::DEFAULT,
            transform: scaling(2.0, 2.0, 2.0)
        };

        assert_abs_diff_eq!(
            p.color_at(&s, &Tuple::point(2.5, 0.0, 0.0)),
            Color::WHITE
        );
    }

    #[test]
    fn gradient_pattern() {
        let pattern = Pattern::Gradient {
            a: Color::WHITE,
            b: Color::BLACK,
            transform: Matrix4::IDENTITY,
        };

        assert_abs_diff_eq!(
            pattern.pattern_color_at(&Tuple::point(0.0, 0.0, 0.0)),
            Color::WHITE
        );
        assert_abs_diff_eq!(
            pattern.pattern_color_at(&Tuple::point(0.25, 0.0, 0.0)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_abs_diff_eq!(
            pattern.pattern_color_at(&Tuple::point(0.5, 0.0, 0.0)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_abs_diff_eq!(
            pattern.pattern_color_at(&Tuple::point(0.75, 0.0, 0.0)),
            Color::new(0.25, 0.25, 0.25)
        );
    }

    #[test]
    fn ring_pattern() {
        let pattern = Pattern::Ring { a: Color::WHITE, b: Color::BLACK, transform: Matrix4::IDENTITY };
        
        assert_abs_diff_eq!(pattern.pattern_color_at(&Tuple::point(0.0, 0.0, 0.0)), Color::WHITE);
        assert_abs_diff_eq!(pattern.pattern_color_at(&Tuple::point(1.0, 0.0, 0.0)), Color::BLACK);
        assert_abs_diff_eq!(pattern.pattern_color_at(&Tuple::point(0.0, 0.0, 1.0)), Color::BLACK);
        assert_abs_diff_eq!(pattern.pattern_color_at(&Tuple::point(0.708, 0.0, 0.708)), Color::BLACK);
    }

    #[test]
    fn checker_pattern() {
        let pattern = Pattern::Checker { a: Color::WHITE, b: Color::BLACK, transform: Matrix4::IDENTITY };

        assert_abs_diff_eq!(pattern.pattern_color_at(&Tuple::point(0.0, 0.0, 0.0)), Color::WHITE);
        assert_abs_diff_eq!(pattern.pattern_color_at(&Tuple::point(0.99, 0.0, 0.0)), Color::WHITE);
        assert_abs_diff_eq!(pattern.pattern_color_at(&Tuple::point(1.01, 0.0, 0.0)), Color::BLACK);

        assert_abs_diff_eq!(pattern.pattern_color_at(&Tuple::point(0.0, 0.0, 0.0)), Color::WHITE);
        assert_abs_diff_eq!(pattern.pattern_color_at(&Tuple::point(0.0, 0.99, 0.0)), Color::WHITE);
        assert_abs_diff_eq!(pattern.pattern_color_at(&Tuple::point(0.0, 1.01, 0.0)), Color::BLACK);

        assert_abs_diff_eq!(pattern.pattern_color_at(&Tuple::point(0.0, 0.0, 0.0)), Color::WHITE);
        assert_abs_diff_eq!(pattern.pattern_color_at(&Tuple::point(0.0, 0.0, 0.99)), Color::WHITE);
        assert_abs_diff_eq!(pattern.pattern_color_at(&Tuple::point(0.0, 0.0, 1.01)), Color::BLACK);

    }
}
