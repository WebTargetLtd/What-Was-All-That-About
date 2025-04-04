# What was all that about?

A utility crate we use iternally at WebTarget to print formatted output and gather metrics in our Rust projects.

Timer macros 
- TC :: Timer create
- TD :: Timer duration
- TE :: Timer end
- TR :: Timer rate

An example usage that uses console's style:

```rust
    // Create a timer
    let tinsert: String = "InsertingRecords".to_string();
    let mut loadtimer = TC!(&tinsert); // Timer create.

    // Do very important work . . . .
    let rows = a_function_that_takes_an_awfully_long_time();

    // End the timer here
    TE!(&tinsert, loadtimer); // End the timer.

    // Display the duration and rate
    say(format!(
        "Inserted {:?} records in {:?} ms at {:?} /s",
        style(rows).green(),
        style(TD!(&tinsert, loadtimer)).green(),
        style(TR!(&tinsert, loadtimer, rows.try_into()?)).green()
    ).as_str())?;

```
