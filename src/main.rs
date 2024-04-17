/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/
use std::thread;
use std::time::Duration;
use std::fs;

fn main() {

    let thread_handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    println!("Before file open");
    fs::File::open("./package.json").expect("Something went wrong opening the file");
    println!("After file open");

    thread_handle.join().unwrap();
	println!("Thread finished");
}