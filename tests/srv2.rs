#[cfg(feature = "srv2")]
#[tokio::test]
async fn srv2_is_deserialized() -> Any {
  let content = with("JRMOIE_wAFk", "en").await.fetch_srv2().await?;
  insta::assert_json_snapshot!(content);
  Ok(())
}
