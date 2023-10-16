use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::Value;
use std::fs::File;
use std::io::Read;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(_event: LambdaEvent<Value>) -> Result<Value, Error> {
    let mut file = File::open("/var/task/output.log").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    Ok(contents.split(",").collect())
}