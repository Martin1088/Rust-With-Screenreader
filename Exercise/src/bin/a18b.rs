// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this


enum Department {
    MaintenanceCrews,
    MarketingDepartmentEmployees,
    Managers,
    LineSupervisors,
    KitchenStaff,
    AssemblyTechnicians,
}
enum Status {
    Activ,
    Terminated,
}
struct Employees {
    name: String,
    position: Department,
    status: Status,
}

fn check(employee: &Employees) -> Result<(), String> {
    println!("Name: {:?}", employee.name);
    match employee.status {
        Status::Terminated => return Err("terminated".to_owned()),
        _ => (),
    }
    match employee.position {
        Department::MaintenanceCrews => Ok(()),
        Department::MarketingDepartmentEmployees => Ok(()),
        Department::Managers => Ok(()),
        _ => Err("invalid position".to_owned())
    }
}

fn print_check(employee: &Employees) -> Result<(), String> {
    let result = check(employee)?;
    println!("access ok");    
    Ok(())
}

fn main() {
    let merlin = Employees{
        name: "Merlin Jurk".to_owned(),
        position: Department::KitchenStaff,
        status: Status::Activ,
    };
    let lilo = Employees{
        name: "Lilo Seene".to_owned(),
        position: Department::Managers,
        status: Status::Activ,
    };

    match print_check(&merlin) {
        Err(e) => println!("access denied {:?}", e),
        _ => (),
    }

    match print_check(&lilo) {
        Err(e) => println!("access denied {:?}", e),
        _ => (),
    }

}
