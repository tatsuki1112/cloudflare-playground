use serde::{Deserialize, Serialize};
use worker::*;

#[derive(Deserialize, Serialize)]
struct User {
    id: u64,
    name: String,
}
#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    Router::new()
        .get_async("/ready", |_, ctx| async move { Response::empty() })
        .get_async("/me", |_, ctx| async {
            Response::ok("tatsuki kanda!!!!".to_string())
        })
        .get_async("/user/:id", |_, ctx| async move {
            let id = ctx.param("id").unwrap();
            let d1 = ctx.env.d1("DB")?;
            let stmt = d1.prepare("select * from user where id = ?1");
            let q = stmt.bind(&[id.into()])?;
            let res = q.first::<User>(None).await?;
            match res {
                Some(user) => Response::from_json(&user),
                None => Response::error("Not found!!!!!", 404),
            }
        })
        .run(req, env)
        .await
}
