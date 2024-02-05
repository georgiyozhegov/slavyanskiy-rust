use crate::alphabet::is_alpha;


#[derive(PartialEq)]
enum State 
{
    String,
    Identifier,
    Other
}


fn quote(c: char, state: &mut State, buffer: &mut String, tokens: &mut Vec<String>) 
{
    if *state == State::String {
        buffer.push(c);
        add_and_clear!(buffer, tokens);
        *state = State::Other;
    } else if !buffer.is_empty() {
        add_and_clear!(buffer, tokens);
        buffer.push(c);
        *state = State::String;
    }
}


pub fn tokenize(source: String) -> Vec<String> 
{
    let mut tokens = Vec::new();
    let mut buffer = String::new();
    let mut state = State::Other;
    for c in source.chars() {
        if c == '"' {
            quote(c, &mut state, &mut buffer, &mut tokens);
        } else if state == State::String {
            buffer.push(c);
        } else if is_alpha(&c) {
            if state != State::Identifier && !buffer.is_empty() {
                add_and_clear!(buffer, tokens);
            }
            buffer.push(c);
            state = State::Identifier;
        } else {
            if state == State::Identifier && !buffer.is_empty() {
                add_and_clear!(buffer, tokens);
            }
            buffer.push(c);
            state = State::Other;
        }
    }
    add_if_not_empty!(buffer, tokens);
    tokens
}
