fn main() {
    println!("Hello, world!");

    another_function();

    yep_another_function(100);

    more_parameters_function(100,200);

    statement_can_not_assign_to_variable(10);

    expression_can_assign_to_variable(10);

    return_function();

    return_last_expression();
}

fn another_function(){
    println!("Another function.");
}

fn yep_another_function(x: i32){
    println!("Another function, parameter is:{}.", x);
}

fn more_parameters_function(x: i32, y: i32){
    println!("Another function, parameter is:x:{}, y:{}.", x, y);
}

fn statement_can_not_assign_to_variable(_x:i32){
    // let y = (let x=12);
}

fn expression_can_assign_to_variable(x:i32){
    let y = x;
    println!("The value of y:{}", y);

    let e =  {
        let z = 10;
        z + 1
        // println!("block is expression");
    };

    println!("The value of block expression:{}", e);
}

fn return_function(){
    let _x = 10;
    println!("return will ignore the following code");
    return;

    println!("can not come here");

}

fn return_last_expression(){
    let v =  last_expression();
    println!("The value of function last expression:{}", v);

    
    let v = last_block_expression();
    println!("The value of function last block expression:{}", v);
}

fn last_expression()->i32{
    let _x = 10;

    let z = 10;
    z + 1
}

fn last_block_expression()->i32{
    let _x = 10;
    {
        let z = 10;
        if z < 10 {
            z + 1
        } else {
            z + 2
        }
    }
}