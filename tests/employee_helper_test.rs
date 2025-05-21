mod employee_helper_test {
    use rust_sandbox::structs::employee::Employee;
    use rust_sandbox::helpers::employee_helper::build_employee;
    
    #[test]
    fn should_return_employee_instance() {
        let first_name = "Paul";
        let last_name = "Smith";
        let role = "Developer";
        let department = "Software Delivery";

        let _expected_result = Employee {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            role: role.to_string(),
            department: department.to_string()
        };

        let _actual_result = build_employee(first_name.to_string(), last_name.to_string(), role.to_string(), department.to_string());
        
        assert_eq!(_actual_result.to_string(), _expected_result.to_string());
        print!("{}", _actual_result.to_string());
    }

}