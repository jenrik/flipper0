#![allow(clippy::large_enum_variant)]
/*! Flipper Application Manifest struct
	 and (de)serialization methods and for export to fam `App` struct.

	 [Official documentation](https://github.com/flipperdevices/flipperzero-firmware/blob/release-candidate/documentation/AppManifests.md)
*/

extern crate handlebars;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use handlebars::Handlebars;
use handlebars::RenderError;


/// Strict template
const TEMPLATE: &str = r#"
App(
	# Automatically generated by `fam` crate.
	appid="{{{appid}}}",
	apptype={{{apptype}}},
	{{#if name}}name="{{{name}}}", {{/if}}
	{{~#if stack_size}}stack_size={{{stack_size}}}, {{/if}}
	{{~#if icon}}icon="{{{icon}}}", {{/if}}
	{{~#if entry_point}}entry_point="{{{entry_point}}}", {{/if}}
	{{~#if order}}order={{{order}}}, {{/if}}
	{{~#if sources.0 }}sources=[{{#each sources}}"{{{this}}}"{{#if @last}}{{else}}, {{/if}}{{/each}}], {{/if}}
	{{~#if flags.0 }}flags=[{{#each flags}}"{{{this}}}"{{#if @last}}{{else}}, {{/if}}{{/each}}], {{/if}}
	{{~#if cdefines.0 }}cdefines=[{{#each cdefines}}"{{{this}}}"{{#if @last}}{{else}}, {{/if}}{{/each}}], {{/if}}
	{{~#if requires.0 }}requires=[{{#each requires}}"{{{this}}}"{{#if @last}}{{else}}, {{/if}}{{/each}}], {{/if}}
	{{~#if conflicts.0 }}conflicts=[{{#each conflicts}}"{{{this}}}"{{#if @last}}{{else}}, {{/if}}{{/each}}], {{/if}}
	{{~#if provides.0 }}provides=[{{#each provides}}"{{{this}}}"{{#if @last}}{{else}}, {{/if}}{{/each}}], {{/if}}
	{{~#if sdk_headers.0 }}sdk_headers=[{{#each sdk_headers}}"{{{this}}}"{{#if @last}}{{else}}, {{/if}}{{/each}}], {{/if}}
	{{~#if fap_libs.0 }}fap_libs=[{{#each fap_libs}}"{{{this}}}"{{#if @last}}{{else}}, {{/if}}{{/each}}], {{/if}}
	{{~#if fap_version}}fap_version={{{fap_version}}}, {{/if}}
	{{~#if fap_icon}}fap_icon="{{{fap_icon}}}", {{/if}}
	{{~#if fap_category}}fap_category="{{{fap_category}}}", {{/if}}
	{{~#if fap_description}}fap_description="{{{fap_description}}}", {{/if}}
	{{~#if fap_author}}fap_author="{{{fap_author}}}", {{/if}}
	{{~#if fap_weburl}}fap_weburl="{{{fap_weburl}}}", {{/if}}
)
"#;

// TODO: Non-strict template for render as-is to **kwargs.


#[derive(Serialize, Deserialize, Clone)]
pub enum Manifest {
	/// For custom fields.
	Json(Value),

	#[cfg(feature = "toml")]
	/// For custom fields.
	Toml(toml::Value),

	Struct {
		/// Name that is displayed in menus
		name: Option<String>,

		/// String, application id within the build system. Used for specifying which applications to include in build configuration and to resolve dependencies and conflicts.
		appid: String,

		/// Member of FlipperAppType.* enumeration. Valid values are:
		/// - `SERVICE`: System service, created at early startup
		/// - `SYSTEM`: Application not being shown in any menus. Can be started by other apps or from CLI
		/// - `APP`: Regular application for main menu
		/// - `PLUGIN`: Application to be built as a part of firmware an to be placed in Plugins menu
		/// - `DEBUG`: Application only visible in Debug menu with debug mode enabled
		/// - `ARCHIVE`: One and only Archive app
		/// - `SETTINGS`: Application to be placed in System settings menu
		/// - `STARTUP`: Callback function to run at system startup. Does not define a separate app
		/// - `EXTERNAL`: Application to be built as .fap plugin
		/// - `METAPACKAGE`: Does not define any code to be run, used for declaring dependencies and application bundles
		apptype: String,

		/// Stack size, in bytes, to allocate for application on its startup. Note that allocating a stack that is too small for an app to run will cause system crash due to stack overflow, and allocating too much stack space will reduce usable heap memory size for apps to process data. Note: you can use ps and free CLI commands to profile your app's memory usage.
		stack_size: Option<usize>,

		/// Animated icon name from built-in assets to be used when building app as a part of firmware.
		icon: Option<String>,

		/// C function to be used as application's entry point
		entry_point: Option<String>,

		/// Internal flags for system apps. Do not use.
		#[serde(default)]
		flags: Vec<String>,

		/// C preprocessor definitions to declare globally for other apps when current application is included in active build configuration.
		#[serde(default)]
		cdefines: Vec<String>,

		/// List of application IDs to also include in build configuration, when current application is referenced in list of applications to build.
		#[serde(default)]
		requires: Vec<String>,

		/// List of application IDs that current application conflicts with. If any of them is found in constructed application list, fbt will abort firmware build process.
		#[serde(default)]
		conflicts: Vec<String>,

		/// Functionally identical to requires field.
		#[serde(default)]
		provides: Vec<String>,

		/// Order of an application within its group when sorting entries in it. The lower the order is, the closer to the start of the list the item is placed. Used for ordering startup hooks and menu entries.
		order: Option<isize>,

		/// List of C header files from this app's code to include in API definitions for external applications.
		#[serde(default)]
		sdk_headers: Vec<String>,

		// External FAPs only:
		/// list of strings, file name masks, used for gathering sources within app folder. Default value of ["*.c*"] includes C and CPP source files.
		#[serde(default)]
		sources: Vec<String>,

		/// Tuple, 2 numbers in form of (x,y): application version to be embedded within .fap file. Default value is (0,1), meaning version "0.1".
		fap_version: Option<(String, String)>,

		/// Name of a .png file, 1-bit color depth, 10x10px, to be embedded within .fap file.
		fap_icon: Option<String>,

		/// List of extra libraries to link application against. Provides access to extra functions that are not exported as a part of main firmware at expense of increased .fap file size and RAM consumption.
		#[serde(default)]
		fap_libs: Vec<String>,

		/// String, may be empty. App subcategory, also works as path of FAP within apps folder in the file system.
		fap_category: Option<String>,

		/// String, may be empty. Short application description.
		fap_description: Option<String>,

		/// String, may be empty. Application's author.
		fap_author: Option<String>,

		/// String, may be empty. Application's homepage.
		fap_weburl: Option<String>,
	},
}

impl Manifest {
	pub fn try_to_string(&self) -> Result<String, RenderError> {
		match self {
			#[cfg(feature = "toml")]
			Manifest::Toml(toml) => render_raw_toml(toml),
			Manifest::Json(json) => render_raw_json(json),
			Manifest::Struct { name,
			                   appid,
			                   apptype,
			                   stack_size,
			                   icon,
			                   entry_point,
			                   flags,
			                   cdefines,
			                   requires,
			                   conflicts,
			                   provides,
			                   order,
			                   sdk_headers,
			                   sources,
			                   fap_version,
			                   fap_icon,
			                   fap_libs,
			                   fap_category,
			                   fap_description,
			                   fap_author,
			                   fap_weburl, } => {
				let json = serde_json::json!({
					"name": name,
					"appid": appid,
					"apptype": apptype,
					"stack_size": stack_size,
					"icon": icon,
					"entry_point": entry_point,
					"flags": flags,
					"cdefines": cdefines,
					"requires": requires,
					"conflicts": conflicts,
					"provides": provides,
					"order": order,
					"sdk_headers": sdk_headers,
					"sources": sources,
					"fap_version": fap_version,
					"fap_icon": fap_icon,
					"fap_libs": fap_libs,
					"fap_category": fap_category,
					"fap_description": fap_description,
					"fap_author": fap_author,
					"fap_weburl": fap_weburl,
				});
				render_raw_json(&json)
			},
		}
	}
}


macro_rules! field {
	($key:ident, str) => {
		pub fn $key(&self) -> Option<&str> {
			match self {
				Self::Struct { $key, .. } => Some($key.as_str()),
				Self::Json(value) => {
					value.as_object()
					     .map(|o| o.get(stringify!($key)).map(|v| v.as_str()).flatten())
					     .flatten()
				},
				#[cfg(feature = "toml")]
				Self::Toml(value) => {
					value.as_table()
					     .map(|o| o.get(stringify!($key)).map(|v| v.as_str()).flatten())
					     .flatten()
				},
			}
		}
	};

	($key:ident, opt str) => {
		pub fn $key(&self) -> Option<&str> {
			match self {
				Self::Struct { $key, .. } => Some($key.as_deref()).flatten(),
				Self::Json(value) => {
					value.as_object()
					     .map(|o| o.get(stringify!($key)).map(|v| v.as_str()).flatten())
					     .flatten()
				},
				#[cfg(feature = "toml")]
				Self::Toml(value) => {
					value.as_table()
					     .map(|o| o.get(stringify!($key)).map(|v| v.as_str()).flatten())
					     .flatten()
				},
			}
		}
	};

	($key:ident, iter String) => {
		pub fn $key(&self) -> Box<dyn Iterator<Item = String> + '_> {
			match self {
				Self::Struct { $key, .. } => Box::new($key.into_iter().cloned()),
				Self::Json(value) => {
					Box::new(value.as_object()
					         .map(|o| {
						         o.get(stringify!($key))
						          .map(|v| serde_json::from_value::<Vec<String>>(v.to_owned()).ok())
						          .flatten()
					         })
					         .flatten()
					         .map(|arr| arr.into_iter())
					         .unwrap_or(Vec::with_capacity(0).into_iter().into()))
				},
				#[cfg(feature = "toml")]
				Self::Toml(value) => todo!(), // TODO: toml::Value as vec to iter
			}
		}
	};
}


impl Manifest {
	field!(appid, str);
	field!(apptype, str);
	field!(name, opt str);
	field!(icon, opt str);
	field!(entry_point, opt str);
	field!(fap_icon, opt str);
	field!(fap_category, opt str);
	field!(fap_description, opt str);
	field!(fap_author, opt str);
	field!(fap_weburl, opt str);
	field!(flags, iter String);
	field!(cdefines, iter String);
	field!(requires, iter String);
	field!(conflicts, iter String);
	field!(provides, iter String);
	field!(sdk_headers, iter String);
	field!(sources, iter String);
	field!(fap_libs, iter String);

	pub fn fap_version(&self) -> Option<(String, String)> {
		match self {
			Self::Struct { fap_version, .. } => fap_version.to_owned(),
			Self::Json(value) => {
				value.as_object().and_then(|o| {
					                 o.get("fap_version")
					                  .and_then(|v| serde_json::from_value::<(String, String)>(v.to_owned()).ok())
				                 })
			},
			#[cfg(feature = "toml")]
			Self::Toml(value) => {
				value.as_table()
				     .map(|o| {
					     o.get("fap_version")
					      .map(|v| {
						      v.as_array()
						       .map(|arr| {
							       arr.get(0)
							          .map(|a| {
								          arr.get(1)
								             .map(|b| (a.as_str().map(ToOwned::to_owned), b.as_str().map(ToOwned::to_owned)))
								             .filter(|(a, b)| a.is_some() && b.is_some())
								             .map(|(a, b)| (a.unwrap(), b.unwrap()))
							          })
							          .flatten()
						       })
						       .flatten()
					      })
					      .flatten()
				     })
				     .flatten()
			},
		}
	}
}


impl From<Value> for Manifest {
	fn from(metadata: Value) -> Self { Self::Json(metadata) }
}


pub fn render_raw_json(json: &Value) -> Result<String, RenderError> {
	let reg = Handlebars::new();
	reg.render_template(TEMPLATE, json)
}


#[cfg(feature = "toml")]
impl From<toml::Value> for Manifest {
	fn from(metadata: toml::Value) -> Self { Self::Toml(metadata) }
}

#[cfg(feature = "toml")]
pub fn render_raw_toml(toml: &toml::Value) -> Result<String, RenderError> {
	let reg = Handlebars::new();
	reg.render_template(TEMPLATE, toml)
}
