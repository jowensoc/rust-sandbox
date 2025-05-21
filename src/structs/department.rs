use crate::structs::employee::Employee;

pub struct Department {
    pub name: String,
    pub practice_lead: Employee
}

impl Department {

    pub fn to_string(&self) -> String {
            format!("{}. Practice Lead: {}", 
            &self.name.to_string(), 
            &self.practice_lead.name_to_string())
    }

}