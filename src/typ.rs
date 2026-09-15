pub struct FunInfo(

#[repr(C)]
pub union TypMeta {
    fun: FunInfo,
}

pub struct Typ {
    mty: MTy,
    meta: TypMeta,
}
