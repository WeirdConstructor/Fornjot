use crate::{
    kernel::{
        geometry::{Circle, Curve, Line},
        topology::{
            edges::{Cycle, Edge},
            vertices::Vertex,
        },
    },
    math::{Point, Scalar, Vector},
};

/// The edges of a shape
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Edges {
    /// The cycles that the edges of the shape form
    ///
    /// Code reading this field generally assumes that cycles do not overlap.
    /// This precondition is currently not checked, and must be upheld by all
    /// code writing to this field.
    pub cycles: Vec<Cycle>,
}

impl Edges {
    /// Construct a new instance of `Edges`, with a single cycle
    pub fn single_cycle(edges: impl IntoIterator<Item = Edge>) -> Self {
        let cycle = Cycle {
            edges: edges.into_iter().collect(),
        };

        Self {
            cycles: vec![cycle],
        }
    }

    /// Create an edge
    ///
    /// If vertices are provided in `vertices`, they must be on `curve`.
    ///
    /// This constructor will convert the vertices into curve coordinates. If
    /// they are not on the curve, this will result in their projection being
    /// converted into curve coordinates, which is likely not the caller's
    /// intention.
    ///
    /// # Implementation notes
    ///
    /// Right now this is just an overly complicated constructor for `Edge`. In
    /// the future, it can add the edge to the proper internal data structures,
    /// and validate any constraints that apply to edge creation.
    pub fn create(
        &mut self,
        curve: Curve,
        vertices: Option<[Vertex; 2]>,
    ) -> Edge {
        Edge { curve, vertices }
    }

    /// Create a line segment
    ///
    /// Calls [`Edges::create`] internally, and inherits its limitations and
    /// requirements.
    pub fn create_line_segment(&mut self, vertices: [Vertex; 2]) -> Edge {
        self.create(
            Curve::Line(Line::from_points(
                vertices.clone().map(|vertex| vertex.point()),
            )),
            Some(vertices),
        )
    }

    /// Create a circle
    ///
    /// Calls [`Edges::create`] internally, and inherits its limitations and
    /// requirements.
    pub fn create_circle(&mut self, radius: Scalar) -> Edge {
        self.create(
            Curve::Circle(Circle {
                center: Point::origin(),
                radius: Vector::from([radius, Scalar::ZERO]),
            }),
            None,
        )
    }
}
