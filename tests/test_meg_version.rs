use support::{project, execs};
use hamcrest::assert_that;
use turbo;

fn setup() {}

test!(simple {
    let p = project("foo");

    assert_that(p.cargo_process("version"),
                execs().with_status(0).with_stdout(&format!("{}\n",
                                                            meg::version())));

    assert_that(p.cargo_process("--version"),
                execs().with_status(0).with_stdout(&format!("{}\n",
                                                            meg::version())));

});
