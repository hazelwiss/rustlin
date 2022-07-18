pub unsafe fn put<V>(adr: usize, val: V) {
    *(adr as *mut V) = val;
}

pub unsafe fn get<V: Copy>(adr: usize) -> V {
    *(adr as *const V)
}

pub unsafe fn get_ref<V>(adr: usize) -> &'static V {
    &*(adr as *const V)
}

pub unsafe fn get_mut<V>(adr: usize) -> &'static mut V {
    &mut *(adr as *mut V)
}
