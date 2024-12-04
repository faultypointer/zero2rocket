use zero2rocket::run;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    run().ignite().await?.launch().await?;
    Ok(())
}
