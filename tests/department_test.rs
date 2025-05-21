#[cfg(test)]
mod department_test {
    use rust_sandbox::structs::department::Department;
    use rust_sandbox::structs::employee::Employee;

    #[test]
    fn test_department_to_string() {
        let department_name = "Software Delivery";
        let employee = Employee {
                                first_name: "Fred".to_string(), 
                                last_name: "Bloggs".to_string(), 
                                role: "Practice Lead".to_string(), 
                                department: department_name.to_string()};

        let department = Department{name: department_name.to_string(), practice_lead: employee};

        let expected_results = "Software Delivery. Practice Lead: Fred Bloggs".to_string();
        let actual_results = department.to_string();

        assert_eq!(actual_results, expected_results);


    }
}