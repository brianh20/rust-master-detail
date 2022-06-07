# Master - Detail

The Master - Detail excercise is a well-known coding excercise. The excercise consists of building a list of elements by their name or title and displaying their details in another view. Think of this as a contact list or an address book.

## Running
- `cargo run` to run

## Crate Structure
- database.rs: connects to i/o filesystem
- listener.rs: listens to connections from outside
- tui.rs: terminal user interface
- render.rs: renders the components on the view/tui

# As a Rust excercise

This repository aims to display the implementation of a number of Rust features/paradigms. This means that perhaps some things could have been different, but there were some "artistic liberties" taken in order to make use of these features/paradigms. A number of them are:
- There is no connection to a DB, instead we are using a JSON file to practice reading I/O file and parsing
- The "publicly exposed" API does not interpret a result, it triggers the add person method
- Adding a person generates random data, there is no input for the person's values

## From the rust book
Topics taken from "Programming Rust 2nd Edition".
https://www.amazon.es/Programming-Rust-Fast-Systems-Development/dp/1492052590

This builds on knowledge/learning/practice from the Elevator Excercise and adds:
- Concurrency
- Generate a thread/channel
- Open/listen to port and handle connection (single thread)
- Read/write from external sources (file)

Pending for a follow-up excercise:
- Multiple threads
- Smart Pointers (RC, ARC)
- Asynchronous programming
- Macros

