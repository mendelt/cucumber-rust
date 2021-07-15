// Copyright (c) 2018-2021  Brendan Molloy <brendan@bbqsrc.net>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

macro_rules! cprint {
    ($fg:expr, $($arg:tt)*) => {{
        use termcolor::{ColorChoice, ColorSpec, StandardStream, WriteColor};
        use std::io::Write;
        let mut stdout = StandardStream::stdout(ColorChoice::Auto);
        let _x = stdout.set_color(ColorSpec::new().set_fg(Some($fg)));
        let _x = write!(&mut stdout, $($arg)*);
        let _x = stdout.reset();
    }};
    (bold $fg:expr, $($arg:tt)*) => {{
        use termcolor::{ColorChoice, ColorSpec, StandardStream, WriteColor};
        use std::io::Write;
        let mut stdout = StandardStream::stdout(ColorChoice::Auto);
        let _x = stdout.set_color(ColorSpec::new().set_fg(Some($fg)).set_bold(true));
        let _x = write!(&mut stdout, $($arg)*);
        let _x = stdout.reset();
    }};
}

macro_rules! cprintln {
    ($fg:expr, $fmt:expr) => (cprint!($fg, concat!($fmt, "\n")));
    ($fg:expr, $fmt:expr, $($arg:tt)*) => (cprint!($fg, concat!($fmt, "\n"), $($arg)*));
    (bold $fg:expr, $fmt:expr) => (cprint!(bold $fg, concat!($fmt, "\n")));
    (bold $fg:expr, $fmt:expr, $($arg:tt)*) => (cprint!(bold $fg, concat!($fmt, "\n"), $($arg)*));
}

#[macro_export]
macro_rules! t {
    // Async with block and mutable world
    (| mut $world:ident, $step:ident | $($input:tt)*) => {
        |mut $world, $step| {
            use $crate::futures::future::FutureExt as _;
            std::panic::AssertUnwindSafe(async move { $($input)* })
                .catch_unwind()
                .map(|r| r.map_err($crate::TestError::PanicError))
                .boxed_local()
        }
    };
    // Async with block and mutable world with type
    (| mut $world:ident : $worldty:path, $step:ident | $($input:tt)*) => {
        |mut $world: $worldty, $step| {
            use $crate::futures::future::FutureExt as _;
            std::panic::AssertUnwindSafe(async move { $($input)* })
                .catch_unwind()
                .map(|r| r.map_err($crate::TestError::PanicError))
                .boxed_local()
        }
    };
    // Async with block and immutable world
    (| $world:ident, $step:ident | $($input:tt)*) => {
        |$world, $step| {
            use $crate::futures::future::FutureExt as _;
            std::panic::AssertUnwindSafe(async move { $($input)* })
                .catch_unwind()
                .map(|r| r.map_err($crate::TestError::PanicError))
                .boxed_local()
        }
    };
    // Async with block and immutable world with type
    (| $world:ident : $worldty:path, $step:ident | $($input:tt)*) => {
        |$world: $worldty, $step| {
            use $crate::futures::future::FutureExt as _;
            std::panic::AssertUnwindSafe(async move { $($input)* })
                .catch_unwind()
                .map(|r| r.map_err($crate::TestError::PanicError))
                .boxed_local()
        }
    };
}
