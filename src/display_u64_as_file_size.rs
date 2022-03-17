pub trait DisplayFileSize {
    fn add_commas(&self) -> String;
    fn display_as_file_size(&self) -> String;
}

impl DisplayFileSize for u64 {
    fn add_commas(&self) -> String {
        let mut s = self.to_string();
        if s.len() < 2 { return s }
        let range = (1..s.len()-2).rev();
        for i in range.step_by(3) {
            s.insert(i, ',');
        }
        return s;
    }
    fn display_as_file_size(&self) -> String {
        let u64_comma = self.add_commas();
        let commas = u64_comma
            .split(',')
            .collect::<Vec<&str>>();
        let suffix = match commas.len() {
            1 => "bytes",
            2 => "Kb",
            3 => "Mb",
            4 => "Gb",
            5 => "Tb",
            _ => ""
        };
        if commas.len() > 1 {
            return format!("{}.{} {}", commas[0], commas[1], suffix);
        }
        else {
            return format!("{} {}", commas[0], suffix);
        }
    }
}