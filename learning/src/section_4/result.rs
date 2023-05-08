#[derive(Debug)]
struct DivisionByZeroError;


pub fn result_example()
{
    for i in 1..15
    {
        let result = div_s(10.0, i as f64);
        match result
        {
            Ok(x) => println!("10 / {} = {}", i, x),
            Err(_) => println!("10 / {} = Error", i)
        }
    }
}

fn div_s(dividend: f64, divisor: f64) -> Result<f64, DivisionByZeroError>
{
    if divisor == 0.0
    {
        Err(DivisionByZeroError)
    }
    else
    {
        Ok(dividend / divisor)
    }
}