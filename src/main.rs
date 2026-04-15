mod arguments_parsing;
use arguments_parsing::parse_args;

mod file_parsing;


fn main() -> std::io::Result<()>
{
    if let Some(argument) = parse_args() {
        println!("{}", argument);
        Ok(())
    } else {
        Ok(())
    }
}
