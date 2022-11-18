use actix_web::{web, App, HttpServer};
mod uploads;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let url = "mysql://root:200324Aa_@localhost:3306/shinx_chat";
    let pool = mysql::Pool::new(url);
    let info = uploads::data::get_info().await;

    match pool{
        Ok(i) =>{
            println!("{}", format!("Port {} have runing as server.", info.port));
            HttpServer::new(move ||{
                App::new()
                    .app_data(awmp::PartsConfig::default().with_file_limit(1073741824))
                    .app_data(web::Data::new(i.clone()))
                    .service(uploads::upload::upload)
            })
            .bind(format!("{}:{}",info.addr, info.port))?
            .run()
            .await?;
        },
        Err(e)=>{
            println!("From server's  message: {:?}",e);
        }
    };

    Ok(())
}