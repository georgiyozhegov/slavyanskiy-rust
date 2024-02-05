

macro_rules! second_argument 
{
    ($args:expr) => {
        {
            let mut args = $args;
            args.next();
            args.next()
        }
    }
}


macro_rules! error 
{
    ($message:expr) => {
        eprintln!("Ошибка: {}", $message);    
        process::exit(1);
    }
}


macro_rules! add_and_clear 
{
    ($buffer:expr, $tokens:expr) => {
        $tokens.push($buffer.to_string());
        $buffer.clear();
    }
} 


macro_rules! add_if_not_empty 
{
    ($buffer:expr, $tokens:expr) => {
        if !$buffer.is_empty() {
            $tokens.push($buffer.to_string());
        }
    }
} 
