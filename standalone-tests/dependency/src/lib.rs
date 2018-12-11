#[macro_use]
extern crate stdweb;

pub fn call() {
    submodule::call();
}

mod submodule {
    pub(crate) fn call() {
        js!( @(no_return)
            Module.test_value = 123;
        );
    }
}
