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
