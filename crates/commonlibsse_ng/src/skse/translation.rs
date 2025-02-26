// use crate::sys::{root, RE};
// use std::collections::HashMap;
// use windows::core::HSTRING;

// #[cfg(feature = "tracing")]
// use tracing::{error, info, warn};

// pub fn parse_translation(name: &str) {
//     let scaleform_manager = unsafe { RE::BSScaleformManager::GetSingleton() };
//     if scaleform_manager.is_null() {
//         error!("Scaleform manager is not available.");
//         return;
//     }
//     let loader = unsafe { scaleform_manager.as_ref() }.and_then(|m| unsafe { m.loader.as_ref() });
//     let translator = loader.and_then(|l| l.GetStateAddRef(RE::GFxState_StateType::kTranslator));

//     let scaleform_translator =
//         translator.and_then(|t| t.downcast_ref::<RE::BSScaleformTranslator>());

//     if scaleform_translator.is_none() {
//         #[cfg(feature = "tracing")]
//         warn!("Failed to import translation for {name}");
//         return;
//     }

//     let ini_setting_collection = unsafe { RE::INISettingCollection::GetSingleton() };

//     let sv = {
//         let s = "sLanguage:General";
//         let len = s.len() as u64;
//         let s_ptr = s.as_ptr() as u64;
//         let sv: root::std::string_view = [0; 2];
//         sv[0] = s_ptr;
//         sv[1] = len;
//         sv
//     };
//     let setting = unsafe { ini_setting_collection.as_ref().map(|c| c.GetSetting(sv)) };

//     let language = setting
//         .filter(|s| unsafe {
//             s.as_ref()
//                 .map(|s| unsafe { s.GetType() } == RE::Setting_Type::kString)
//                 .unwrap_or_default()
//         })
//         .map(|s| unsafe { s.as_ref() }.and_then(|s| s.c_str().to_string()))
//         .unwrap_or_else(|| "ENGLISH".to_string());

//     let path = format!("Interface\\Translations\\{}_{}.txt", name, language);
//     root::std::string;
//     let mut file_stream = RE::BSResourceNiBinaryStream::new2(&path);

//     if !file_stream.good() {
//         return;
//     }

//     info!("Reading translations from {}...", path);

//     let mut bom: u16 = 0;
//     if file_stream.read_exact(&mut bom).is_err() || bom != 0xFEFF {
//         error!("BOM Error, file must be encoded in UCS-2 LE.");
//         return;
//     }

//     let mut translation_map = HashMap::new();
//     while let Some(line) = file_stream.read_line_w('\n') {
//         if line.len() < 4 || !line.starts_with('$') {
//             continue;
//         }

//         let trimmed = line.trim_end_matches('\r');
//         if let Some(delim_idx) = trimmed.find('\t') {
//             let (key, value) = trimmed.split_at(delim_idx);
//             let key = key.trim();
//             let value = value[1..].trim();

//             if let (Some(cached_key), Some(cached_translation)) = (
//                 RE::BSScaleformTranslator::GetCachedString(key),
//                 RE::BSScaleformTranslator::GetCachedString(value),
//             ) {
//                 translation_map.insert(cached_key, cached_translation);
//             }
//         }
//     }

//     if let Some(translator) = scaleform_translator {
//         translator.translation_map.extend(translation_map);
//     }
// }

// pub fn translate(key: &str) -> Option<String> {
//     if !key.starts_with('$') {
//         return None;
//     }

//     let scaleform_manager = unsafe { RE::BSScaleformManager::GetSingleton() };
//     let loader = unsafe { scaleform_manager.as_ref().and_then(|m| m.loader.as_ref()) };
//     let translator = loader.and_then(|l| l.getStateAddRef(RE::GFxState_StateType::kTranslator));

//     let translator = translator?;
//     let mut result = unsafe { RE::GFxWStringBuffer::new() };

//     let key_utf16 = HSTRING::from(key);
//     let translate_info = RE::GFxTranslator_TranslateInfo {
//         key: key_utf16.as_ptr(),
//         result: &mut result,
//         instanceName: ::core::ptr::null_mut(),
//         flags: 0,
//         pad19: 0,
//         pad1A: 0,
//         pad1C: 0,
//     };

//     translator.translate(&translate_info);
//     if unsafe { result.empty() } {
//         return None;
//     }

//     Some(HSTRING::from(unsafe { result.c_str() }).to_string())
// }
