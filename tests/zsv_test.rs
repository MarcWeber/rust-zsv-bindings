use std::{cell::RefCell, mem::{self, transmute}, rc::Rc, slice};

use libc::{c_void, fread};
use rust_zsv_bindings::{zsv_cell_count, zsv_delete, zsv_finish, zsv_get_cell, zsv_new, zsv_opts, zsv_parse_more, zsv_parser, zsv_status, zsv_status_zsv_status_no_more_input, zsv_status_zsv_status_ok};

struct MyOpts {
  parser: zsv_parser,
  cells: Rc<RefCell<Vec<Vec<Box<[u8]>>>>>,
}

extern "C" fn row_handler (ctx : * mut ::std::os::raw::c_void) {
  let mo: *const MyOpts = ctx as *const MyOpts;
  let my_opts: &MyOpts = unsafe { mem::transmute(mo) };

  let cell_count: usize =  unsafe { zsv_cell_count(my_opts.parser) } ;
  let row: Vec<_> = (1..cell_count).map(|x| {
    let c = unsafe { zsv_get_cell(my_opts.parser, x) };
    let s = unsafe { slice::from_raw_parts(c.str_, c.len) };
    Box::from(s)
  }).collect();
  my_opts.cells.borrow_mut().push(row)
}


pub unsafe extern "C" fn myread(
    mut ptr: *mut libc::c_void,
    mut size: usize,
    mut nmemb: usize,
    mut stream: *mut ::std::os::raw::c_void,

) -> usize {
  let x: Box<[u8]> = Box::from_raw(transmute(stream));
  return 0;
}


#[test]
fn it_can_parse_a_zsv_file() {
  let csv_contents = Box::new( b"NAME,AGE\nBen,20");

  let mut mo = MyOpts {
    parser: unsafe { mem::zeroed() },
    cells: Rc::new(RefCell::new(vec![]))
  };

  let mut zo : zsv_opts = {
    unsafe { mem::zeroed() }
  };

  zo.delimiter = ',' as i8;
  // zo.row_handler = Some( unsafe { mem::transmute::<_, unsafe extern "C" fn(*mut std::ffi::c_void)>(handler2 as *const ()) } );
  zo.row_handler = Some( row_handler );
  zo.ctx = &mut mo as *mut _ as *mut c_void;

  // IF FILE
  // let cstr = CString::new(file_path).unwrap();
  // let mode = CString::new("r").unwrap();
  // let file = unsafe { fopen(cstr.as_ptr(), mode.as_ptr()) };


  zo.stream = unsafe { transmute(Box::into_raw(csv_contents)) };

  zo.read = Some(myread as unsafe extern "C" fn(*mut c_void, usize, usize, *mut c_void) -> usize);

  mo.parser = unsafe { zsv_new(&mut zo) };

  let mut stat: zsv_status;
  loop {
    stat = unsafe { zsv_parse_more(mo.parser) };
    if stat != zsv_status_zsv_status_ok {
      break
    }
  }

  unsafe {
    zsv_finish(mo.parser);
    zsv_delete(mo.parser);
    assert!(stat == zsv_status_zsv_status_no_more_input);
  }

  let x = unsafe { Box::from_raw(zo.stream) };
}
