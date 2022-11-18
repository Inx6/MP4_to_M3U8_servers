use actix_web::{web, post, HttpResponse};
use crate::uploads::data;
use rand::Rng;

#[post("/uploads")]
pub async fn upload(mut parts: awmp::Parts, _db:web::Data<mysql::Pool>) -> Result<HttpResponse, actix_web::Error> {
    let info = data::get_info().await;
    // 获取文件类型
    let i = format!("{}",&parts.texts.as_hash_map().get("type").unwrap());
    // 自定义缓存路径
    let path = format!("{}", info.src);

    // 判断文件类型
    if i == "video"{
        let file_parts = parts
            .files
            .take("file")
            .pop()
            .and_then(|f| f.persist_in(&path).ok())
            .map(|f| format!("File uploaded to: {}", f.display()))
            .unwrap_or_default();
        
        // 新建文件名
        let ns = format!("HD{}",rand::thread_rng().gen::<u32>());
        // 新建文件保存目录
        let file_save = format!("{}/{}.m3u8", &path, ns);
        // 文件进入新线程，不再等待
        let result = tokio::task::spawn_blocking(move ||data::complete((file_parts.replace("File uploaded to: ",""), file_save)));
        // 直接返回结果
        Ok(actix_web::HttpResponse::Ok().body(format!("{:?}",result)))
    }else{
        Ok( actix_web::HttpResponse::Ok().body("文件不相符!"))
    }
}
