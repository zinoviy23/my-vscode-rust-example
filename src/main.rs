/*--------------------------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information.
 *-------------------------------------------------------------------------------------------------------------*/

fn main() {
    let vars = std::env::vars()
        .into_iter()
        .map(|it| format!("{} = {}", it.0, it.1))
        .collect::<Vec<_>>();
    dbg!(vars);
    let name = "VS Code Remote - Containers";
    println!("Hello, {}!", name);
    println!("Hello, again, 2!");

    println!("Hello world 2");
}
