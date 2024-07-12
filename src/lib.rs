#![deny(clippy::all)]

use farmfe_core::{
  config::{Config, Mode},
  context::CompilationContext,
  error::Result,
  plugin::{Plugin, PluginFinalizeResourcesHookParams},
  resource::{Resource, ResourceOrigin, ResourceType},
  serde_json,
};

use std::sync::Arc;

use farmfe_macro_plugin::farm_plugin;

mod script;
use crate::script::generate_script;
mod sw;
use crate::sw::{generate_sw, insert_resource};

use minify_js::{minify, Session, TopLevelMode};

#[derive(serde::Deserialize, Clone)]
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
}

#[farm_plugin]
pub struct FarmPluginPwa {
  options: Options,
}

impl FarmPluginPwa {
  fn new(_: &Config, options: String) -> Self {
    let options: Options = serde_json::from_str(&options).unwrap();
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
      // println!("cache_name: {}", cache_name);
      let patten = options
        .patten
        .unwrap_or("/(.html|.js|.mjs|.css|.png|.jpg|.jpeg|.svg|.webp|.svga)$/".to_string());
      // println!("patten: {}", patten);

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
          // println!("origin_html: {}", origin_html);
          let new_html = format!("{}{}", origin_html, script);
          html_resource.bytes = new_html.as_bytes().to_vec();
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
