
// 1. commond line
// use std::str::FromStr;
// use std::env;

// fn main() {
//     let mut numbers = Vec::new();
//     for arg in env::args().skip(1){
//         numbers.push(u64::from_str(&arg)
//                     .expect("error parsing argument"));
//     }

//     if numbers.len()==0{
//         eprintln!("Usage: gcd NUMBER ...");
//         std::process::exit(1);
//     }

//     let mut d=numbers[0];
//     for m in &numbers[1..]{
//         d=gcd(d,*m);
//     }

//     println!("The greatest common divisor of {:?} is {}",numbers,d);
// }

// 2. web
use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main(){
    let server=HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("Severing on http://localhost:3000...");
    
    server
        .bind("127.0.0.1:3000").expect("error binding sever to address")
        .run()
        .await
        .expect("error running server");
}

async fn get_index()->HttpResponse{
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">Compute GCD</button>
                </form>
            "#,
        )
}

use serde::Deserialize;
#[derive(Deserialize)]
struct GcdParameters{
    n: u64,
    m: u64,
}

async fn post_gcd(form: web::Form<GcdParameters>)->HttpResponse{
    if form.n==0 || form.m==0{
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Compute the GCD with 0 is boring.");
    }

    let response=
        format!("The greatest common divisor of the numbers {} and {} \
                is <b>{}<b>\n",
                form.n, form.m, gcd(form.n, form.m));

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

fn gcd(mut n: u64, mut m: u64)->u64{
    assert!(n!=0 && m!=0);
    while m!=0{
        if m<n{
            let t=m;
            m=n;
            n=t;
        }
        m=m%n;
    }
    n
}


// #[test]
// fn test_gcd(){
//     assert_eq!(gcd(14, 15),1);
//     assert_eq!(gcd(2*4*6, 2),2);
// }