// Copyright 2022 Bill Fraser (@wfraser) <wfraser@codewise.org>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use clap::Parser;

#[derive(Parser, Debug, PartialEq)]
struct Main {
    #[clap(short)]
    s: String,

    #[clap(long)]
    some_string: String,

    #[clap(long)]
    same_name: String,

    #[clap(flatten)]
    foo_args: Foo,

    #[clap(flatten)]
    bar_args: Bar,
}

#[derive(Parser, Debug, PartialEq)]
#[clap(prefix = "foo", next_help_heading = "Foo options")]
struct Foo {
    #[clap(long)]
    some_param: String,

    #[clap(long)]
    same_name: String, // without prefix, would conflict with the one in Main
}

#[derive(Parser, Debug, PartialEq)]
#[clap(prefix = "bar", rename_all = "pascal", next_help_heading = "Bar options")]
struct Bar {
    #[clap(long)]
    another_param: String,
}

#[test]
fn test_all() {
    let expected = Main {
        s: "s-value".to_string(),
        some_string: "some-string-value".to_string(),
        same_name: "same-name-value".to_string(),
        foo_args: Foo {
            some_param: "foo-some-param-value".to_string(),
            same_name: "foo-same-name-value".to_string(),
        },
        bar_args: Bar {
            another_param: "bar-another-param-value".to_string(),
        }
    };

    let result = Main::try_parse_from(&[
        "test",
        "-s", "s-value",
        "--some-string", "some-string-value",
        "--same-name", "same-name-value",
        "--foo.some-param", "foo-some-param-value",
        "--foo.same-name=foo-same-name-value",
        "--bar.AnotherParam", "bar-another-param-value",
    ]).unwrap();
    assert_eq!(result, expected);
}
