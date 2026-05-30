use std::fs;

use crate::prelude::*;
use godot::classes::{
    EditorImportPlugin, EditorPlugin, IEditorImportPlugin, IEditorPlugin, Resource, ResourceSaver,
};
use godot::global::Error;

#[derive(GodotClass)]
#[class(base=EditorPlugin, tool, init)]
pub struct JSImportPlugin {
    base: Base<EditorPlugin>,
}

#[godot_api]
impl IEditorPlugin for JSImportPlugin {
    fn enter_tree(&mut self) {
        let importer = JsImporter::new_gd();
        self.base_mut().add_import_plugin(&importer);
        godot_print!("JS Import Plugin registered");
    }
}

#[derive(GodotClass)]
#[class(base=EditorImportPlugin, tool)]
pub struct JsImporter {
    base: Base<EditorImportPlugin>,
}

#[godot_api]
impl IEditorImportPlugin for JsImporter {
    fn init(base: Base<EditorImportPlugin>) -> Self {
        Self { base }
    }

    fn get_importer_name(&self) -> GString {
        "my_project.js_file_importer".into()
    }

    fn get_visible_name(&self) -> GString {
        "JavaScript Text File".into()
    }

    fn get_recognized_extensions(&self) -> PackedStringArray {
        let mut array = PackedStringArray::new();
        array.push("js");
        array.push("ts");
        array
    }

    fn get_save_extension(&self) -> GString {
        "res".into()
    }

    fn get_resource_type(&self) -> GString {
        "Resource".into()
    }

    fn get_preset_count(&self) -> i32 {
        1
    }

    fn get_preset_name(&self, _preset_index: i32) -> GString {
        "Default".into()
    }

    fn get_import_options(&self, _path: GString, _preset_index: i32) -> Array<AnyDictionary> {
        Array::new()
    }

    fn can_import_threaded(&self) -> bool {
        false
    }

    fn get_option_visibility(
        &self,
        _path: GString,
        _option_name: StringName,
        _options: VarDictionary,
    ) -> bool {
        false
    }

    fn import(
        &mut self,
        source_file: GString,
        save_path: GString,
        _options: VarDictionary,
        _platform_variants: Array<GString>,
        _gen_files: Array<GString>,
    ) -> Error {
        let global_src_path =
            godot::classes::ProjectSettings::singleton().globalize_path(&source_file.to_string());

        if let Err(err) = fs::read_to_string(global_src_path.to_string()) {
            godot_error!("Failed to read JS file: {}", err);
            return Error::ERR_FILE_CANT_READ;
        }

        let resource = Resource::new_gd();
        let full_save_path = format!("{}.{}", save_path, self.get_save_extension());

        ResourceSaver::singleton()
            .save_ex(&resource)
            .path(&full_save_path)
            .done()
    }
}
