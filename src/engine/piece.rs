use std::ops::{Mul, Neg};

use crate::engine::{Coord, Offset};
use cgmath::{ElementWise, EuclideanSpace, Vector2, Zero};

use super::Matrix;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Piece {
    pub kind: Kind,
    pub position: Offset,
    pub rotation: Rotation,
}

impl Piece {
    pub const CELL_COUNT: usize = 4;

    pub fn moved_by(&self, offset: Offset) -> Self {
        Self {
            position: self.position + offset,
            ..*self
        }
    }

    pub fn cells(&self) -> Option<[Coord; Self::CELL_COUNT]> {
        let offsets = self.kind.cells().map(self.rotator()).map(self.trasnlator());
        let mut coords = [Coord::origin(); Self::CELL_COUNT];
        for (offset, coord_slot) in offsets.into_iter().zip(&mut coords) {
            let positive_offset = offset.cast::<usize>()?;
            let coord = Coord::from_vec(positive_offset);
            if Matrix::valid_coord(coord) {
                *coord_slot = coord;
            } else {
                return None;
            }
        }
        Some(coords)
    }

    fn rotator(&self) -> impl Fn(Offset) -> Offset + '_ {
        |cell| match self.kind {
            Kind::O => cell,
            _ => {
                cell * self.rotation
                    + self
                        .rotation
                        .intrinsic_offset()
                        .mul_element_wise(self.kind.local_grid_size())
            }
        }
    }

    fn trasnlator(&self) -> impl Fn(Offset) -> Offset {
        let position = self.position;
        move |cell| cell + position
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    O,
    I,
    T,
    L,
    J,
    S,
    Z,
}

impl Kind {
    pub const ALL: [Self; 7] = [
        Self::O,
        Self::I,
        Self::T,
        Self::L,
        Self::J,
        Self::S,
        Self::Z,
    ];

    pub fn cells(&self) -> [Offset; Piece::CELL_COUNT] {
        match self {
            Kind::O => &[(1, 1), (1, 2), (2, 1), (2, 2)],
            Kind::I => &[(0, 2), (1, 2), (2, 2), (3, 2)],
            Kind::T => &[(0, 1), (1, 1), (2, 1), (1, 2)],
            Kind::L => &[(0, 1), (1, 1), (2, 1), (2, 2)],
            Kind::J => &[(0, 2), (0, 1), (1, 1), (2, 1)],
            Kind::S => &[(0, 1), (1, 1), (1, 2), (2, 2)],
            Kind::Z => &[(0, 2), (1, 2), (1, 1), (2, 1)],
        }
        .map(Vector2::from)
    }

    fn local_grid_size(&self) -> isize {
        match self {
            Self::I => 4,
            _ => 3,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Rotation {
    N,
    E,
    S,
    W,
}

impl Rotation {
    fn intrinsic_offset(&self) -> Offset {
        match self {
            Rotation::N => Offset::zero(),
            Rotation::E => Offset::unit_y(),
            Rotation::S => Offset::new(1, 1),
            Rotation::W => Offset::unit_x(),
        }
    }
}

impl<S> Mul<Rotation> for Vector2<S>
where
    S: Neg<Output = S>,
{
    type Output = Self;
    fn mul(self, rhs: Rotation) -> Self::Output {
        match rhs {
            Rotation::N => self,
            Rotation::E => Self::new(self.y, -self.x),
            Rotation::S => Self::new(-self.x, -self.y),
            Rotation::W => Self::new(-self.y, self.x),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s_piece_positioning() {
        let s = Piece {
            kind: Kind::S,
            position: Offset::new(5, 6),
            rotation: Rotation::W,
        };
        assert_eq!(
            s.cells(),
            Some([(7, 6), (7, 7), (6, 7), (6, 8)].map(Coord::from))
        )
    }
}
