use utils::*;
use dependency;

pub fn run() {
    // See https://github.com/rust-lang/rust/issues/56639 for more details.
    test( "indirectly_call_js_snippet_from_a_submodule_in_another_crate", || {
        dependency::call();
        js! {
            assert.strictEqual( Module.test_value, 123 );
        }
    });
}
