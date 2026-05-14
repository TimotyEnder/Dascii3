use std::{cell::Cell, io::Cursor, vec};

use crate::{
    model::{
        elements::pos3::Pos3,
        objects::{dot::Dot, line::Line},
    },
    screenspace::elements::{
        cell_color::CellColor, drawable::Drawable, screenspace_position::ScreenPosition,
    },
};

pub struct Cube {
    center: Pos3,
    corners: Vec<Pos3>,
    outline_color: CellColor,
    fill_color: Option<CellColor>,
}
impl Cube {
    pub fn from_center(center: &Pos3, scale: usize) -> Self {
        let s = scale as f64;
        Self {
            center: *center,
            corners: vec![
                Pos3::new(&(center.x() + s), &(center.y() + s), &(center.z() + s)),
                Pos3::new(&(center.x() + s), &(center.y() + s), &(center.z() - s)),
                Pos3::new(&(center.x() + s), &(center.y() - s), &(center.z() + s)),
                Pos3::new(&(center.x() + s), &(center.y() - s), &(center.z() - s)),
                Pos3::new(&(center.x() - s), &(center.y() + s), &(center.z() + s)),
                Pos3::new(&(center.x() - s), &(center.y() + s), &(center.z() - s)),
                Pos3::new(&(center.x() - s), &(center.y() - s), &(center.z() + s)),
                Pos3::new(&(center.x() - s), &(center.y() - s), &(center.z() - s)),
            ],
            outline_color: CellColor::WHITE,
            fill_color: None,
        }
    }

    pub fn from_center_with_outline_color(
        center: &Pos3,
        scale: usize,
        outline_color: &CellColor,
    ) -> Self {
        let s = scale as f64;
        Self {
            center: *center,
            corners: vec![
                Pos3::new(&(center.x() + s), &(center.y() + s), &(center.z() + s)),
                Pos3::new(&(center.x() + s), &(center.y() + s), &(center.z() - s)),
                Pos3::new(&(center.x() + s), &(center.y() - s), &(center.z() + s)),
                Pos3::new(&(center.x() + s), &(center.y() - s), &(center.z() - s)),
                Pos3::new(&(center.x() - s), &(center.y() + s), &(center.z() + s)),
                Pos3::new(&(center.x() - s), &(center.y() + s), &(center.z() - s)),
                Pos3::new(&(center.x() - s), &(center.y() - s), &(center.z() + s)),
                Pos3::new(&(center.x() - s), &(center.y() - s), &(center.z() - s)),
            ],
            outline_color: *outline_color,
            fill_color: None,
        }
    }
    pub fn from_center_filled(
        center: &Pos3,
        scale: usize,
        outline_color: &CellColor,
        fill_color: &CellColor,
    ) -> Self {
        let s = scale as f64;
        Self {
            center: *center,
            corners: vec![
                Pos3::new(&(center.x() + s), &(center.y() + s), &(center.z() + s)),
                Pos3::new(&(center.x() + s), &(center.y() + s), &(center.z() - s)),
                Pos3::new(&(center.x() + s), &(center.y() - s), &(center.z() + s)),
                Pos3::new(&(center.x() + s), &(center.y() - s), &(center.z() - s)),
                Pos3::new(&(center.x() - s), &(center.y() + s), &(center.z() + s)),
                Pos3::new(&(center.x() - s), &(center.y() + s), &(center.z() - s)),
                Pos3::new(&(center.x() - s), &(center.y() - s), &(center.z() + s)),
                Pos3::new(&(center.x() - s), &(center.y() - s), &(center.z() - s)),
            ],
            outline_color: *outline_color,
            fill_color: Some(*fill_color),
        }
    }
    pub fn rotate(&mut self, angle_x: &f64, angle_y: &f64, angle_z: &f64) {
        for corner in self.corners.iter_mut() {
            Self::transform_into_center_vector_and_rotate(
                angle_x,
                angle_y,
                angle_z,
                &self.center,
                corner,
            );
        }
    }
    fn transform_into_center_vector_and_rotate(
        angle_x: &f64,
        angle_y: &f64,
        angle_z: &f64,
        center: &Pos3,
        corner: &mut Pos3,
    ) {
        let x = corner.x() - center.x();
        let y = corner.y() - center.y();
        let z = corner.z() - center.z();
        *corner = Pos3::new(&x, &y, &z);
        corner.rotate(angle_x, angle_y, angle_z);
        let x = corner.x() + center.x();
        let y = corner.y() + center.y();
        let z = corner.z() + center.z();
        *corner = Pos3::new(&x, &y, &z);
    }
    fn draw_face(
        top_left: &ScreenPosition,
        top_right: &ScreenPosition,
        bottom_right: &ScreenPosition,
        bottom_left: &ScreenPosition,
        other_colored_cells: &Vec<ScreenPosition>,
        fill_color: &CellColor,
    ) -> Vec<ScreenPosition> {
        let colored_cells = Vec::new();
        let top_point = if top_left.y() > top_right.y() {
            top_left
        } else {
            top_right
        };
        let bottom_point = if bottom_left.y() < bottom_right.y() {
            bottom_left
        } else {
            bottom_right
        };
        let mut y = top_point.y();
        while (y >= bottom_point.y()) {
            y -= 1;
        }
        colored_cells
    }
}
impl Drawable for Cube {
    fn draw(&self, screen: &mut crate::screenspace::screen::screen::Screen) -> Vec<ScreenPosition> {
        let mut colored_cells = Vec::new();
        let edges = [
            // back face (z = cz+s): 0--2, 2--6, 6--4, 4--0
            (0, 2),
            (2, 6),
            (6, 4),
            (4, 0),
            // front face (z = cz-s): 1--3, 3--7, 7--5, 5--1
            (1, 3),
            (3, 7),
            (7, 5),
            (5, 1),
            // connectors between front and back
            (0, 1),
            (2, 3),
            (6, 7),
            (4, 5),
        ];
        let faces = [
            // Back face (z = cz+s)
            (4, 6, 2, 0), // top-left, top-right, bottom-right, bottom-left
            // Front face (z = cz-s)
            (5, 7, 3, 1), // top-left, top-right, bottom-right, bottom-left
            // Left face (connecting 4-0 and 5-1)
            (4, 0, 1, 5), // back-top-left, back-bottom-left, front-bottom-left, front-top-left
            // Right face (connecting 6-2 and 7-3)
            (6, 2, 3, 7), // back-top-right, back-bottom-right, front-bottom-right, front-top-right
            // Top face
            (4, 6, 7, 5), // back-top-left, back-top-right, front-top-right, front-top-left
            // Bottom face
            (0, 2, 3, 1), // back-bottom-left, back-bottom-right, front-bottom-right, front-bottom-left
        ];

        for &(from, to) in &edges {
            let line = Line::from_to(&self.corners[from], &self.corners[to]);
            for cell in line.draw(screen) {
                colored_cells.push(cell);
            }
        }
        if let Some(color) = self.fill_color {
            for &(top_left, top_right, bottom_right, bottom_left) in &faces {
                for cell in Self::draw_face(
                    &screen.project_point(&self.corners[top_left]),
                    &screen.project_point(&self.corners[top_right]),
                    &screen.project_point(&self.corners[bottom_right]),
                    &screen.project_point(&self.corners[bottom_left]),
                    &colored_cells,
                    &color,
                ) {
                    colored_cells.push(cell);
                }
            }
        }
        colored_cells
    }

    fn position(&self) -> Pos3 {
        self.center
    }
}
