#![allow(non_upper_case_globals)] //perhaps this three lines isn't necessary
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!("bindings.rs"));


fn main() {
    unsafe{

        let mut instance1 = std::mem::MaybeUninit::<root::ZFCounter>::uninit();  
        root::ZFCounter_ZFCounter(instance1.as_mut_ptr());
        instance1.assume_init_mut().increment(); 
        instance1.assume_init_mut().increment();

        let mut instance2 = std::mem::MaybeUninit::<root::ZFCounter>::uninit();  
        root::ZFCounter_ZFCounter(instance2.as_mut_ptr());
        instance2.assume_init_mut().increment(); 

    }
}
