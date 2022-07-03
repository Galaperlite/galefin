#![allow(non_snake_case)]
// imports
use std::env;
// file imports
#[path = "./help.rs"] mod help;

pub fn main()
{
    let args: Vec<String> = env::args().skip(1).collect(); // leave the ./
    let argsModifer: Vec<String> = env::args().skip(2).collect(); // skip the ./ and -_modifier
    let argLength: u8 = args.len().try_into().unwrap(); // get arg length
    let argString: String; // a added up of command line args
    // check args stuff
    // if arglenght is not greater than 0 (i.e no args) then make mistake error
    if !(argLength > 0)
    {
        help::mistake();
    }
    else
    {
        // modifier
        let mut modifier: &str = &args[0];
        let modifierFirstLetter = modifier.chars().nth(0).unwrap();
        // modifier if else statements
        // if it is -h or --help throuw help 
        if modifier.eq("-h") || modifier.eq("--help")
        {
            help::help();
        }
        // if it is -b or --bin download a bin
        else if modifier.eq("-b") || modifier.eq("--bin")
        {
            // modifier
            modifier = &args[1];
            // work to do
            println!("{}", modifier);
        }
        // if it is not any of the following modifiers or args just download the package
        else
        {
            // if they give a -__ something that does not match the above then throw error
            if modifierFirstLetter.eq(&'-')
            {
                help::mistake();
            }
            // download the package compiling way
            else
            {
                println!("{}", modifier);
            }
        }
    }
}

