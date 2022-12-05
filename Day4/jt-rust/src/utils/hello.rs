pub mod hello{
    pub fn respond_hello<'a>()-> &'a str{
        let response = "General Kenobi!";
        println!("{}", response);
        response
    }
}