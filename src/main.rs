use std::io;
use std::io::Write;
use std::process;
use triangulizer::{calc_hypotenuse, calc_other_side_with_hypotenuse};

fn readln() -> String {
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Entered string is incorrect");

    input.trim().to_string()
}


fn main() {
    let triangle_empty = r"
            ##                                              
            # #                                             
            #  #                                            
            #   #                                           
            #    #                                          
            #     #                                         
            #      #                                        
            #       #                                       
            #        #                                      
       a    #         #    c                                
            #          #                                    
            #           #                                   
            #            #                                  
            #             #                                 
            #              #                                
            #               #                               
            #################                               
                                                            
                    b                                ";
    let triangle_hypotenuse = r"
            ##                                              
            # #                                             
            #  #                                            
            #   #                                           
            #    #                                          
            #     #                                         
            #      #                                        
            #       #                                       
            #        #                                      
       a    #         #    (c) <-- CALCULATING THIS SIDE                                
            #          #                                    
            #           #                                   
            #            #                                  
            #             #                                 
            #              #                                
            #               #                               
            #################                               
                                                            
                    b                                ";
    println!("Triangulizer");
    println!("{}", triangle_empty);
    loop {
        println!("Please choose option:\n1. Calculate hypotenuse\n2. Calculate side from hypotenuse and one other side\n3. Exit");
        let choice = readln();
        if choice == "1" {
            println!("{}", triangle_hypotenuse);
            println!("not to scale");
            print!("Please enter side 1: ");
            let a: f64 = readln().parse().expect("Please enter a number");
            print!("Please enter side 2: ");
            let b: f64 = readln().parse().expect("Please enter a number");
            let hypotenuse = calc_hypotenuse(a, b);
            println!("Your hypotenuse is: {}", hypotenuse);
        } else if choice == "2" {
            print!("Please enter hypotenuse length: ");
            let hyp: f64 = readln().parse().expect("Please enter a number");
            print!("Enter other side's length: ");
            let o: f64 = readln().parse().expect("Please enter a number");
            if hyp < o {
                println!("Other side cannot be larger than hypotenuse!");
                continue;
            }
            let missing_side = calc_other_side_with_hypotenuse(o, hyp);
            println!("Your other side is: {}", missing_side);
        } else if choice == "3" {
            process::exit(0);
        } else {
            println!("Invalid choice");
        }
    }
}
