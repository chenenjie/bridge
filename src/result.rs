error_chain!{
    foreign_links{
        Io(::std::io::Error);
        Parse(::std::net::AddrParseError);
    }   
}
