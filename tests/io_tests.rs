extern crate blockbuffers;
extern crate flatbuffers;

pub mod common;

use common::example_generated::example::{get_root_as_example, Example};
use flatbuffers::FlatBufferBuilder;

#[test]
fn io_happy_pass() {
    let (buf, loc) = {
        let mut builder = FlatBufferBuilder::new_with_capacity(1024);
        let ex = Example::create(&mut builder, &Default::default());
        builder.finish(ex, None);
        builder.collapse()
    };

    let ex = get_root_as_example(&buf[loc..]);
    assert_eq!(0, ex.version());
}
