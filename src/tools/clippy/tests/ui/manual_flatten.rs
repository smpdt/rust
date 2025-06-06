#![warn(clippy::manual_flatten)]
#![allow(clippy::useless_vec, clippy::uninlined_format_args)]
//@no-rustfix
fn main() {
    // Test for loop over implicitly adjusted `Iterator` with `if let` expression
    let x = vec![Some(1), Some(2), Some(3)];
    for n in x {
        //~^ manual_flatten

        if let Some(y) = n {
            println!("{}", y);
        }
    }

    // Test for loop over implicitly adjusted `Iterator` with `if let` statement
    let y: Vec<Result<i32, i32>> = vec![];
    for n in y.clone() {
        //~^ manual_flatten

        if let Ok(n) = n {
            println!("{}", n);
        };
    }

    // Test for loop over by reference
    for n in &y {
        //~^ manual_flatten

        if let Ok(n) = n {
            println!("{}", n);
        }
    }

    // Test for loop over an implicit reference
    let z = &y;
    for n in z {
        //~^ manual_flatten

        if let Ok(n) = n {
            println!("{}", n);
        }
    }

    // Test for loop over `Iterator` with `if let` expression
    let z = vec![Some(1), Some(2), Some(3)];
    let z = z.iter();
    for n in z {
        //~^ manual_flatten

        if let Some(m) = n {
            println!("{}", m);
        }
    }

    // Using the `None` variant should not trigger the lint
    // Note: for an autofixable suggestion, the binding in the for loop has to take the
    // name of the binding in the `if let`
    let z = vec![Some(1), Some(2), Some(3)];
    for n in z {
        if n.is_none() {
            println!("Nada.");
        }
    }

    // Using the `Err` variant should not trigger the lint
    for n in y.clone() {
        if let Err(e) = n {
            println!("Oops: {}!", e);
        }
    }

    // Having an else clause should not trigger the lint
    for n in y.clone() {
        if let Ok(n) = n {
            println!("{}", n);
        } else {
            println!("Oops!");
        }
    }

    let vec_of_ref = vec![&Some(1)];
    for n in &vec_of_ref {
        //~^ manual_flatten

        if let Some(n) = n {
            println!("{:?}", n);
        }
    }

    let vec_of_ref = &vec_of_ref;
    for n in vec_of_ref {
        //~^ manual_flatten

        if let Some(n) = n {
            println!("{:?}", n);
        }
    }

    let slice_of_ref = &[&Some(1)];
    for n in slice_of_ref {
        //~^ manual_flatten

        if let Some(n) = n {
            println!("{:?}", n);
        }
    }

    struct Test {
        a: usize,
    }

    let mut vec_of_struct = [Some(Test { a: 1 }), None];

    // Usage of `if let` expression should not trigger lint
    for n in vec_of_struct.iter_mut() {
        if let Some(z) = n {
            *n = None;
        }
    }

    // Using manual flatten should not trigger the lint
    for n in vec![Some(1), Some(2), Some(3)].iter().flatten() {
        println!("{}", n);
    }

    // Using nested `Some` pattern should not trigger the lint
    for n in vec![Some((1, Some(2)))] {
        if let Some((_, Some(n))) = n {
            println!("{}", n);
        }
    }

    run_unformatted_tests();
}

#[rustfmt::skip]
fn run_unformatted_tests() {
    // Skip rustfmt here on purpose so the suggestion does not fit in one line
    for n in vec![
    //~^ manual_flatten

        Some(1),
        Some(2),
        Some(3)
    ].iter() {
        if let Some(n) = n {
            println!("{:?}", n);
        }
    }
}
