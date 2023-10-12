use worker::{console_log, event, Context, Date, Env, Request, Response, Result, Url};

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    log_request(&req);
    console_error_panic_hook::set_once();

    let path = req.path();
    if path == "/" {
        let url: Url = "https://github.com/sycertech/crates.pm".parse()?;

        return Response::redirect(url);
    }

    let mut url: Url = "https://crates.io/crates".parse()?;
    url.set_path(format!("crates/{}", path).as_str());

    Response::redirect(url)
}
