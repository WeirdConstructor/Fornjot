use crate::{
    debug::DebugInfo,
    kernel::{
        shape::{edges::Edges, Shape},
        topology::faces::{Face, Faces},
    },
    math::{Aabb, Scalar},
};

use super::ToShape;

impl ToShape for fj::Difference2d {
    fn to_shape(&self, tolerance: Scalar, debug_info: &mut DebugInfo) -> Shape {
        // This method assumes that `b` is fully contained within `a`:
        // https://github.com/hannobraun/Fornjot/issues/92

        let mut shape = Shape::new();

        let mut a = self.a.to_shape(tolerance, debug_info);
        let mut b = self.b.to_shape(tolerance, debug_info);

        *shape.edges() = {
            let (a, b) =
                if a.edges().cycles.len() == 1 && b.edges().cycles.len() == 1 {
                    (
                        a.edges().cycles.pop().unwrap(),
                        b.edges().cycles.pop().unwrap(),
                    )
                } else {
                    // See issue:
                    // https://github.com/hannobraun/Fornjot/issues/95
                    todo!(
                    "The 2-dimensional difference operation only supports one \
                    cycle in each operand."
                );
                };

            Edges { cycles: vec![a, b] }
        };

        shape.faces = {
            let (a, b) = if a.faces.0.len() == 1 && b.faces.0.len() == 1 {
                // Can't panic. We just checked that length of `a` and `b` is 1.
                (a.faces.0.pop().unwrap(), b.faces.0.pop().unwrap())
            } else {
                // See issue:
                // https://github.com/hannobraun/Fornjot/issues/95
                todo!(
                    "The 2-dimensional difference operation only supports one \
                    face in each operand."
                );
            };

            let (a, b, surface_a, surface_b) = match (a, b) {
                (
                    Face::Face {
                        edges: a,
                        surface: surface_a,
                    },
                    Face::Face {
                        edges: b,
                        surface: surface_b,
                    },
                ) => (a, b, surface_a, surface_b),
                _ => {
                    // None of the 2D types still use the triangles representation.
                    unreachable!()
                }
            };

            if surface_a != surface_b {
                // Panicking is not great, but as long as we don't have a real
                // error handling mechanism, it will do.
                panic!("Trying to subtract sketches with different surfaces.")
            }
            let surface = surface_a;

            let mut edges = a;
            edges.cycles.extend(b.cycles);

            Faces(vec![Face::Face { edges, surface }])
        };

        shape
    }

    fn bounding_volume(&self) -> Aabb<3> {
        // This is a conservative estimate of the bounding box: It's never going
        // to be bigger than the bounding box of the original shape that another
        // is being subtracted from.
        self.a.bounding_volume()
    }
}
