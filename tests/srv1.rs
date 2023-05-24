use helpers::*;

mod helpers;

#[cfg(feature = "srv1")]
mod srv1 {
  use super::*;

  #[tokio::test]
  async fn bravit() -> Any {
    let content = with("JRMOIE_wAFk", "en").await.fetch_srv1().await?;
    insta::assert_json_snapshot!(content);
    Ok(())
  }

  #[tokio::test]
  async fn song() -> Any {
    let transcript = with("PHzOOQfhPFg", "en").await.fetch_srv1().await?;
    insta::assert_json_snapshot!(transcript);
    Ok(())
  }
}
