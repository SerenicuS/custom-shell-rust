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
}

pub enum OkFlavorResponse {
    FlavorDefaul,
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


/*
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
 */
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
            Self::ErrorListedFilesDoesNotExist => write!(f, "What kind of action you want me to do sweetie? Say it properly."),
            Self::ErrorProcessDoesNotExist => write!(f, "What kind of action you want me to do sweetie? Say it properly."),
            Self::ErrorIncompleteLaunchProcess => write!(f, "I can't do it properly if you won't say clearly what you desire sweetie."),

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

        

    }
}
