//! # Project Demo (Rust Struct + Methods)
//!
//! This program demonstrates a `Project` struct with:
//! - Setter methods (`&mut self`)
//! - Getter methods (`&self`)
//! - Associated function (without `self`)

/// Represents a software/project entity with basic project details.
///
/// This struct stores:
/// - Project name
/// - Technology used
/// - Project category
/// - Number of employees working
/// - Number of users
/// - Completion status
/// - Organization name

#[derive(Debug, Clone)]
struct Technology {
    frontend: String,
    backend: String,
}
#[derive(Debug, Clone)]
struct Project {
    /// Name of the project.
    name: String,

    /// Technology used in the project.
    tech: Technology,

    /// Category/type of the project (example: Communication, Management).
    category: String,

    /// Number of employees working on the project.
    no_of_emp: u8,

    /// Number of users using the project.
    no_of_user: u16,

    /// Project completion status.
    is_completed: bool,

    /// Organization/company name.
    organization: String,
}

impl Project {
    /// Updates the name of the project.
    ///
    /// # Arguments
    /// - `new_name` : new name to set
    fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    /// Updates the frontend only.
    ///
    /// # Arguments
    /// - `new_frontend` : new frontend tech to set
    fn set_frontend(&mut self, new_frontend: String) {
        self.tech.frontend = new_frontend;
    }
    
    /// Updates the backend only.
    /// 
    /// # Arguments
    /// - `new_backend` : new backend tech to set
    fn set_backend(&mut self, new_backend: String) {
        self.tech.backend = new_backend;
    }

    /// Updates the category of the project.
    ///
    /// # Arguments
    /// - `new_category` : new project category
    fn set_categoy(&mut self, new_category: String) {
        self.category = new_category;
    }

    /// Updates the number of employees working on the project.
    ///
    /// # Arguments
    /// - `emp_no` : employee count
    fn set_no_of_emp(&mut self, emp_no: u8) {
        self.no_of_emp = emp_no;
    }

    /// Updates the number of users using the project.
    ///
    /// # Arguments
    /// - `user_count` : user count
    fn set_user_count(&mut self, user_count: u16) {
        self.no_of_user = user_count;
    }

    /// Updates whether the project is completed or not.
    ///
    /// # Arguments
    /// - `completed` : `true` = completed, `false` = not completed
    fn set_is_completed(&mut self, completed: bool) {
        self.is_completed = completed;
    }

    /// Updates the organization name of the project.
    ///
    /// # Arguments
    /// - `org` : organization name
    fn set_org_name(&mut self, org: String) {
        self.organization = org;
    }

    /// Returns a formatted string containing project name.
    ///
    /// # Returns
    /// A sentence describing the project name.
    ///
    /// # Example
    /// ```rust
    /// # struct Project { name: String, tech: String, category: String, no_of_emp: u8, no_of_user: u16, is_completed: bool, organization: String }
    /// # impl Project { fn get_name(&self) -> String { format!("Name of Project is {}", self.name) } }
    /// let p = Project {
    ///     name: "WebRTC".to_string(),
    ///     tech: "Rust".to_string(),
    ///     category: "Communication".to_string(),
    ///     no_of_emp: 10,
    ///     no_of_user: 200,
    ///     is_completed: false,
    ///     organization: "Inventyv".to_string(),
    /// };
    /// assert!(p.get_name().contains("WebRTC"));
    /// ```
    fn get_name(&self) -> String {
        format!("Name of Project is {}", self.name)
    }

    /// Returns frontend tech.
    fn get_frontend(&self) -> String {
        self.tech.frontend.clone()
    }

    /// Returns backend tech.
    fn get_backend(&self) -> String {
        self.tech.backend.clone()
    }

    /// Returns the category of the project.
    fn get_category(&self) -> String {
        self.category.clone()
    }

    /// Returns the number of employees working on the project.
    fn get_no_of_emp(&self) -> u8 {
        self.no_of_emp
    }

    /// Returns the number of users using the project.
    fn get_user_count(&self) -> u16 {
        self.no_of_user
    }

    /// Returns a user-friendly completion status message.
    ///
    /// # Returns
    /// - `"It is completed"` if completed
    /// - `"It is not completed!"` if not completed
    fn get_is_completed(&self) -> String {
        if self.is_completed { format!("It is completed") } else { format!("It is not completed!") }
    }

    /// Returns the organization name.
    fn get_org_name(&self) -> String {
        self.organization.clone()
    }

    /// Returns the full project information in a formatted multi-line string.
    fn get_full_project(&self) -> String {
        format!(
            " Name of project : {},\n Frontend Tech : {},\n Backend Tech : {},\n Project category : {},\n No of employee works on it : {},\n No of User : {},\n Project completed? : {},\n Organization name : {}",
            self.name,
            self.tech.frontend,
            self.tech.backend,
            self.category,
            self.no_of_emp,
            self.no_of_user,
            self.is_completed,
            self.organization
        )
    }

    /// Associated function that returns formatted project data **without using `self`**.
    ///
    /// This is useful when you want to generate output without creating a `Project` object.
    ///
    /// # Arguments
    /// - `name` : project name
    /// - `tech` : tech stack
    /// - `category` : project category
    /// - `emp` : number of employees
    /// - `user` : number of users
    /// - `completed` : completion status
    /// - `org` : organization name
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

pub fn main() {
    println!();
    println!("This code execution from _3_struct.");
    let p1 = Project {
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

    println!("{}", p1.get_name());
    println!("Frontend: {}", p1.get_frontend());
    println!("Backend: {}", p1.get_backend());
    println!("{}", p1.get_category());
    println!("{}", p1.get_no_of_emp());
    println!("{}", p1.get_user_count());
    println!("{}", p1.get_is_completed());
    println!("{}", p1.get_org_name());
    println!();
    println!("{}", p1.get_full_project());

    let mut custom_project = Project {
        name: "".to_string(),
        tech: Technology {
            frontend: "".to_string(),
            backend: "".to_string(),
        },
        category: String::from(""),
        no_of_emp: 0,
        no_of_user: 0,
        is_completed: false,
        organization: String::from(""),
    };

    println!();
    println!("Empty object with default value set.........");
    println!("{}", custom_project.get_full_project());

    custom_project.set_name("Intern's Project".to_string());
    custom_project.set_frontend("ANGULAR".to_string());
    custom_project.set_backend("RUST".to_string());
    custom_project.set_categoy("Submission project".to_string());
    custom_project.set_no_of_emp(30);
    custom_project.set_user_count(800);
    custom_project.set_is_completed(true);
    custom_project.set_org_name("INVENTYV".to_string());

    println!();
    println!("After setter.........");
    println!("{}", custom_project.get_full_project());

    println!();
    println!("print without self.........");
    println!(
        "{}",
        Project::get_full_project_wo_self(
            "CRM".to_string(),
            Technology {
                frontend: "Angular".to_string(),
                backend: "Rust".to_string(),
            },
            "Management".to_string(),
            60,
            400,
            true,
            "inventyv".to_string()
        )
    );
}
