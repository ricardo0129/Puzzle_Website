use tide::Request;
use tide::prelude::*;

#[derive(Debug, Deserialize)]
struct Usersubmission {
    ans: String,
    time: String,

}
struct TestCase {
    input: String,
    output: String
}

struct Problem {
    statement: String,
    test_cases: String
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/validate").post(check_answer);
    app.at("/problem").get(get_problem);
    //app.at("/hello").get(|_| async { Ok("Hello, world!") });
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

fn get_expected_answer() -> String {
    return "hello".to_string();
}

async fn get_problem(mut req: Request<()>) -> tide:Result {
    Ok("HELLO THOS WHOLR\n".into());
} 

async fn check_answer(mut req: Request<()>) -> tide::Result {
    let data: Usersubmission = req.body_json().await?;
    //Ok(format!("Hello, {}! I've put in an order for {} shoes", name="a", legs="b").into())
    if data.ans == get_expected_answer() {
        return Ok("CORRECT".into())
    }
    else{
        return Ok("INCORRECT".into())
    }
}
