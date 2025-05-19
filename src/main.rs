use rust_sandbox::structs::employee::Employee;

fn main() {
    println!("Hello, world!");

    print_employee("Paul".to_string(), "Smith".to_string(), "Web Developer".to_string(), "Software Delivery".to_string());
    print_employee("Laura".to_string(), "Smith".to_string(), "Business Analyst".to_string(), "Product Delivery".to_string());
    print_employee("David".to_string(), "Smith".to_string(), "Product Manager".to_string(), "Product Delivery".to_string());
    print_employee("Jane".to_string(), "Smith".to_string(), "QA Tester".to_string(), "Software Delivery".to_string());
    print_employee("Harry".to_string(), "Smith".to_string(), "Accountant".to_string(), "Finance".to_string());
}

fn print_employee(first_name: String, last_name: String, role: String, department: String) {
    let employee = Employee {
        first_name,
        last_name,
        role,
        department
    };

    println!("{}", employee.to_string());
}
