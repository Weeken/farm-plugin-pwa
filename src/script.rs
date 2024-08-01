pub fn generate_script(sw_js_path: &str, scope: &str) -> String {
  let script = "<script>
                        if ('serviceWorker' in navigator) {
                          window.addEventListener('load', function() {
                            navigator.serviceWorker
                              .register('[SW_JS]',{ scope: '[SCOPE]' })
                              .then(function() {
                                console.log('ServiceWorker registered.');
                              })
                              .catch(function(err) {
                                console.log('ServiceWorker registration failed: ', err);
                              });
                          });
                        }
                      </script>"
    .replace("[SW_JS]", sw_js_path)
    .replace("[SCOPE]", scope);

  return script;
}

pub fn inset_meta(html: &String, meta_or_link: String) -> String {
  let end_head = "</head>";
  return html.replace(end_head, format!("{}{}", meta_or_link, end_head).as_str());
}
