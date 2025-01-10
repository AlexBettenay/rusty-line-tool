use std::thread;
use std::sync::{Arc, Mutex};
use std::io;

//demonstration of object oriented programming in Rust

// LinePoints class. Accepts two points along a line
struct LinePoints {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
}

impl LinePoints {
    //creates new line
    fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Self{
        LinePoints { x1, y1, x2, y2}
        
    }
    //return the slope and y intersect of the line
    fn equation(&self) -> (f64, f64){
        let slope = (self.y2 - self.y1) / (self.x2 - self.x1);
        let y_intersect = self.y1 - slope * self.x1;
        (slope, y_intersect)
    }
}

//Line class accepts slope and y intersect
struct Line {
    slope: f64,
    y_intersect: f64
}

impl Line {
    //create new line
    fn new(slope: f64, y_intersect: f64) -> Self{
        Line { slope, y_intersect }
    }

    //finds the intersect of two lines. If lines are parrellel, returns None
    fn intersect(&self, other: &Self) -> Option<(f64, f64)> {
        if self.slope == other.slope {
            None
        
        } else {
            let x = (other.y_intersect - self.y_intersect) / (self.slope - other.slope);
            let y = self.slope * x + self.y_intersect;
            Some((x, y))
        }
    }
}

fn main() {

    println!("Welcome to the Line Tool");
    println!("");

    loop {
        println!("Please select an option:");
        println!("1. Find Equation of line from two sets of Coordinates");
        println!("2. Find intersection of two lines from Equations");
        println!("3. Find intersection of two lines from two sets of coordinates from each line");
        println!("4. Exit");

        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() { //example of the match function
            "1" => function1(),
            "2" => function2(),
            "3" => function3(),
            "4" => panic!("the Disco"), //call panic to exit the program to show an example of Panic
                                        //Panic! at the disco. get it? lol
            _   => println!("Invalid option"),
        }
    }
}

fn function1() {

    let mut points = Vec::new();

    // Prompt the user to enter the coordinates of the two points on the line
    for i in 1..=2 {
        loop {
            let mut input = String::new();
            println!("Enter x and y coordinates for point {} (separated by a space):", i);
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            //make sure the input is numbers
            let coords: Vec<f64> = input
                .trim()
                .split_whitespace()
                .map(|x| x.parse().unwrap_or_else(|_| {
                    println!("Invalid input. Please enter a valid number.");
                    0.0
                }))
                .collect();

            //make sure the input is two numbers
            if coords.len() == 2 {
                points.push(coords);
                break;
            } else {
                println!("Invalid input. Please enter two numbers separated by a space.");
            }
        }
    }

    let line = LinePoints::new(points[0][0], points[0][1], points[1][0], points[1][1]);
    let result = line.equation();

    println!("The equation of the line is: y = {}x + {}", result.0, result.1);

}

fn function2() {
    let mut equations = Vec::new();

    for i in 1..=2 {
        loop {
            let mut input = String::new();
            println!("Enter the equation of line {} in the form 'y = mx + c':", i);
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            match input.trim() { //example of match function
                // Extract the slope and y-intercept from the input string
                s if s.starts_with("y = ") => {
                    let mut parts = s[4..].splitn(2, " + ");
                    let slope: &str = parts.next().unwrap();
                    let mut s = String::from(slope);
                    s.truncate(s.len() - 1);
                    let y_intercept: &str = parts.next().unwrap();
                    match s.parse::<f64>() {
                        Ok(slope) => {
                            equations.push(slope);
                            match y_intercept.parse::<f64>() {
                                Ok(y_intercept) => {
                                    equations.push(y_intercept);
                                    break;
                                },
                                Err(_) => println!("Invalid input for y-intercept. Please try again.")
                            }
                        },
                        Err(_) => println!("Invalid input for slope. Please try again.")
                    }
                }
                // Prompt the user to enter a valid input
                _ => println!("Invalid input. Please enter the equation of a line in the form 'y = mx + c'."),
            }
        }
    }

    let line1 = Line::new(equations[0], equations[1]);
    let line2 = Line::new(equations[2], equations[3]);

    let intersection = line1.intersect(&line2); // & denotes a reference to the data at line2

    match intersection {
        // The division was valid
        Some((x, y)) => println!("Result: x = {}, y = {}", x, y),
        // The division was invalid
        None    => println!("Lines are Parallel"),
    }
}

fn function3() {

    //accept input of 4 sets of
    let mut points = Vec::new();

    let mut increm = 1;
    let mut line_num: i32;

    for i in 1..=4 {
        loop {
            match i {
                2 => line_num = 2,
                4 => line_num = 2,
                _ => line_num = 1
            }
        
            let mut input = String::new();
            println!("Enter x and y coordinates for point {} on line {} (separated by a space):", line_num, increm);
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            //make sure the input is numbers
            let coords: Vec<f64> = input
                .trim()
                .split_whitespace()
                .map(|x| x.parse().unwrap_or_else(|_| {
                    println!("Invalid input. Please enter a valid number.");
                    0.0
                }))
                .collect();

            //make sure the input is two numbers`
            if coords.len() == 2 {
                points.push(coords);
                
                if i > 1 && i % 2 == 0{
                    increm += 1;
                }
                break;
            } else {
                println!("Invalid input. Please enter two numbers separated by a space.");
            }
        }
    }



    let line1 = Arc::new(Mutex::new(LinePoints::new(points[0][0], points[0][1], points[1][0], points[1][1])));
    let line2 = Arc::new(Mutex::new(LinePoints::new(points[2][0], points[2][1], points[3][0], points[3][1])));

    //Example of multithreading in Rust
    //create thread 1
    let handle1 = thread::spawn({
        let line1 = Arc::clone(&line1);
        move || {
            let equation = {
                let line = line1.lock().unwrap();
                line.equation()
            };
            equation
        }
    });

    //create thread 2
    let handle2 = thread::spawn({
        let line2 = Arc::clone(&line2);
        move || {
            let equation = {
                let line = line2.lock().unwrap();
                line.equation()
            };
            equation
        }
    });

    let equation = handle1.join().unwrap();
    let equation2 = handle2.join().unwrap();

    let line1 = Line::new(equation.0, equation.1);
    let line2 = Line::new(equation2.0, equation2.1);

    let intersection = line1.intersect(&line2);

    match intersection {
        // The division was valid
        Some((x, y)) => println!("Result: x = {}, y = {}", x, y),
        // The division was invalid
        None    => println!("Lines are Parallel"),
    }
}
