pub struct Employee {
    pub first_name: String,
    pub last_name: String,
    pub role: String,
    pub department: String
}

impl Employee {
    pub fn to_string(&self) -> String {
        format!("{} {}. {} - {}", 
        &self.first_name.to_string(), 
        &self.last_name.to_string(), 
        &self.role.to_string(), 
        &self.department.to_string())
    }

    pub fn surname_forename_to_string(&self) -> String {
        format!("{}, {}", 
        &self.last_name.to_string(),
        &self.first_name.to_string())
    }
}