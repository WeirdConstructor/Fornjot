pub mod edges;
pub mod handle;
pub mod vertices;

use kiddo::KdTree;

use crate::math::{Point, Scalar};

use super::topology::faces::Faces;

use self::{edges::Edges, handle::HandleInner, vertices::Vertices};

/// The boundary representation of a shape
///
/// # Implementation note
///
/// The goal for `Shape` is to enforce full self-consistency, through the API it
/// provides. Steps have been made in that direction, but right now, the API is
/// still full of holes, forcing callers to just be careful for the time being.
#[derive(Clone, Debug)]
pub struct Shape {
    vertices: VerticesInner,
    edges: Edges,

    pub faces: Faces,
}

impl Shape {
    /// Construct a new shape
    pub fn new() -> Self {
        Self {
            vertices: VerticesInner::new(),
            edges: Edges { cycles: Vec::new() },
            faces: Faces(Vec::new()),
        }
    }

    /// Access the shape's vertices
    pub fn vertices(&mut self) -> Vertices {
        Vertices {
            vertices: &mut self.vertices,
        }
    }

    /// Access the shape's edges
    pub fn edges(&mut self) -> &mut Edges {
        &mut self.edges
    }
}

type VerticesInner = KdTree<Scalar, HandleInner<Point<3>>, 3>;
