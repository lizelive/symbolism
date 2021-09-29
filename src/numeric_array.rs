extern crate nalgebra as na;
use serde::{Deserialize, Serialize};

use na::DMatrix;

//use nalgebra_sparse::{coo::CooMatrix, csr::CsrMatrix, csc::CscMatrix};

use crate::number::{MachineFloat, MachineInt, Number};

use std::{collections::HashMap};

type IndexType = usize;

type IndexVec = Vec<IndexType>;


#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub enum ArrayIndex {
    U0,
    U1 { index: IndexType },
    U2 { row: IndexType, col: IndexType },
    Uv { indices: IndexVec },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum NaMatrix {
    MachineFloat(DMatrix<MachineFloat>),
    MachineInt(DMatrix<MachineInt>),
}

impl NaMatrix {
    fn shape(&self) -> ArrayIndex {
        match self {
            NaMatrix::MachineFloat(data) => ArrayIndex::U2 {
                row: data.nrows(),
                col: data.ncols(),
            },
            NaMatrix::MachineInt(data) => ArrayIndex::U2 {
                row: data.nrows(),
                col: data.ncols(),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum NumericArray {
    NaMatrix(NaMatrix),
    NumberVecArray {
        size: ArrayIndex,
        data: Vec<Number>,
    },
    // MachineIntVecArray {
    //     size: ArrayIndex,
    //     data: Vec<MachineInt>,
    // },
    // MachineFloatVecArray {
    //     size: ArrayIndex,
    //     data: Vec<MachineFloat>,
    // },
    NumberHashMap {
        size: ArrayIndex,
        default: Number,
        data: HashMap<ArrayIndex, Number>,
    },
    Constant {
        size: ArrayIndex,
        value: Number,
    },
    //MachineFloatCooMatrix{size: ArrayIndex, data: CooMatrix<MachineFloat>}
    //MachineFloatCsrMatrix{size: ArrayIndex, data: CsrMatrix<MachineFloat>}
    //MachineFloatCscMatrix{size: ArrayIndex, data: CscMatrix<MachineFloat>}
}

impl NumericArray {
    pub fn get(&self, index: ArrayIndex) -> Number {
        match (self, index) {
            (NumericArray::NaMatrix(matrix), ArrayIndex::U2 { row, col }) => match matrix {
                NaMatrix::MachineFloat(data) => data[(row, col)].into(),
                NaMatrix::MachineInt(data) => data[(row, col)].into(),
            },
            (NumericArray::NumberVecArray { size, data }, index) => {
                let index = size.linear_index(&index).expect("invalid index");
                data[index]
            }
            (
                NumericArray::NumberHashMap {
                    size: _,
                    default,
                    data,
                },
                index,
            ) => *data.get(&index).unwrap_or(default),
            (NumericArray::Constant { size: _, value }, _index) => *value,
            _ => panic!("invalid index / backing paring"),
        }
    }

    pub fn set(&mut self, index: ArrayIndex, value: Number) {
        match (self, index) {
            (NumericArray::NaMatrix(matrix), ArrayIndex::U2 { row, col }) => match matrix {
                NaMatrix::MachineFloat(data) => data[(row, col)] = value.into(),
                NaMatrix::MachineInt(data) => data[(row, col)] = value.into(),
            },
            (NumericArray::NumberVecArray { size, data }, index) => {
                let index = size.linear_index(&index).expect("invalid index");
                data[index] = value
            }
            (
                NumericArray::NumberHashMap {
                    size: _,
                    default: _,
                    data,
                },
                index,
            ) => {
                data.insert(index, value);
            }
            (NumericArray::Constant { size: _, value: _ }, _index) => panic!("cant set const array"),
            _ => panic!("invalid index / backing paring"),
        }
    }

    pub fn shape(&self) -> ArrayIndex {
        match self {
            NumericArray::NumberVecArray { size, data: _ } => size.clone(),
            NumericArray::Constant { size, value: _ } => size.clone(),
            NumericArray::NumberHashMap {
                size,
                default: _,
                data: _,
            } => size.clone(),
            NumericArray::NaMatrix(matrix) => matrix.shape(),
        }
    }
}

// fn nest_list<TState, TInput>(
//     f: &dyn Fn(&TState, &TInput) -> TState,
//     start: TState,
//     list: &Vec<TInput>,
// ) -> Vec<TState> {
//     let mut out = Vec::new();
//     let mut x = start;
//     for y in list {
//         out.push(x);
//         x = f(&x, y)
//     }
//     return out;
// }

impl From<ArrayIndex> for IndexVec {
    fn from(val: ArrayIndex) -> Self {
        match val {
            ArrayIndex::U1 { index } => vec![index],
            ArrayIndex::U2 { row, col } => vec![row, col],
            ArrayIndex::Uv { indices } => indices,
            ArrayIndex::U0 => vec![],
        }
    }
}

impl From<IndexVec> for ArrayIndex {
    fn from(val: IndexVec) -> Self {
        match val.len() {
            0 => ArrayIndex::U0,
            1 => ArrayIndex::U1 { index: val[0] },
            2 => ArrayIndex::U2 {
                row: val[0],
                col: val[1],
            },
            _ => ArrayIndex::Uv { indices: val },
            //_ => panic!("invalid index length"),
        }
    }
}

impl ArrayIndex {
    pub fn len(self: &ArrayIndex) -> IndexType {
        match self {
            ArrayIndex::U0 => 0,
            ArrayIndex::U1 { index: _ } => 1,
            ArrayIndex::U2 { row: _, col: _ } => 2,
            ArrayIndex::Uv { indices } => indices.len(),
        }
    }
    // fn normalized(self: &ArrayIndex) -> ArrayIndex{
    //     if let ArrayIndex::Uv { indices } = self{
    //         if self.len() < 3{
    //             return indices.into();
    //         }
    //     }
    //     return self.clone()
    // }

    pub fn linear_index(self: &ArrayIndex, index: &ArrayIndex) -> Option<IndexType> {
        match (self, index) {
            (ArrayIndex::U0, ArrayIndex::U0) => Some(0),
            (ArrayIndex::U1 { index: _size }, ArrayIndex::U1 { index }) => Some(*index),
            (
                ArrayIndex::U2 {
                    row: nrow,
                    col: _ncol,
                },
                ArrayIndex::U2 { row, col },
            ) => Some(row + nrow * col),
            _ => None,
        }
        // let size: IndexVec = self.into();
        // let index: IndexVec = index.into();
        // let mut multiplyer: IndexType = 1;
        // let mut acc: IndexType = 0;
        // for i in 0..size.len() {
        //     let msize = size[i];
        //     let mindex = index[i];
        //     acc += multiplyer * (mindex - 1);
        //     multiplyer *= msize;
        // }
        // return acc
    }
    pub fn contains(self: &ArrayIndex, index: &ArrayIndex) -> bool {
        match (self, index) {
            (ArrayIndex::U0, ArrayIndex::U0) => true,
            (ArrayIndex::U1 { index: size }, ArrayIndex::U1 { index }) => {
                1 <= *index && *index <= *size
            }
            (
                ArrayIndex::U2 {
                    row: nrow,
                    col: ncol,
                },
                ArrayIndex::U2 { row, col },
            ) => *row >= 1 && *col >= 1 && *row <= *nrow && *col <= *ncol,
            _ => false,
        }

        // match shape {
        //     ArrayIndex::U1 { index: length } => {
        //         if let ArrayIndex::U1 { index } = index {
        //             return index <= length;
        //         }
        //         return false;
        //     }
        //     ArrayIndex::U2 { row: nrow, col: ncol } => match index {
        //         ArrayIndex::U2 { row, col } => row <=nrow && col <= ncol,
        //         _ => false
        //     },
        //     ArrayIndex::U0 => return true,
        // }
        // let size: IndexVec = shape.into();
        // let index: IndexVec = index.into();
        // let mut oob = false;
        // for i in 0..size.len() {
        //     let msize = size[i];
        //     let mindex = index[i];
        //     oob |= mindex <= 0 || mindex > msize;
        // }
        // return !oob;
    }
}

