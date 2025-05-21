use crate::structs::employee::Employee;

pub fn build_employee(first_name: String, last_name: String, role: String, department: String) -> Employee {
    Employee {
        first_name: first_name,
        last_name: last_name, 
        role: role,
        department: department
    }
}