#[cfg(feature = "srv3")]
#[tokio::test]
async fn srv3_is_deserialized() -> Any {
  let content = with("JRMOIE_wAFk", "en").await.fetch_srv3().await?;
  insta::assert_json_snapshot!(content);
  Ok(())
}
