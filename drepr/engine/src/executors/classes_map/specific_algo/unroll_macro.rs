macro_rules! unroll_enumerate {
  ($arrays:expr, 0, $macro_func:ident $(, $arg:tt $($o:literal)?)*) => {
  };
  ($arrays:expr, 1, $macro_func:ident $(, $arg:tt $($o:literal)?)*) => {
    $macro_func!(0, $arrays[0] $(, $arg $($o)?)*);
  };
  ($arrays:expr, 2, $macro_func:ident $(, $arg:tt $($o:literal)?)*) => {
    $macro_func!(0, $arrays[0] $(, $arg $($o)?)*);
    $macro_func!(1, $arrays[1] $(, $arg $($o)?)*);
  };
  ($arrays:expr, 3, $macro_func:ident $(, $arg:tt $($o:literal)?)*) => {
    $macro_func!(0, $arrays[0] $(, $arg $($o)?)*);
    $macro_func!(1, $arrays[1] $(, $arg $($o)?)*);
    $macro_func!(2, $arrays[2] $(, $arg $($o)?)*);
  };
  ($arrays:expr, 4, $macro_func:ident $(, $arg:tt $($o:literal)?)*) => {
    $macro_func!(0, $arrays[0] $(, $arg $($o)?)*);
    $macro_func!(1, $arrays[1] $(, $arg $($o)?)*);
    $macro_func!(2, $arrays[2] $(, $arg $($o)?)*);
    $macro_func!(3, $arrays[3] $(, $arg $($o)?)*);
  };
  ($arrays:expr, 5, $macro_func:ident $(, $arg:tt $($o:literal)?)*) => {
    $macro_func!(0, $arrays[0] $(, $arg $($o)?)*);
    $macro_func!(1, $arrays[1] $(, $arg $($o)?)*);
    $macro_func!(2, $arrays[2] $(, $arg $($o)?)*);
    $macro_func!(3, $arrays[3] $(, $arg $($o)?)*);
    $macro_func!(4, $arrays[4] $(, $arg $($o)?)*);
  };
  ($arrays:expr, 6, $macro_func:ident $(, $arg:tt $($o:literal)?)*) => {
    $macro_func!(0, $arrays[0] $(, $arg $($o)?)*);
    $macro_func!(1, $arrays[1] $(, $arg $($o)?)*);
    $macro_func!(2, $arrays[2] $(, $arg $($o)?)*);
    $macro_func!(3, $arrays[3] $(, $arg $($o)?)*);
    $macro_func!(4, $arrays[4] $(, $arg $($o)?)*);
    $macro_func!(5, $arrays[5] $(, $arg $($o)?)*);
  };
  ($arrays:expr, 7, $macro_func:ident $(, $arg:tt $($o:literal)?)*) => {
    $macro_func!(0, $arrays[0] $(, $arg $($o)?)*);
    $macro_func!(1, $arrays[1] $(, $arg $($o)?)*);
    $macro_func!(2, $arrays[2] $(, $arg $($o)?)*);
    $macro_func!(3, $arrays[3] $(, $arg $($o)?)*);
    $macro_func!(4, $arrays[4] $(, $arg $($o)?)*);
    $macro_func!(5, $arrays[5] $(, $arg $($o)?)*);
    $macro_func!(6, $arrays[6] $(, $arg $($o)?)*);
  };
  ($arrays:expr, 8, $macro_func:ident $(, $arg:tt $($o:literal)?)*) => {
    $macro_func!(0, $arrays[0] $(, $arg $($o)?)*);
    $macro_func!(1, $arrays[1] $(, $arg $($o)?)*);
    $macro_func!(2, $arrays[2] $(, $arg $($o)?)*);
    $macro_func!(3, $arrays[3] $(, $arg $($o)?)*);
    $macro_func!(4, $arrays[4] $(, $arg $($o)?)*);
    $macro_func!(5, $arrays[5] $(, $arg $($o)?)*);
    $macro_func!(6, $arrays[6] $(, $arg $($o)?)*);
    $macro_func!(7, $arrays[7] $(, $arg $($o)?)*);
  };
  ($arrays:expr, 9, $macro_func:ident $(, $arg:tt $($o:literal)?)*) => {
    $macro_func!(0, $arrays[0] $(, $arg $($o)?)*);
    $macro_func!(1, $arrays[1] $(, $arg $($o)?)*);
    $macro_func!(2, $arrays[2] $(, $arg $($o)?)*);
    $macro_func!(3, $arrays[3] $(, $arg $($o)?)*);
    $macro_func!(4, $arrays[4] $(, $arg $($o)?)*);
    $macro_func!(5, $arrays[5] $(, $arg $($o)?)*);
    $macro_func!(6, $arrays[6] $(, $arg $($o)?)*);
    $macro_func!(7, $arrays[7] $(, $arg $($o)?)*);
    $macro_func!(8, $arrays[8] $(, $arg $($o)?)*);
  };
  ($arrays:expr, 10, $macro_func:ident $(, $arg:tt $($o:literal)?)*) => {
    $macro_func!(0, $arrays[0] $(, $arg $($o)?)*);
    $macro_func!(1, $arrays[1] $(, $arg $($o)?)*);
    $macro_func!(2, $arrays[2] $(, $arg $($o)?)*);
    $macro_func!(3, $arrays[3] $(, $arg $($o)?)*);
    $macro_func!(4, $arrays[4] $(, $arg $($o)?)*);
    $macro_func!(5, $arrays[5] $(, $arg $($o)?)*);
    $macro_func!(6, $arrays[6] $(, $arg $($o)?)*);
    $macro_func!(7, $arrays[7] $(, $arg $($o)?)*);
    $macro_func!(8, $arrays[8] $(, $arg $($o)?)*);
    $macro_func!(9, $arrays[9] $(, $arg $($o)?)*);
  };
  ($arrays:expr, 11, $macro_func:ident $(, $arg:tt $($o:literal)?)*) => {
    $macro_func!(0, $arrays[0] $(, $arg $($o)?)*);
    $macro_func!(1, $arrays[1] $(, $arg $($o)?)*);
    $macro_func!(2, $arrays[2] $(, $arg $($o)?)*);
    $macro_func!(3, $arrays[3] $(, $arg $($o)?)*);
    $macro_func!(4, $arrays[4] $(, $arg $($o)?)*);
    $macro_func!(5, $arrays[5] $(, $arg $($o)?)*);
    $macro_func!(6, $arrays[6] $(, $arg $($o)?)*);
    $macro_func!(7, $arrays[7] $(, $arg $($o)?)*);
    $macro_func!(8, $arrays[8] $(, $arg $($o)?)*);
    $macro_func!(9, $arrays[9] $(, $arg $($o)?)*);
    $macro_func!(10, $arrays[10] $(, $arg $($o)?)*);
    $macro_func!(11, $arrays[11] $(, $arg $($o)?)*);
  };
  ($arrays:expr, 12, $macro_func:ident $(, $arg:tt $($o:literal)?)*) => {
    $macro_func!(0, $arrays[0] $(, $arg $($o)?)*);
    $macro_func!(1, $arrays[1] $(, $arg $($o)?)*);
    $macro_func!(2, $arrays[2] $(, $arg $($o)?)*);
    $macro_func!(3, $arrays[3] $(, $arg $($o)?)*);
    $macro_func!(4, $arrays[4] $(, $arg $($o)?)*);
    $macro_func!(5, $arrays[5] $(, $arg $($o)?)*);
    $macro_func!(6, $arrays[6] $(, $arg $($o)?)*);
    $macro_func!(7, $arrays[7] $(, $arg $($o)?)*);
    $macro_func!(8, $arrays[8] $(, $arg $($o)?)*);
    $macro_func!(9, $arrays[9] $(, $arg $($o)?)*);
    $macro_func!(10, $arrays[10] $(, $arg $($o)?)*);
    $macro_func!(11, $arrays[11] $(, $arg $($o)?)*);
  };
  ($arrays:expr, 13, $macro_func:ident $(, $arg:tt $($o:literal)?)*) => {
    $macro_func!(0, $arrays[0] $(, $arg $($o)?)*);
    $macro_func!(1, $arrays[1] $(, $arg $($o)?)*);
    $macro_func!(2, $arrays[2] $(, $arg $($o)?)*);
    $macro_func!(3, $arrays[3] $(, $arg $($o)?)*);
    $macro_func!(4, $arrays[4] $(, $arg $($o)?)*);
    $macro_func!(5, $arrays[5] $(, $arg $($o)?)*);
    $macro_func!(6, $arrays[6] $(, $arg $($o)?)*);
    $macro_func!(7, $arrays[7] $(, $arg $($o)?)*);
    $macro_func!(8, $arrays[8] $(, $arg $($o)?)*);
    $macro_func!(9, $arrays[9] $(, $arg $($o)?)*);
    $macro_func!(10, $arrays[10] $(, $arg $($o)?)*);
    $macro_func!(11, $arrays[11] $(, $arg $($o)?)*);
    $macro_func!(12, $arrays[12] $(, $arg $($o)?)*);
  };
  ($arrays:expr, 14, $macro_func:ident $(, $arg:tt $($o:literal)?)*) => {
    $macro_func!(0, $arrays[0] $(, $arg $($o)?)*);
    $macro_func!(1, $arrays[1] $(, $arg $($o)?)*);
    $macro_func!(2, $arrays[2] $(, $arg $($o)?)*);
    $macro_func!(3, $arrays[3] $(, $arg $($o)?)*);
    $macro_func!(4, $arrays[4] $(, $arg $($o)?)*);
    $macro_func!(5, $arrays[5] $(, $arg $($o)?)*);
    $macro_func!(6, $arrays[6] $(, $arg $($o)?)*);
    $macro_func!(7, $arrays[7] $(, $arg $($o)?)*);
    $macro_func!(8, $arrays[8] $(, $arg $($o)?)*);
    $macro_func!(9, $arrays[9] $(, $arg $($o)?)*);
    $macro_func!(10, $arrays[10] $(, $arg $($o)?)*);
    $macro_func!(11, $arrays[11] $(, $arg $($o)?)*);
    $macro_func!(12, $arrays[12] $(, $arg $($o)?)*);
    $macro_func!(13, $arrays[13] $(, $arg $($o)?)*);
  };
  ($arrays:expr, 15, $macro_func:ident $(, $arg:tt $($o:literal)?)*) => {
    $macro_func!(0, $arrays[0] $(, $arg $($o)?)*);
    $macro_func!(1, $arrays[1] $(, $arg $($o)?)*);
    $macro_func!(2, $arrays[2] $(, $arg $($o)?)*);
    $macro_func!(3, $arrays[3] $(, $arg $($o)?)*);
    $macro_func!(4, $arrays[4] $(, $arg $($o)?)*);
    $macro_func!(5, $arrays[5] $(, $arg $($o)?)*);
    $macro_func!(6, $arrays[6] $(, $arg $($o)?)*);
    $macro_func!(7, $arrays[7] $(, $arg $($o)?)*);
    $macro_func!(8, $arrays[8] $(, $arg $($o)?)*);
    $macro_func!(9, $arrays[9] $(, $arg $($o)?)*);
    $macro_func!(10, $arrays[10] $(, $arg $($o)?)*);
    $macro_func!(11, $arrays[11] $(, $arg $($o)?)*);
    $macro_func!(12, $arrays[12] $(, $arg $($o)?)*);
    $macro_func!(13, $arrays[13] $(, $arg $($o)?)*);
    $macro_func!(14, $arrays[14] $(, $arg $($o)?)*);
  };
  ($arrays:expr, 16, $macro_func:ident $(, $arg:tt $($o:literal)?)*) => {
    $macro_func!(0, $arrays[0] $(, $arg $($o)?)*);
    $macro_func!(1, $arrays[1] $(, $arg $($o)?)*);
    $macro_func!(2, $arrays[2] $(, $arg $($o)?)*);
    $macro_func!(3, $arrays[3] $(, $arg $($o)?)*);
    $macro_func!(4, $arrays[4] $(, $arg $($o)?)*);
    $macro_func!(5, $arrays[5] $(, $arg $($o)?)*);
    $macro_func!(6, $arrays[6] $(, $arg $($o)?)*);
    $macro_func!(7, $arrays[7] $(, $arg $($o)?)*);
    $macro_func!(8, $arrays[8] $(, $arg $($o)?)*);
    $macro_func!(9, $arrays[9] $(, $arg $($o)?)*);
    $macro_func!(10, $arrays[10] $(, $arg $($o)?)*);
    $macro_func!(11, $arrays[11] $(, $arg $($o)?)*);
    $macro_func!(12, $arrays[12] $(, $arg $($o)?)*);
    $macro_func!(13, $arrays[13] $(, $arg $($o)?)*);
    $macro_func!(14, $arrays[14] $(, $arg $($o)?)*);
    $macro_func!(15, $arrays[15] $(, $arg $($o)?)*);
  };
  ($arrays:expr, 17, $macro_func:ident $(, $arg:tt $($o:literal)?)*) => {
    $macro_func!(0, $arrays[0] $(, $arg $($o)?)*);
    $macro_func!(1, $arrays[1] $(, $arg $($o)?)*);
    $macro_func!(2, $arrays[2] $(, $arg $($o)?)*);
    $macro_func!(3, $arrays[3] $(, $arg $($o)?)*);
    $macro_func!(4, $arrays[4] $(, $arg $($o)?)*);
    $macro_func!(5, $arrays[5] $(, $arg $($o)?)*);
    $macro_func!(6, $arrays[6] $(, $arg $($o)?)*);
    $macro_func!(7, $arrays[7] $(, $arg $($o)?)*);
    $macro_func!(8, $arrays[8] $(, $arg $($o)?)*);
    $macro_func!(9, $arrays[9] $(, $arg $($o)?)*);
    $macro_func!(10, $arrays[10] $(, $arg $($o)?)*);
    $macro_func!(11, $arrays[11] $(, $arg $($o)?)*);
    $macro_func!(12, $arrays[12] $(, $arg $($o)?)*);
    $macro_func!(13, $arrays[13] $(, $arg $($o)?)*);
    $macro_func!(14, $arrays[14] $(, $arg $($o)?)*);
    $macro_func!(15, $arrays[15] $(, $arg $($o)?)*);
    $macro_func!(16, $arrays[16] $(, $arg $($o)?)*);
  };
  ($arrays:expr, 18, $macro_func:ident $(, $arg:tt $($o:literal)?)*) => {
    $macro_func!(0, $arrays[0] $(, $arg $($o)?)*);
    $macro_func!(1, $arrays[1] $(, $arg $($o)?)*);
    $macro_func!(2, $arrays[2] $(, $arg $($o)?)*);
    $macro_func!(3, $arrays[3] $(, $arg $($o)?)*);
    $macro_func!(4, $arrays[4] $(, $arg $($o)?)*);
    $macro_func!(5, $arrays[5] $(, $arg $($o)?)*);
    $macro_func!(6, $arrays[6] $(, $arg $($o)?)*);
    $macro_func!(7, $arrays[7] $(, $arg $($o)?)*);
    $macro_func!(8, $arrays[8] $(, $arg $($o)?)*);
    $macro_func!(9, $arrays[9] $(, $arg $($o)?)*);
    $macro_func!(10, $arrays[10] $(, $arg $($o)?)*);
    $macro_func!(11, $arrays[11] $(, $arg $($o)?)*);
    $macro_func!(12, $arrays[12] $(, $arg $($o)?)*);
    $macro_func!(13, $arrays[13] $(, $arg $($o)?)*);
    $macro_func!(14, $arrays[14] $(, $arg $($o)?)*);
    $macro_func!(15, $arrays[15] $(, $arg $($o)?)*);
    $macro_func!(16, $arrays[16] $(, $arg $($o)?)*);
    $macro_func!(17, $arrays[17] $(, $arg $($o)?)*);
  };
  ($arrays:expr, 19, $macro_func:ident $(, $arg:tt $($o:literal)?)*) => {
    $macro_func!(0, $arrays[0] $(, $arg $($o)?)*);
    $macro_func!(1, $arrays[1] $(, $arg $($o)?)*);
    $macro_func!(2, $arrays[2] $(, $arg $($o)?)*);
    $macro_func!(3, $arrays[3] $(, $arg $($o)?)*);
    $macro_func!(4, $arrays[4] $(, $arg $($o)?)*);
    $macro_func!(5, $arrays[5] $(, $arg $($o)?)*);
    $macro_func!(6, $arrays[6] $(, $arg $($o)?)*);
    $macro_func!(7, $arrays[7] $(, $arg $($o)?)*);
    $macro_func!(8, $arrays[8] $(, $arg $($o)?)*);
    $macro_func!(9, $arrays[9] $(, $arg $($o)?)*);
    $macro_func!(10, $arrays[10] $(, $arg $($o)?)*);
    $macro_func!(11, $arrays[11] $(, $arg $($o)?)*);
    $macro_func!(12, $arrays[12] $(, $arg $($o)?)*);
    $macro_func!(13, $arrays[13] $(, $arg $($o)?)*);
    $macro_func!(14, $arrays[14] $(, $arg $($o)?)*);
    $macro_func!(15, $arrays[15] $(, $arg $($o)?)*);
    $macro_func!(16, $arrays[16] $(, $arg $($o)?)*);
    $macro_func!(17, $arrays[17] $(, $arg $($o)?)*);
    $macro_func!(18, $arrays[18] $(, $arg $($o)?)*);
  };
  ($arrays:expr, 20, $macro_func:ident $(, $arg:tt $($o:literal)?)*) => {
    $macro_func!(0, $arrays[0] $(, $arg $($o)?)*);
    $macro_func!(1, $arrays[1] $(, $arg $($o)?)*);
    $macro_func!(2, $arrays[2] $(, $arg $($o)?)*);
    $macro_func!(3, $arrays[3] $(, $arg $($o)?)*);
    $macro_func!(4, $arrays[4] $(, $arg $($o)?)*);
    $macro_func!(5, $arrays[5] $(, $arg $($o)?)*);
    $macro_func!(6, $arrays[6] $(, $arg $($o)?)*);
    $macro_func!(7, $arrays[7] $(, $arg $($o)?)*);
    $macro_func!(8, $arrays[8] $(, $arg $($o)?)*);
    $macro_func!(9, $arrays[9] $(, $arg $($o)?)*);
    $macro_func!(10, $arrays[10] $(, $arg $($o)?)*);
    $macro_func!(11, $arrays[11] $(, $arg $($o)?)*);
    $macro_func!(12, $arrays[12] $(, $arg $($o)?)*);
    $macro_func!(13, $arrays[13] $(, $arg $($o)?)*);
    $macro_func!(14, $arrays[14] $(, $arg $($o)?)*);
    $macro_func!(15, $arrays[15] $(, $arg $($o)?)*);
    $macro_func!(16, $arrays[16] $(, $arg $($o)?)*);
    $macro_func!(17, $arrays[17] $(, $arg $($o)?)*);
    $macro_func!(18, $arrays[18] $(, $arg $($o)?)*);
    $macro_func!(19, $arrays[19] $(, $arg $($o)?)*);
  };
  ($arrays:expr, 30, $macro_func:ident $(, $arg:tt $($o:literal)?)*) => {
    $macro_func!(0, $arrays[0] $(, $arg $($o)?)*);
    $macro_func!(1, $arrays[1] $(, $arg $($o)?)*);
    $macro_func!(2, $arrays[2] $(, $arg $($o)?)*);
    $macro_func!(3, $arrays[3] $(, $arg $($o)?)*);
    $macro_func!(4, $arrays[4] $(, $arg $($o)?)*);
    $macro_func!(5, $arrays[5] $(, $arg $($o)?)*);
    $macro_func!(6, $arrays[6] $(, $arg $($o)?)*);
    $macro_func!(7, $arrays[7] $(, $arg $($o)?)*);
    $macro_func!(8, $arrays[8] $(, $arg $($o)?)*);
    $macro_func!(9, $arrays[9] $(, $arg $($o)?)*);
    $macro_func!(10, $arrays[10] $(, $arg $($o)?)*);
    $macro_func!(11, $arrays[11] $(, $arg $($o)?)*);
    $macro_func!(12, $arrays[12] $(, $arg $($o)?)*);
    $macro_func!(13, $arrays[13] $(, $arg $($o)?)*);
    $macro_func!(14, $arrays[14] $(, $arg $($o)?)*);
    $macro_func!(15, $arrays[15] $(, $arg $($o)?)*);
    $macro_func!(16, $arrays[16] $(, $arg $($o)?)*);
    $macro_func!(17, $arrays[17] $(, $arg $($o)?)*);
    $macro_func!(18, $arrays[18] $(, $arg $($o)?)*);
    $macro_func!(19, $arrays[19] $(, $arg $($o)?)*);
    $macro_func!(20, $arrays[20] $(, $arg $($o)?)*);
    $macro_func!(21, $arrays[21] $(, $arg $($o)?)*);
    $macro_func!(22, $arrays[22] $(, $arg $($o)?)*);
    $macro_func!(23, $arrays[23] $(, $arg $($o)?)*);
    $macro_func!(24, $arrays[24] $(, $arg $($o)?)*);
    $macro_func!(25, $arrays[25] $(, $arg $($o)?)*);
    $macro_func!(26, $arrays[26] $(, $arg $($o)?)*);
    $macro_func!(27, $arrays[27] $(, $arg $($o)?)*);
    $macro_func!(28, $arrays[28] $(, $arg $($o)?)*);
    $macro_func!(29, $arrays[29] $(, $arg $($o)?)*);
  };
  ($arrays:expr, unknown, $macro_func:ident $(, $arg:tt $($o:literal)?)*) => {
    for (__index, __value) in $arrays.iter().enumerate() {
      $macro_func!(__index, __value $(, $arg $($o)?)*);
    }
  }
}
