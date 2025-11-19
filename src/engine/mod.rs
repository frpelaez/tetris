mod piece;

use std::ops::{Index, IndexMut};

use cgmath::{Point2, Vector2};
use piece::{Kind as PieceKind, Piece};
use rand::{rng, rngs::ThreadRng, seq::SliceRandom};

type Coord = Point2<usize>;
type Offset = Vector2<isize>;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Move {
    Left,
    Right,
}

impl Move {
    fn offset(&self) -> Offset {
        match self {
            Self::Left => -Offset::unit_x(),
            Self::Right => Offset::unit_x(),
        }
    }
}

pub struct Engine {
    matrix: Matrix,
    bag: Vec<PieceKind>,
    rng: ThreadRng,
    cursor: Option<Piece>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            matrix: Matrix::blank(),
            bag: Vec::new(),
            rng: rng(),
            cursor: None,
        }
    }

    fn refill_bag(&mut self) {
        debug_assert!(self.bag.is_empty());
        self.bag.extend(PieceKind::ALL.as_slice());
        self.bag.shuffle(&mut self.rng);
    }

    fn place_cursor(&mut self) {
        let cursor = self
            .cursor
            .take()
            .expect("Called 'place_cursor' without a cursor");
        debug_assert!(
            !self.matrix.is_placeable(&cursor),
            "Tried to place cursor in an invalid location {:?}",
            cursor
        );
        for coord in cursor.cells().unwrap() {
            self.matrix[coord] = true;
        }
    }

    fn move_cursor(&mut self, r#move: Move) -> Result<(), ()> {
        let Some(cursor) = self.cursor.as_mut() else {
            return Ok(());
        };
        let new = cursor.moved_by(r#move.offset());
        if self.matrix.is_placeable(&new) {
            self.cursor = Some(new);
            Ok(())
        } else {
            Err(())
        }
    }
}

struct Matrix([bool; Self::WIDTH * Self::HEIGHT]);

impl Matrix {
    const WIDTH: usize = 10;
    const HEIGHT: usize = 20;
    const SIZE: usize = Self::WIDTH * Self::HEIGHT;

    fn blank() -> Self {
        Self([false; Self::SIZE])
    }

    fn in_matrix(coord: Coord) -> bool {
        Self::valid_coord(coord) && coord.y < Self::HEIGHT
    }

    fn valid_coord(coord: Coord) -> bool {
        coord.x < Self::WIDTH
    }

    fn index(Coord { x, y }: Coord) -> usize {
        y * Self::WIDTH + x
    }

    fn is_placeable(&self, piece: &Piece) -> bool {
        let Some(cells) = piece.cells() else {
            return false;
        };
        cells
            .into_iter()
            .all(|coord| Matrix::in_matrix(coord) && !self[coord])
    }

    fn is_clipping(&self, piece: &Piece) -> bool {
        let Some(cells) = piece.cells() else {
            return false;
        };
        cells
            .into_iter()
            .all(|coord| !Matrix::in_matrix(coord) || !self[coord])
    }
}

impl Index<Coord> for Matrix {
    type Output = bool;
    fn index(&self, coord: Coord) -> &Self::Output {
        assert!(Self::in_matrix(coord));
        &self.0[Self::index(coord)]
    }
}

impl IndexMut<Coord> for Matrix {
    fn index_mut(&mut self, coord: Coord) -> &mut Self::Output {
        assert!(Self::in_matrix(coord));
        &mut self.0[Self::index(coord)]
    }
}
