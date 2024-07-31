#![deny(clippy::all)]

use farmfe_core::{
  config::{Config, Mode},
  context::CompilationContext,
  error::Result,
  plugin::{Plugin, PluginFinalizeResourcesHookParams},
  resource::{Resource, ResourceOrigin, ResourceType},
  serde_json::{from_str, to_string, Map, Value},
};

use std::sync::Arc;

use farmfe_macro_plugin::farm_plugin;

mod script;
use crate::script::{generate_script, inset_meta};
mod sw;
use crate::sw::{generate_sw, insert_resource};

use minify_js::{minify, Session, TopLevelMode};

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Options {
  pub scope: Option<String>,
  /**
   * SW 名
   */
  pub sw_name: Option<String>,
  /**
   * 缓存名
   */
  pub cache_name: Option<String>,
  /**
   * 初始缓存文件
   */
  pub static_files: Option<Vec<String>>,
  /**
   * 缓存请求正则
   */
  pub patten: Option<String>,
  /**
   * pwa manifest
   */
  pub manifest: Option<Map<String, Value>>,
}

#[farm_plugin]
pub struct FarmPluginPwa {
  options: Options,
}

impl FarmPluginPwa {
  fn new(_config: &Config, _options: String) -> Self {
    let options: Options = from_str(&_options).unwrap();
    Self { options }
  }
}

impl Plugin for FarmPluginPwa {
  fn name(&self) -> &str {
    "FarmPluginPwa"
  }

  fn priority(&self) -> i32 {
    99
  }

  fn finalize_resources(
    &self,
    _param: &mut PluginFinalizeResourcesHookParams,
    _context: &Arc<CompilationContext>,
  ) -> Result<Option<()>> {
    if matches!(_context.config.mode, Mode::Production) {
      let public_path = &_param.config.output.public_path;
      let options = self.options.clone();
      let mut static_files = options.static_files.unwrap_or(vec![]);
      // println!("static_files: {:?}", static_files);
      let scope = options.scope.unwrap_or(public_path.to_string());
      // println!("scope: {}", scope);
      let sw_name = options.sw_name.unwrap_or("sw".to_string());
      // println!("sw_name: {}", sw_name);
      let cache_name = options.cache_name.unwrap_or("sw-cache".to_string());

      let patten = options
        .patten
        .unwrap_or("/(.html|.js|.mjs|.css|.png|.jpg|.jpeg|.svg|.webp|.svga)$/".to_string());

      let manifest = options.manifest.unwrap_or_default();
      let is_manifest_empty = manifest.is_empty();

      let sw_js_path = format!("{}{}.js", public_path, sw_name);

      let script = generate_script(&sw_js_path, &scope);
      // println!("script: {}", script);

      let resource_map_clone = _param.resources_map.clone();
      for (name, resource) in resource_map_clone.iter() {
        if !name.starts_with("FARM_RUNTIME_") {
          // println!("name: {}", name);
          static_files.push(format!("{}{}", public_path, name.to_string()));
        }
        if name == "index.html" {
          let mut html_resource = resource.clone();
          let origin_html = String::from_utf8(html_resource.bytes).unwrap();

          let mut new_html: String = origin_html;
          if !is_manifest_empty {
            let manifest_link = format!(
              "<link rel=\"manifest\" href=\"{}manifest.json\" />",
              public_path
            );
            new_html = inset_meta(&new_html, manifest_link);

            let manifest_json = to_string(&manifest).unwrap();

            let manifest_json_resource = Resource {
              name: "manifest.json".to_string(),
              bytes: manifest_json.as_bytes().to_vec(),
              emitted: false,
              resource_type: ResourceType::Custom("json".to_string()),
              origin: ResourceOrigin::Module("unknown".into()),
              info: None,
            };

            insert_resource(
              _param.resources_map,
              "manifest.json".to_string(),
              manifest_json_resource,
            );
          }
          let final_html = format!("{}{}", new_html, script);
          html_resource.bytes = final_html.as_bytes().to_vec();
          _param.resources_map.insert(name.to_string(), html_resource);
        }
      }
      let static_files_str = json::stringify(static_files);
      // println!("static_files: {:?}", &static_files_str);

      let sw_text = generate_sw(&cache_name, &static_files_str, &patten);
      let session = Session::new();
      let mut out = Vec::new();
      minify(&session, TopLevelMode::Global, sw_text.as_bytes(), &mut out).unwrap();
      // println!("minify_code: {:?}", out.as_slice().to_vec());

      let file_name = format!("{}.js", sw_name);

      let sw_resource = Resource {
        name: file_name.clone(),
        bytes: out.as_slice().to_vec(),
        emitted: false,
        resource_type: ResourceType::Js,
        origin: ResourceOrigin::Module("unknown".into()),
        info: None,
      };

      insert_resource(_param.resources_map, file_name.clone(), sw_resource);
    }
    Ok(None)
  }
}
