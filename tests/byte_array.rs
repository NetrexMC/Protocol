use binary_utils::*;
use mcpe_protocol::interfaces::{ByteArray, VarString};

pub const FOO_TEST_DATA: &[u8] = &[
    // Length (Array Length)
    2,
    // String length
    7,
    // String Data (string)
    70, 111, 111, 32, 111, 110, 101,
    // magic (512)
    0, 0, 2, 0,
    // bar (32)
    32,
    // String length
    12,
    // String Data (hello)
    72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33,
    // Next struct...
    // String length
    7,
    // String Data (string)
    70, 111, 111, 32, 116, 119, 111,
    // magic (1024)
    0, 0, 4, 0,
    // bar (32)
    240,
    // String length
    12,
    // String Data (string)
    72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33
];

#[derive(Debug, Clone, BinaryStream)]
pub struct Foo {
    pub string: VarString,
    pub magic: u32,
    pub bar: u8,
    pub hello: VarString,
}

#[test]
fn write_byte_array() {
    let foos = vec![
        Foo {
            string: VarString("Foo one".to_string()),
            magic: 512,
            bar: 32,
            hello: VarString("Hello World!".to_string()),
        },
        Foo {
            string: VarString("Foo two".to_string()),
            magic: 1024,
            bar: 240,
            hello: VarString("Hello World!".to_string()),
        },
    ];

    let data = ByteArray::<Foo>(foos);
    assert_eq!(&data.fparse()[..], FOO_TEST_DATA);
}

#[test]
fn read_byte_array() {
    let data = ByteArray::<Foo>::fcompose(FOO_TEST_DATA, &mut 0);
    let foos = vec![
        Foo {
            string: VarString("Foo one".to_string()),
            magic: 512,
            bar: 32,
            hello: VarString("Hello World!".to_string()),
        },
        Foo {
            string: VarString("Foo two".to_string()),
            magic: 1024,
            bar: 240,
            hello: VarString("Hello World!".to_string()),
        },
    ];

    // we need to do our own test case for this.
    for x in 0..foos.len() {
        let actual_foo = foos[x].clone();
        let test_foo = data.0[x].clone();
        // test each prop of foo
        assert_eq!(actual_foo.string.0, test_foo.string.0);
        assert_eq!(actual_foo.magic, test_foo.magic);
        assert_eq!(actual_foo.bar, test_foo.bar);
        assert_eq!(actual_foo.hello.0, test_foo.hello.0);
    }
}
