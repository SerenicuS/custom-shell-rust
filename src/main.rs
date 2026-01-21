use std::env::set_current_dir;
use std::fmt;
use std::io;
use std::io::Write;
pub enum ShellOkResponse {
    OkGeneral,
    OkDeleteFile,
    OkCreateFile,
    OkReturnDirectory,
    OkListedFiles,
    OkMoveDirectory,
    OkTerminate,
    OkCreateDirectory,
    OkDeleteDirectory,
    OkReadFile,
    OkOpenedFile,
    OkLaunchProcess,
}

pub enum ShellErrorResponse {
    ErrorGeneral,
    ErrorBadArgs,
    ErrorTooManyArgs,
    ErrorSystem,
    ErrorFileDoesNotExist,
    ErrorPermissionDenied,
    ErrorRootDirectory,
    ErrorListedFilesDoesNotExist,
    ErrorProcessDoesNotExist,
    ErrorIncompleteLaunchProcess,
    ErrorDirectoryDoesNotExist,
}

pub enum OkFlavorResponse {
    FlavorDefault,
    FlavorIpConfigAttempt,
    FlavorPingAttempt,
}

pub enum BadFlavorResponse {
    FlavorUserZero,
    FlavorUserSimilar,
    FlavorUserWrong,
}

pub enum GeneralFlavorResponse {
    FlavorMenu1,
    FlavorMenu2,
    FlavorMenu3,
    FlavorExit,
    FlavorRegister1,
    FlavorRegister2,
    FlavorStart1,
    FlavorChaosNotHear,
    FlavorChaosWrongCommand,
}

pub enum MiscFlavor {
    FlavorWhitespace1,
    FlavorWhiteSpace2,
    FlavorUserReply1,
}

const HELP_MENU: &str = r#"
    You are too greedy.
    ---------------
     1. tellme                      ->    List Commands
     2. mayileave                   ->    Exit the Terminal
     3. iamhere                     ->    Locate current Directory
     4. mommy?                      ->    List Files in current Directory
     5. walkwithme <filename>       ->    Move to another Directory
     6. goback                      ->    Return to Previous Directory
     7. canihave <filename>         ->    Create File
     8. takethe <filename>          ->    Delete File
     9. letusplayhouse <directory>  ->    Create a Directory
    10. removethehouse <directory>  ->    Delete a Directory
    11. openthis <filename>         ->    Open the File
    12. readthis <filename>         ->    Read the File's contents
    13. doxxme                      ->    Windows Ip Configuration
    14. callmeplease <ip/dns>       ->    Ping device
    ---------------
    "#;


impl fmt::Display for ShellErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            Self::ErrorGeneral => write!(f, "Tell me the instructions correctly sweetie."),
            Self::ErrorBadArgs => write!(f, "You didn't complete your sentence sweetie, are you flustered?."),
            Self::ErrorTooManyArgs => write!(f, "Greedy Aren't you?."),
            Self::ErrorSystem => write!(f, "Oh my, the system crashed."),
            Self::ErrorFileDoesNotExist => write!(f, "You are not allowed to do that sweetie?"),
            Self::ErrorPermissionDenied => write!(f, "This is as far as we can go sweetie."),
            Self::ErrorRootDirectory => write!(f, "Hmmm, no one is here, only your mommy right?."),
            Self::ErrorListedFilesDoesNotExist => write!(f, "Hmmm, no one is here, only your mommy right?."),
            Self::ErrorProcessDoesNotExist => write!(f, "What kind of action you want me to do sweetie? Say it properly."),
            Self::ErrorIncompleteLaunchProcess => write!(f, "I can't do it properly if you won't say clearly what you desire sweetie."),
            Self::ErrorDirectoryDoesNotExist => write!(f, "I cannot find the house sweetie."),

        }
    }
}


impl fmt::Display for GeneralFlavorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            Self::FlavorMenu1 => write!(f, "Hello To my Custom Shell!"),
            Self::FlavorMenu2 => write!(f, "It is made by \"HiveMind\" to showcase my talents ^^."),
            Self::FlavorMenu3 => write!(f, "Press Y(Manipulation) or Z(Default) key to start using it. "),
            Self::FlavorExit => write!(f, "Exiting....."),
            Self::FlavorRegister1 => write!(f, "Do you know your name?"),
            Self::FlavorRegister2 => write!(f, "Tell me your name sweetie.. "),
            Self::FlavorStart1 => write!(f, "Good boy, always listen to your mommy."),
            Self::FlavorChaosNotHear => write!(f, "Are you talking sweetie? I did not hear you. Can you repeat that again?"),
            Self::FlavorChaosWrongCommand => write!(f, "You already told me that, you are so impatient sweetie."),

        }
    }
}


fn main() {
    println!("{}", GeneralFlavorResponse::FlavorMenu1);
    println!("{}", GeneralFlavorResponse::FlavorMenu2);
    println!("{}", GeneralFlavorResponse::FlavorMenu3);

    let mut input = String::new();// lineBuffer
    println!();
    io::stdin().read_line(&mut input).expect(&GeneralFlavorResponse::FlavorExit.to_string());

    match input.trim(){
        "Y" => shell_start_default(input),
        _ => println!("{}", GeneralFlavorResponse::FlavorExit),
    }


}

fn shell_start_default(mut input: String){
    println!("{}", GeneralFlavorResponse::FlavorStart1);

    loop{
        input.clear();
        print!(">");
        io::stdout().flush().unwrap(); // This exists because without this, the ">" will not show up and get stuck.
        io::stdin().read_line(&mut input).expect(&ShellErrorResponse::ErrorGeneral.to_string());

        shell_attempt_command(&input)

    }
}

fn shell_attempt_command(input: &str){
    let clean_input = input.trim();
    let args: Vec<&str> = clean_input.split_whitespace().collect();

    if args.is_empty(){
        println!("{}", ShellErrorResponse::ErrorBadArgs);
        return;
    }


    match args[0]{
        "tellme" => shell_help(),
        "mayileave" => std::process::exit(0),
        "iamhere" => shell_get_directory(),
        "mommy?" => shell_list_files_in_directory(),
        "walkwithme" => {
            if args.len() > 1 {
                shell_move_directory(&args[1])
            }
            else{
                println!("{}", ShellErrorResponse::ErrorBadArgs);
            };
        }

        _ => println!("{}", ShellErrorResponse::ErrorGeneral),
    }

}

fn shell_list_files_in_directory(){
    let files = std::fs::read_dir(".").expect(&ShellErrorResponse::ErrorListedFilesDoesNotExist.to_string());

    for entry in files{
        let entry = entry.expect(&ShellErrorResponse::ErrorPermissionDenied.to_string());
        println!("{}", entry.path().display());
    }
}
fn shell_get_directory(){
    let dir = std::env::current_dir().expect(&ShellErrorResponse::ErrorRootDirectory.to_string());
    println!("{}", dir.display());

}


fn shell_get_directory_return() -> String{
    let dir = std::env::current_dir().expect(&ShellErrorResponse::ErrorRootDirectory.to_string());

    dir.display().to_string()

}
fn shell_help(){
    println!("{}", HELP_MENU);
}

fn shell_move_directory(path: &str){
    match set_current_dir(path){
        Ok(_) => println!("Moved Inside: {}", shell_get_directory_return()),
        Err(_) => println!("{}", ShellErrorResponse::ErrorSystem),
    }
}