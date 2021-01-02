use crate::BaseT;

extern "C" fn fn_caller_0<F, Ret>(ptr: *mut BaseT) -> Ret
where
    F: FnMut() -> Ret,
{
    unsafe {
        let f = &mut *(ptr as *mut F);
        f()
    }
}

extern "C" fn fn_caller_1<F, Ret, P1>(ptr: *mut BaseT, p1: P1) -> Ret
where
    F: FnMut(P1) -> Ret,
{
    unsafe {
        let f = &mut *(ptr as *mut F);
        f(p1)
    }
}

extern "C" fn fn_caller_2<F, Ret, P1, P2>(ptr: *mut BaseT, p1: P1, p2: P2) -> Ret
where
    F: FnMut(P1, P2) -> Ret,
{
    unsafe {
        let f = &mut *(ptr as *mut F);
        f(p1, p2)
    }
}

extern "C" fn fn_caller_3<F, Ret, P1, P2, P3>(ptr: *mut BaseT, p1: P1, p2: P2, p3: P3) -> Ret
where
    F: FnMut(P1, P2, P3) -> Ret,
{
    unsafe {
        let f = &mut *(ptr as *mut F);
        f(p1, p2, p3)
    }
}

#[allow(dead_code)]
extern "C" fn fn_caller_4<F, Ret, P1, P2, P3, P4>(
    ptr: *mut BaseT,
    p1: P1,
    p2: P2,
    p3: P3,
    p4: P4,
) -> Ret
where
    F: FnMut(P1, P2, P3, P4) -> Ret,
{
    unsafe {
        let f = &mut *(ptr as *mut F);
        f(p1, p2, p3, p4)
    }
}

extern "C" fn fn_caller_5<F, Ret, P1, P2, P3, P4, P5>(
    ptr: *mut BaseT,
    p1: P1,
    p2: P2,
    p3: P3,
    p4: P4,
    p5: P5,
) -> Ret
where
    F: FnMut(P1, P2, P3, P4, P5) -> Ret,
{
    unsafe {
        let f = &mut *(ptr as *mut F);
        f(p1, p2, p3, p4, p5)
    }
}

extern "C" fn fn_caller_6<F, Ret, P1, P2, P3, P4, P5, P6>(
    ptr: *mut BaseT,
    p1: P1,
    p2: P2,
    p3: P3,
    p4: P4,
    p5: P5,
    p6: P6,
) -> Ret
where
    F: FnMut(P1, P2, P3, P4, P5, P6) -> Ret,
{
    unsafe {
        let f = &mut *(ptr as *mut F);
        f(p1, p2, p3, p4, p5, p6)
    }
}

pub fn wrap_closure_0<F, Ret>(_closure: &F) -> extern "C" fn(ptr: *mut BaseT) -> Ret
where
    F: FnMut() -> Ret,
{
    fn_caller_0::<F, Ret>
}

pub fn wrap_closure_1<F, Ret, P1>(_closure: &F) -> extern "C" fn(ptr: *mut BaseT, p1: P1) -> Ret
where
    F: FnMut(P1) -> Ret,
{
    fn_caller_1::<F, Ret, P1>
}

pub fn wrap_closure_2<F, Ret, P1, P2>(
    _closure: &F,
) -> extern "C" fn(ptr: *mut BaseT, p1: P1, p2: P2) -> Ret
where
    F: FnMut(P1, P2) -> Ret,
{
    fn_caller_2::<F, Ret, P1, P2>
}

pub fn wrap_closure_3<F, Ret, P1, P2, P3>(
    _closure: &F,
) -> extern "C" fn(ptr: *mut BaseT, p1: P1, p2: P2, p3: P3) -> Ret
where
    F: FnMut(P1, P2, P3) -> Ret,
{
    fn_caller_3::<F, Ret, P1, P2, P3>
}

#[allow(dead_code)]
pub fn wrap_closure_4<F, Ret, P1, P2, P3, P4>(
    _closure: &F,
) -> extern "C" fn(ptr: *mut BaseT, p1: P1, p2: P2, p3: P3, P4) -> Ret
where
    F: FnMut(P1, P2, P3, P4) -> Ret,
{
    fn_caller_4::<F, Ret, P1, P2, P3, P4>
}

pub fn wrap_closure_5<F, Ret, P1, P2, P3, P4, P5>(
    _closure: &F,
) -> extern "C" fn(ptr: *mut BaseT, p1: P1, p2: P2, p3: P3, P4, P5) -> Ret
where
    F: FnMut(P1, P2, P3, P4, P5) -> Ret,
{
    fn_caller_5::<F, Ret, P1, P2, P3, P4, P5>
}

pub fn wrap_closure_6<F, Ret, P1, P2, P3, P4, P5, P6>(
    _closure: &F,
) -> extern "C" fn(ptr: *mut BaseT, p1: P1, p2: P2, p3: P3, P4, P5, P6) -> Ret
where
    F: FnMut(P1, P2, P3, P4, P5, P6) -> Ret,
{
    fn_caller_6::<F, Ret, P1, P2, P3, P4, P5, P6>
}
