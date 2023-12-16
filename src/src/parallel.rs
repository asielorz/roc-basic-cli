use core::ffi::c_void;
use rayon::prelude::*;

#[derive(Clone, Copy)]
struct Task {
    task_function: extern "C" fn(*mut c_void),
    params: *mut c_void,
}

unsafe impl Send for Task {}
unsafe impl Sync for Task {}

#[no_mangle]
pub unsafe extern "C" fn roc_parallel_context_create(task_count_hint: usize) -> *mut c_void {
    let vec: Vec<Task> = Vec::with_capacity(task_count_hint);
    std::mem::transmute::<_, *mut c_void>(Box::into_raw(Box::new(vec)))
}

#[no_mangle]
pub unsafe extern "C" fn roc_parallel_context_register_task(
    context: *mut c_void,
    task_function: extern "C" fn(*mut c_void),
    params: *mut c_void,
) {
    let as_vec = std::mem::transmute::<_, *mut Vec<Task>>(context);
    (*as_vec).push(Task {
        task_function,
        params,
    });
}

#[no_mangle]
pub unsafe extern "C" fn roc_parallel_context_run(context: *mut c_void) {
    let as_vec = std::mem::transmute::<_, *mut Vec<Task>>(context);
    (*as_vec).par_iter().for_each(|task| {
        (task.task_function)(task.params);
    });
}

#[no_mangle]
pub unsafe extern "C" fn roc_parallel_context_destroy(context: *mut c_void) {
    let as_vec = std::mem::transmute::<_, *mut Vec<Task>>(context);
    let _ = Box::from_raw(as_vec);
}
