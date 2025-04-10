// Publicly expose the mall module
pub mod mall;

pub use floor::store;
pub use mall::*;
pub use store::employee;

// Find and return the largest store in the mall by square meters
pub fn biggest_store(mall: mall::Mall) -> store::Store {
    let mut res: store::Store = store::Store::new("", 0, vec![]);

    for floor_a in mall.floors.iter() {
        for shop in floor_a.stores.iter() {
            if shop.square_meters > res.square_meters {
                res = shop.clone();
            }
        }
    }

    res
}

// Find and return all employees with the highest salary in the mall
pub fn highest_paid_employee(mall: mall::Mall) -> Vec<employee::Employee> {
    let mut res = vec![employee::Employee::new("", 0, 0, 0, 0.0)];

    for elem in mall.floors.iter() {
        for shop in elem.stores.iter() {
            for emp in shop.employees.clone().into_iter() {
                if emp.salary > res[0].salary {
                    res[0] = emp.clone();
                } else if emp.salary == res[0].salary {
                    res.push(emp.clone());
                }
            }
        }
    }

    res
}

// Return the total number of employees including guards
pub fn nbr_of_employees(mall: mall::Mall) -> usize {
    let mut res = 0;

    for floor_a in mall.floors.iter() {
        for shop in floor_a.stores.iter() {
            res += shop.employees.len();
        }
    }

    res + mall.guards.len()
}

// Hire additional guards if current security is insufficient
pub fn check_for_securities(mall: &mut mall::Mall, available_sec: Vec<guard::Guard>) {
    let mut size = 0;

    for floor in mall.floors.iter() {
        size += floor.size_limit;
    }

    let mut i = 0;
    while (mall.guards.len() as f64) < size as f64 / 200.0 {
        mall.hire_guard(available_sec[i].clone());
        i += 1;
    }
}

// Give raise or cut to employees based on their working hours
pub fn cut_or_raise(mall: &mut mall::Mall) {
    for (i, elem) in mall.clone().floors.iter().enumerate() {
        for (j, shop) in elem.stores.iter().enumerate() {
            for (z, emp) in shop.employees.iter().enumerate() {
                if emp.working_hours.1 - emp.working_hours.0 >= 10 {
                    mall.floors[i].stores[j].employees[z].raise(emp.salary * 0.1);
                } else {
                    mall.floors[i].stores[j].employees[z].cut(emp.salary * 0.1);
                }
            }
        }
    }
}