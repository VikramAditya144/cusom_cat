# Understanding std::env in Rust 
Imagine your computer is like a big, busy classroom. Every time you run a program, it's like a new student walking in to do some work.

The std::env module is like a helpful teacher's assistant that helps this student (your program) get all the information it needs to start working.

## 1. Environment Variables 
Think of environment variables as a public bulletin board in the classroom. It contains sticky notes with important messages that every student (program) can read.

### Getting Information from the Board
env::var("KEY")
Like asking: “Can you find the sticky note that says LUNCH_MENU?”

Returns the value if it exists, e.g., "Pizza today!"

Or an error if the key doesn't exist.

env::vars()
Like saying: “Read me all the notes on the bulletin board.”

Returns all key-value pairs available.

### Putting Information on the Board
env::set_var("KEY", "VALUE")
Like a student writing their own sticky note (e.g., "MY_COLOR": "Blue") and posting it.

Only the program and its subprocesses (friends) can see it.

env::remove_var("KEY")
Like removing the sticky note for everyone in your own space (and subprocesses).

## 2. Command-Line Arguments 
These are like notes packed in a student’s lunchbox—custom messages given to a program when it's launched.

env::args()
Like reading the lunchbox notes.

If the program is run as my_program.exe jump high,
then args() will return:
["my_program.exe", "jump", "high"]

The first element is always the program name.

env::args_os()
Like lunchbox notes written in other languages or special characters.

Useful for OS-level strings like file paths.

## 3. Current Directory 
This refers to where the student is sitting (the program's working directory).

env::current_dir()
Like asking: “Where are you sitting right now?”

Example: “I’m at the corner desk (C:/Users/Alice/Documents)”

env::set_current_dir("PATH")
Like telling the student: “Please move to the big table in the center of the room.”

### Summary 
Concept	Analogy	Function
Environment Variables	Bulletin Board	var(), vars()
Setting Env Variables	Add note to board	set_var(), remove_var()
Command-Line Arguments	Notes in the lunchbox	args(), args_os()
Current Directory	Student's seating position	current_dir(), set_current_dir()

