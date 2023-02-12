pub const MPI_SUCCESS: i32 = 0;
pub const MPI_ERR_OTHER: i32 = 0;

#[allow(non_camel_case_types)]
pub enum MpiComparisonResult {
    MPI_IDENT = 0,
    MPI_CONGRUENT = 1,
    MPI_SIMILAR = 2,
    MPI_UNEQUAL = 3,
}

pub const MPI_COMM_WORLD: i32 = 0;
pub const MPI_COMM_SELF: i32 = 1;
pub const MPI_COMM_NULL: i32 = 2;

pub const MPI_INT8_T: i32 = 0;
pub const MPI_INT16_T: i32 = 1;
pub const MPI_INT32_T: i32 = 2;
pub const MPI_INT: i32 = 3;
pub const MPI_INT64_T: i32 = 4;
pub const MPI_UINT8_T: i32 = 5;
pub const MPI_UINT16_T: i32 = 6;
pub const MPI_UINT32_T: i32 = 7;
pub const MPI_UINT_T: i32 = 8;
pub const MPI_UINT64_T: i32 = 9;
pub const MPI_LONG: i32 = 10;
pub const MPI_LONG_LONG: i32 = 11;
pub const MPI_LONG_LONG_INT: i32 = 12;
pub const MPI_FLOAT: i32 = 13;
pub const MPI_DOUBLE: i32 = 14;
pub const MPI_DOUBLE_INT: i32 = 15;
pub const MPI_CHAR: i32 = 16;
pub const MPI_C_BOOL: i32 = 17;
pub const MPI_BYTE: i32 = 18;
pub const MPI_DATATYPE_NULL: i32 = -1;

pub const MPI_MAX: i32 = 0;
pub const MPI_MIN: i32 = 1;
pub const MPI_SUM: i32 = 2;
pub const MPI_PROD: i32 = 3;
pub const MPI_LAND: i32 = 4;
pub const MPI_LOR: i32 = 5;
pub const MPI_BAND: i32 = 6;
pub const MPI_BOR: i32 = 7;
pub const MPI_MAXLOC: i32 = 8;
pub const MPI_MINLOC: i32 = 9;
pub const MPI_OP_NULL: i32 = -1;

