trait Employee {
    fn show_details(&self);
}

struct Developer {
    name: String,
    id: u32,
}

impl Developer {
    fn new(name: &str, id: u32) -> Self {
        Developer {
            name: name.to_string(),
            id,
        }
    }
}

impl Employee for Developer {
    fn show_details(&self) {
        println!("Developer: {} with ID: {}", self.name, self.id);
    }
}

struct Manager {
    name: String,
    id: u32,
}

impl Manager {
    fn new(name: &str, id: u32) -> Self {
        Manager {
            name: name.to_string(),
            id,
        }
    }
}

impl Employee for Manager {
    fn show_details(&self) {
        println!("Manager: {} with ID: {}", self.name, self.id);
    }
}

struct Company {
    employees: Vec<Box<dyn Employee>>,
}

impl Company {
    fn new() -> Self {
        Company {
            employees: Vec::new(),
        }
    }

    fn add_employee(&mut self, employee: Box<dyn Employee>) {
        self.employees.push(employee);
    }

    fn show_employee_details(&self) {
        for employee in &self.employees {
            employee.show_details();
        }
    }
}
