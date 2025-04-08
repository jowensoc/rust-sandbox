#[cfg(test)]
mod employee_test {
    use rust_sandbox::structs::employee::Employee;

    #[test]
    fn should_return_employee_details() {
        let _employee = Employee {
            first_name: "Paul".to_string(),
            last_name: "Smith".to_string(),
            role: "Web Developer".to_string(),
            department: "Software Delivery".to_string()
        };

        let _expected_result = "Paul Smith. Web Developer - Software Delivery".to_string();
        let _actual_result = _employee.to_string();
        
        assert_eq!(_actual_result, _expected_result);
    }

}