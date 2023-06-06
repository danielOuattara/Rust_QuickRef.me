fn main() {
    /* If expression
    ------------------ */
    let case1: i32 = 81;
    let case2: i32 = 82;

    if case1 < case2 {
        println!("case1 is less than case2");
    }

    /* If...Else Expression
    ------------------------ */
    let case3 = 8;
    let case4 = 9;

    if case3 >= case4 {
        println!("case3 is greater than case4");
    } else {
        println!("case4 is greater than case3");
    }

    /* If...Else...if...Else Expression
    ------------------------------------ */
    let foo = 12;
    let bar = 13;

    if foo == bar {
        println!("foo is equal to bar");
    } else if foo < bar {
        println!("foo less than bar");
    } else if foo != bar {
        println!("foo is not equal to bar");
    } else {
        println!("Nothing");
    }

    /* If ...Let expression (literal pattern)
    ------------------------------------------ */
    let mut _arr1: [i64; 3] = [1, 2, 3];
    if let [1, 2, _] = _arr1 {
        println!("Works with number array");
    }

    let mut _arr2: [&str; 3] = ["one", "two", "three"];
    if let ["Apple", "Banana", _] = _arr2 {
        println!("Works with str array too");
    }

    let _tuple_1 = ("India", 7, 90, (1, 2, 3));
    if let (_, 7, 90, (1, 2, 3)) = _tuple_1 {
        println!("Works with tuples too");
    }

    let _tuple_2 = (9, 7, 89, 12, "OKAY");
    if let (9, 7, 89, 12, blank) = _tuple_2 {
        println!("Every {blank} mate ?");
    }

    let _tuple_3 = (89, 90, "Yes");
    if let (9, 89, "Yes") = _tuple_3 {
        println!(" YES ! Pattern did match");
    } else {
        println!("NO ! Pattern did not match");
    }

    /*  Match Expression
    ---------------------- */
    let day_of_week = 2;
    match day_of_week {
        1 => {
            println!("Its Monday");
        }
        2 => {
            println!("It's Tuesday");
        }
        3 => {
            println!("It's Wednesday");
        }
        4 => {
            println!("It's Thursday");
        }
        5 => {
            println!("It's Friday");
        }
        6 => {
            println!("It's Saturday");
        }
        7 => {
            println!("It's Sunday");
        }
        _ => {
            println!("Default!")
        }
    };

    /* Nested...If Expression
    ------------------------- */
    let nested_conditions = 89;
    if nested_conditions == 89 {
        let just_a_value = 98;
        if just_a_value >= 97 {
            println!("Greater than 97");
        }
    }

    let value = 5;
    if value > 0 && value < 10 {
        println!("{value} is between 0 and 10");
    }

    /* For loop
    ------------- */
    for mut _i in 0..16 {
        println!("The value of 'i' is :  {_i}");
    }

    /* While loop
    --------------- */
    let mut check = 0;
    while check < 11 {
        println!("Check is : {check}");
        check += 1;
        println!("After incrementing: {check}");

        if check == 10 {
            break; // stop while
        }
    }
    println!("-----------end of while loop");

    /* Loop keyword & infinite loop
    -------------------------------- */
    // loop {
    //     println!("hello world forever!");
    // }

    /* Break statement
    ------------------- */

    let mut i = 1;
    loop {
        println!("i is {i}");
        if i > 100 {
            break;
        }
        i *= 2;
    }

    println!("----------------------------- end of loop");

    /* Continue Statement
    ----------------------- */

    for (v, c) in (0..10 + 1).enumerate() {
        println!("The {c} number loop");
        if v == 9 {
            println!("Here we go continue ?");
            continue;
        }
        println!("the value of v is :{v}");
    }

    println!("----------------------------- end of loop");

    for (v, c) in (0..10 + 1).enumerate() {
        println!("The {c} number loop");
        if v == 5 {
            println!("Here we break");
            break;
        }
        println!("the value of v is :{v}");
    }
    println!("----------------------------- end of loop");
}
