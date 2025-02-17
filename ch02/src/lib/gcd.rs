use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GcdParameters {
    pub n: u64,
    pub m: u64,
}

#[get("/")]
pub async fn init_gcd() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
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

#[post("/gcd")]
pub async fn post_gcd(form: web::Form<GcdParameters>) -> impl Responder {
    if form.n == 0 && form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is undefined");
    }

    let d = calculate(form.n, form.m);
    let response = format!("The GCD of {} and {} is {}", form.n, form.m, d);

    HttpResponse::Ok().content_type("text/html").body(response)
}

fn calculate(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}
