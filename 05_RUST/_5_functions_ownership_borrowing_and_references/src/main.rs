#[derive(Debug)]
#[allow(dead_code)]
struct Technology {
    frontend: String,
    backend: String,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Project {
    name: String,
    tech: Technology,
    category: String,
    no_of_emp: u8,
    no_of_user: u16,
    is_completed: bool,
    organization: String,
}

#[allow(dead_code)]
impl Project {
    fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    fn set_frontend(&mut self, new_frontend: String) {
        self.tech.frontend = new_frontend;
    }

    fn set_backend(&mut self, new_backend: String) {
        self.tech.backend = new_backend;
    }

    fn set_category(&mut self, new_category: String) {
        self.category = new_category;
    }

    fn set_no_of_emp(&mut self, emp_no: u8) {
        self.no_of_emp = emp_no;
    }

    fn set_user_count(&mut self, user_count: u16) {
        self.no_of_user = user_count;
    }

    fn set_is_completed(&mut self, completed: bool) {
        self.is_completed = completed;
    }

    fn set_org_name(&mut self, org: String) {
        self.organization = org;
    }

    fn get_name(&self) -> String {
        format!("Name of Project is {}", self.name)
    }

    fn get_frontend(&self) -> String {
        self.tech.frontend.clone()
    }

    fn get_backend(&self) -> String {
        self.tech.backend.clone()
    }

    fn get_category(&self) -> String {
        self.category.clone()
    }

    fn get_no_of_emp(&self) -> u8 {
        self.no_of_emp
    }

    fn get_user_count(&self) -> u16 {
        self.no_of_user
    }

    fn get_is_completed(&self) -> String {
        if self.is_completed { format!("It is completed") } else { format!("It is not completed!") }
    }

    fn get_org_name(&self) -> String {
        self.organization.clone()
    }

    fn get_full_project(&self) -> String {
        format!(
            "1)Address : {:p},\n Name of project : {},\n Frontend Tech : {},\n Backend Tech : {},\n Project category : {},\n No of employee works on it : {},\n No of User : {},\n Project completed? : {},\n Organization name : {}\n\n2)Address : {:p},\n Name of project : {},\n Frontend Tech : {},\n Backend Tech : {},\n Project category : {},\n No of employee works on it : {},\n No of User : {},\n Project completed? : {},\n Organization name : {}",
            self,
            self.name,
            self.tech.frontend,
            self.tech.backend,
            self.category,
            self.no_of_emp,
            self.no_of_user,
            self.is_completed,  
            self.organization,
            &self,
            &self.name,
            &self.tech.frontend,
            &self.tech.backend,
            &self.category,
            &self.no_of_emp,
            &self.no_of_user,
            &self.is_completed,
            &self.organization
        )
    }

    fn get_full_project_wo_self(
        name: String,
        tech: Technology,
        category: String,
        emp: u8,
        user: u16,
        completed: bool,
        org: String
    ) -> String {
        format!(
            " Name of project : {},\n Frontend Tech : {},\n Backend Tech : {},\n Project category : {},\n No of employee works on it : {},\n No of User : {},\n Project completed? : {},\n Organization name : {}",
            name,
            tech.frontend,
            tech.backend,
            category,
            emp,
            user,
            completed,
            org
        )
    }

}

fn main() {
    let mut p1 = Project {
        name: "WebRTC".to_string(),
        tech: Technology {
            frontend: "Angular".to_string(),
            backend: "Rust".to_string(),
        },
        category: String::from("Communication"),
        no_of_emp: 50,
        no_of_user: 800,
        is_completed: false,
        organization: String::from("inventyv"),
    };

    let p2  = &mut p1;
    p2.set_name("New Project".to_string());
    println!("{}",p2.get_full_project());
    println!();
    
    p2.set_backend("NODE.JS".to_string());
    println!("{}",p2.get_full_project());
    println!();

    println!("{:p}",p2);
    println!("{:p}",&p2);
    println!("{:p}",&p1);
}
