Project 2:

About:
     This file analysis prgram is written with two approaches, linear approach and a multithreaded approach. 

Execution process:

    The program is written in Rust and already compiled. It can be run in terminal* using from the root directory with following commands:
    ./version1/target/debug/version1
    ./version2/target/debug/version2
    Tried in linux terminal. (Not sure aobut the execution process in MacOS or Windows terminal)
    In order to compile and execute the program, I used "cargo run" from the each versions respective directory. In order to build "cargo build" can be used from each versions respective directories and "cargo run" can be used"  Make sure rust and cargo is available to compile and execute the program. 
    
File structure

    project2
        version1  (Linear Approach)
            data/
                BRANCH1/
                    branch_weekly_sales.txt
                BRANCH2/
                    branch_weekly_sales.txt
                ...
                BRANCH40/
                    branch_weekly_sales.txt
        
                weekly_summary/
                    weekly_sales_summary.txt //output file after the input files are analyzed
        
            src/
                lib_mod.rs
                main.rs
            
            target/ #build output directory
                    
            Cargo.toml
            Cargo.lock
            log.txt  #log files for what happens during the process. 
            
        version2 (Multi-threaded Approach)
            data/
                BRANCH1/
                    branch_weekly_sales.txt
                BRANCH2/
                    branch_weekly_sales.txt
                ...
                BRANCH40/
                    branch_weekly_sales.txt
    
                weekly_summary/
                    weekly_sales_summary.txt //output file after the input files are analyzed
    
            src/
                lib_mod.rs
                main.rs
            
            target/ #build output directory
                    
            Cargo.toml
            Cargo.lock
            log.txt  #log files for what happens during the process.

        README.md
        create.sh // shell script used to create 40 branch directories with the input file. Uses random from 1-20 for product sales, so every execution wouldn't give the same result.
