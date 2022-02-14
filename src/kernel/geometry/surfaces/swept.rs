use parry3d_f64::math::Isometry;

use crate::{
    kernel::geometry::Curve,
    math::{Point, Vector},
};

/// A surface that was swept from a curve
#[derive(Clone, Debug, PartialEq)]
pub struct Swept {
    /// The curve that this surface was swept from
    pub curve: Curve,

    /// The path that the curve was swept along
    ///
    /// Currently, only sweeps along the z-axis are supported.
    pub path: Vector<1>,
}

impl Swept {
    /// Transform the surface
    #[must_use]
    pub fn transform(self, _transform: &Isometry<f64>) -> Self {
        // TASK: Implement.
        todo!()
    }

    /// Convert a point in model coordinates to surface coordinates
    pub fn point_model_to_surface(&self, _point: Point<3>) -> Point<2> {
        // TASK: Implement.
        todo!()
    }

    /// Convert a point in surface coordinates to model coordinates
    pub fn point_surface_to_model(&self, _point: &Point<2>) -> Point<3> {
        // TASK: Implement.
        todo!()
    }

    /// Convert a vector in surface coordinates to model coordinates
    pub fn vector_surface_to_model(&self, _vector: &Vector<2>) -> Vector<3> {
        // TASK: Implement.
        todo!()
    }
}

// TASK: Add test suite.
