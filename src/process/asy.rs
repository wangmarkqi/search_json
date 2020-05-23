use futures::executor::block_on;
use crate::common::strerr::StrError;
pub async fn f1() -> std::result::Result<String,StrError> {
    Ok("asdfa".to_string())
}

pub async fn f2() -> std::result::Result<String,StrError> {
    let res = f1().await?;
    dbg!(&res);
    Ok(res)
}

pub fn test() {
    let res=block_on(f2());

}