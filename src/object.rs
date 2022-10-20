use crate::{
    lighting::Material, matrix4::Matrix4, plane::plane_object_intersect,
    sphere::sphere_object_intersect, tuple::Tuple,
};

#[derive(Clone, Copy)]
pub enum ObjectType {
    Sphere,
    Plane,
}

#[derive(Clone, Copy)]
pub struct Object {
    pub object_type: ObjectType,
    pub material: Material,
    pub transform: Matrix4,
}

impl Object {
    pub const SPHERE: Object = Object {
        object_type: ObjectType::Sphere,
        material: Material::DEFAULT,
        transform: Matrix4::IDENTITY,
    };
    pub const PLANE: Object = Object {
        object_type: ObjectType::Plane,
        material: Material::DEFAULT,
        transform: Matrix4::IDENTITY,
    };

    pub fn normal_at(&self, world_point: &Tuple) -> Tuple {
        let inv_transform = self.transform.inverse();

        let object_point = inv_transform.mul_tuple(&world_point);
        let object_normal = self.object_normal(&object_point);
        let world_normal = inv_transform.transpose().mul_tuple(&object_normal);
        Tuple::vector(world_normal.x, world_normal.y, world_normal.z).normalize()
    }

    pub fn object_normal(&self, object_point: &Tuple) -> Tuple {
        match self.object_type {
            ObjectType::Sphere => object_point
                .subtract(&Tuple::point(0.0, 0.0, 0.0))
                .normalize(),
            ObjectType::Plane => Tuple::vector(0.0, 1.0, 0.0),
        }
    }

    pub fn object_intersect(&self, ray: &crate::ray::Ray) -> crate::ray::Intersections {
        match self.object_type {
            ObjectType::Sphere => sphere_object_intersect(self, ray),
            ObjectType::Plane => plane_object_intersect(self, ray),
        }
    }
}
