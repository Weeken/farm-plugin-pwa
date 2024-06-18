use farmfe_core::resource::Resource;
use std::collections::HashMap;

pub fn generate_sw(cache_name: &str, static_files: &str, patten: &str) -> String {
  let sw_text = "
  const filesCacheName = '[FILES_CACHE_NAME]';
  const staticFiles = [STATIC_FILES];
  const patten = [PATTEN];

  self.addEventListener('install', function (e) {
    self.skipWaiting();
    e.waitUntil(
      caches.open(filesCacheName).then((cache) => {
        return cache.addAll(staticFiles);
      })
    );
  });
  self.addEventListener('activate', function (event) {
    event.waitUntil(
      self.clients.claim().then(() => {
        self.clients.matchAll().then((clients) => {
          clients.forEach((client) => client.navigate(client.url));
        });
        return caches.keys().then(function (cacheNames) {
          return Promise.all(
            cacheNames
              .filter(function (cacheName) {
                return cacheName != filesCacheName;
              })
              .map(function (cacheName) {
                return caches.delete(cacheName);
              })
          );
        });
      })
    );
  });
  self.addEventListener('fetch', function (e) {
    e.respondWith(proxyRequest(e.request));
  });
  const isCacheSource = (url) => {
    return patten.test(url) && url.startsWith('https');
  };
  const proxyRequest = async (request) => {
    if (isCacheSource(request.url)) {
      return caches.match(request).then(function (response) {
        if (response) {
          return response;
        } else {
          return fetch(request).then((response) => {
            return caches.open(filesCacheName).then((cache) => {
              cache.put(request, response.clone());
              return response;
            });
          });
        }
      });
    } else {
      return fetch(request);
    }
  };"
    .replace("[FILES_CACHE_NAME]", cache_name)
    .replace("[STATIC_FILES]", static_files)
    .replace("[PATTEN]", patten);

  return sw_text;
}

pub fn insert_resource(
  resources_map: &mut HashMap<String, Resource>,
  name: String,
  resource: Resource,
) {
  if !resources_map.contains_key(&name) {
    resources_map.insert(name, resource);
  }
}
