use std::{cell::Cell, collections::HashSet, hash::Hash, vec};

use crate::{model::elements::pos3::Pos3, screenspace::elements::cell_color::CellColor};

pub struct Mesh {
    pub vertices: Vec<Pos3>,
    pub edges: Vec<(usize, usize)>,
    pub faces: Vec<((usize, usize, usize), Option<CellColor>)>,
    pub center: Pos3,
    pub out_line_color: CellColor,
}
impl Mesh {
    pub fn empty() -> Self {
        Mesh {
            vertices: Vec::new(),
            edges: Vec::new(),
            faces: Vec::new(),
            center: Pos3::default(),
            out_line_color: CellColor::WHITE,
        }
    }
    pub fn dot(pos: &Pos3) -> Self {
        Mesh {
            vertices: Vec::from(vec![*pos]),
            edges: Vec::new(),
            faces: Vec::new(),
            center: *pos,
            out_line_color: CellColor::WHITE,
        }
    }
    pub fn line(from: &Pos3, to: &Pos3) -> Self {
        Mesh {
            vertices: vec![*from, *to],
            edges: vec![(0, 1)],
            faces: Vec::new(),
            center: mid_point_in_line(from, to),
            out_line_color: CellColor::WHITE,
        }
    }
    pub fn cube(
        center: &Pos3,
        x_size: f64,
        y_size: f64,
        z_size: f64,
        fill_color: Option<CellColor>,
    ) -> Self {
        let half_x = x_size / 2.0;
        let half_y = y_size / 2.0;
        let half_z = z_size / 2.0;

        Mesh {
            vertices: vec![
                // 0: Front-top-right (x+, y+, z+)
                Pos3::new(
                    center.x() + half_x,
                    center.y() + half_y,
                    center.z() + half_z,
                ),
                // 1: Back-top-right (x+, y+, z-)
                Pos3::new(
                    center.x() + half_x,
                    center.y() + half_y,
                    center.z() - half_z,
                ),
                // 2: Front-bottom-right (x+, y-, z+)
                Pos3::new(
                    center.x() + half_x,
                    center.y() - half_y,
                    center.z() + half_z,
                ),
                // 3: Back-bottom-right (x+, y-, z-)
                Pos3::new(
                    center.x() + half_x,
                    center.y() - half_y,
                    center.z() - half_z,
                ),
                // 4: Front-top-left (x-, y+, z+)
                Pos3::new(
                    center.x() - half_x,
                    center.y() + half_y,
                    center.z() + half_z,
                ),
                // 5: Back-top-left (x-, y+, z-)
                Pos3::new(
                    center.x() - half_x,
                    center.y() + half_y,
                    center.z() - half_z,
                ),
                // 6: Front-bottom-left (x-, y-, z+)
                Pos3::new(
                    center.x() - half_x,
                    center.y() - half_y,
                    center.z() + half_z,
                ),
                // 7: Back-bottom-left (x-, y-, z-)
                Pos3::new(
                    center.x() - half_x,
                    center.y() - half_y,
                    center.z() - half_z,
                ),
            ],
            edges: vec![
                (4, 6),
                (6, 2),
                (2, 0),
                (0, 4), // Front face (z+)
                (5, 7),
                (7, 3),
                (3, 1),
                (1, 5), // Back face (z-)
                (4, 0),
                (0, 1),
                (1, 5),
                (5, 4), // Top face (y+)
                (6, 2),
                (2, 3),
                (3, 7),
                (7, 6), // Bottom face (y-)
                (4, 6),
                (6, 7),
                (7, 5),
                (5, 4), // Left face (x-)
                (0, 2),
                (2, 3),
                (3, 1),
                (1, 0), // Right face (x+)
            ],
            faces: vec![
                // Front face (z+) - split into two triangles
                ((4, 6, 2), fill_color),
                ((4, 2, 0), fill_color),
                // Back face (z-) - split into two triangles
                ((5, 7, 3), fill_color),
                ((5, 3, 1), fill_color),
                // Top face (y+) - split into two triangles
                ((4, 0, 1), fill_color),
                ((4, 1, 5), fill_color),
                // Bottom face (y-) - split into two triangles
                ((6, 2, 3), fill_color),
                ((6, 3, 7), fill_color),
                // Left face (x-) - split into two triangles
                ((4, 6, 7), fill_color),
                ((4, 7, 5), fill_color),
                // Right face (x+) - split into two triangles
                ((0, 2, 3), fill_color),
                ((0, 3, 1), fill_color),
            ],
            center: *center,
            out_line_color: CellColor::WHITE,
        }
    }
    pub fn custom_polygon(
        points: Vec<Pos3>,
        edges: Vec<(usize, usize)>,
        faces: Vec<((usize, usize, usize), Option<CellColor>)>,
        center: &Pos3,
    ) -> Self {
        Mesh {
            vertices: points,
            edges: edges,
            faces: faces,
            center: *center,
            out_line_color: CellColor::WHITE,
        }
    }
    pub fn rotate(&mut self, angle_x: f64, angle_y: f64, angle_z: f64) {
        for corner in self.vertices.iter_mut() {
            corner.rotate_around_pivot(angle_x, angle_y, angle_z, &self.center);
        }
    }
    pub fn translate(&mut self, point: &Pos3) {
        let vector = (
            point.x() - self.center.x(),
            point.y() - self.center.y(),
            point.z() - self.center.z(),
        );
        self.center = Pos3::from(*point);
        for vertex in self.vertices.iter_mut() {
            vertex.translate(vector);
        }
    }
}
fn mid_point_in_line(from: &Pos3, to: &Pos3) -> Pos3 {
    Pos3::new(
        (from.x() + to.x()) / 2.0,
        (from.y() + to.y()) / 2.0,
        (from.z() + to.z()) / 2.0,
    )
}
