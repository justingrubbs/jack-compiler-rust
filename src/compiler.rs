

pub struct Compiler {
    stuff: String,
    file_name: String,

    ctx: String,
    var_counts: (i16,i16),
    label_count: i16,
    instruction_stack: Vec<String>,
}

impl Compiler {
    pub fn new(filename: String, classname: String) -> Self {
        Self {
            filename,
            classname,
            ctx: HashMap::new(),
            var_counts: (0, 0, 0, 0),
            instructions: Vec::new(),
            label_count: 0,
        }
    }
}