

struct Reader {
    file : String,
}


impl Reader{
    fn new(path : String)->Reader{
        Reader{
            file : path;
        }
    }
}
