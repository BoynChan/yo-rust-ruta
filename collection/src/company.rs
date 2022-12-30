use std::collections::HashMap;
pub struct Company {
    deparment_employee: HashMap<String, Vec<String>>,
}

impl Company {
    pub fn add_employee(&mut self, sentence: &String) -> Option<bool> {
        let mut s_l: Vec<String> = Vec::new();
        for s in sentence.split_whitespace() {
            s_l.push(String::from(s));
        }
        if s_l.len() != 4 {
            return Option::None;
        }
        if s_l[0] != "Add" || s_l[2] != "to" {
            return Option::None;
        }
        match self.deparment_employee.get_mut(&s_l[3]) {
            Some(v) => v.push(s_l[1].clone()),
            None => {
                let _ = self
                    .deparment_employee
                    .insert(s_l[3].clone(), vec![s_l[1].clone()]);
            }
        }
        return Option::Some(true);
    }

    pub fn new() -> Company {
        Company {
            deparment_employee: HashMap::new(),
        }
    }

    pub fn deparment_list(&self, dep: &String) -> Option<&Vec<String>> {
        dbg!(&self.deparment_employee);
        self.deparment_employee.get(dep)
    }
}
