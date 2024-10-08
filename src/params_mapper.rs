use std::collections::HashMap;
use serde_json::Value;

#[derive(Debug)]
struct ComfyTaskParams {
    params: HashMap<String, String>,
    fooo2node: HashMap<String, String>,
}

impl ComfyTaskParams {
    fn new(params: HashMap<String, String>) -> Self {
        let fooo2node = HashMap::from([
            ("seed", "KSampler:main_sampler:seed;TiledKSampler:main_sampler:seed;KolorsSampler:main_sampler:seed;RandomNoise:noise_seed:noise_seed;easy seed:sync_seed:seed"),
            ("steps", "KSampler:main_sampler:steps;TiledKSampler:main_sampler:steps;KolorsSampler:main_sampler:steps;BasicScheduler:scheduler_select:steps"),
            ("cfg_scale", "KSampler:main_sampler:cfg;TiledKSampler:main_sampler:cfg;KolorsSampler:main_sampler:cfg;CLIPTextEncodeFlux:prompt:guidance"),
            ("sampler", "KSampler:main_sampler:sampler_name;TiledKSampler:main_sampler:sampler_name;KSamplerSelect:sampler_select:sampler_name"),
            ("scheduler", "KSampler:main_sampler:scheduler;TiledKSampler:main_sampler:scheduler;KolorsSampler:main_sampler:scheduler;BasicScheduler:scheduler_select:scheduler"),
            ("denoise", "KSampler:main_sampler:denoise;TiledKSampler:main_sampler:denoise;KolorsSampler:main_sampler:denoise_strength;BasicScheduler:scheduler_select:denoise"),
            ("tiling", "TiledKSampler:main_sampler:tiling;SeamlessTile:seamless_tile:tiling;CircularVAEDecode:vae_tiled:tiling"),
            ("tiled_offset_x", "OffsetImage:offset_image:x_percent"),
            ("tiled_offset_y", "OffsetImage:offset_image:y_percent"),
            ("base_model", "CheckpointLoaderSimple:base_model:ckpt_name;UNETLoader:base_model:unet_name;CheckpointLoaderNF4:base_model:ckpt_name;UnetLoaderGGUF:base_model:unet_name"),
            ("base_model_dtype", "UNETLoader:base_model:weight_dtype"),
            ("merge_model", "UNETLoader:merge_model:unet_name"),
            ("model_merge_ratio", "ModelMergeSimple:model_merge_ratio:ratio"),
            ("lora_speedup", "LoraLoaderModelOnly:lora_speedup:lora_name"),
            ("lora_speedup_strength", "LoraLoaderModelOnly:lora_speedup:strength_model"),
            ("lora_1", "LoraLoaderModelOnly:lora_1:lora_name;LoraLoaderModelOnly:lora_speedup:lora_name"),
            ("lora_1_strength", "LoraLoaderModelOnly:lora_1:strength_model;LoraLoaderModelOnly:lora_speedup:strength_model"),
            ("lora_2", "LoraLoaderModelOnly:lora_2:lora_name"),
            ("lora_2_strength", "LoraLoaderModelOnly:lora_2:strength_model"),
            ("lora_3", "LoraLoaderModelOnly:lora_3:lora_name"),
            ("lora_3_strength", "LoraLoaderModelOnly:lora_3:strength_model"),
            ("lora_4", "LoraLoaderModelOnly:lora_4:lora_name"),
            ("lora_4_strength", "LoraLoaderModelOnly:lora_4:strength_model"),
            ("lora_5", "LoraLoaderModelOnly:lora_5:lora_name"),
            ("lora_5_strength", "LoraLoaderModelOnly:lora_5:strength_model"),
            ("width", "EmptyLatentImage:aspect_ratios_size:width;EmptySD3LatentImage:aspect_ratios_size:width;ImageResize+:resize_input_image:width;KolorsSampler:main_sampler:width;easy int:aspect_ratios_width:value"),
            ("height", "EmptyLatentImage:aspect_ratios_size:height;EmptySD3LatentImage:aspect_ratios_size:height;ImageResize+:resize_input_image:height;KolorsSampler:main_sampler:height;easy int:aspect_ratios_height:value"),
            ("prompt", "CLIPTextEncode:prompt:text;MZ_ChatGLM3_V2:prompt:text;KolorsTextEncode:prompt_negative_prompt:prompt;CLIPTextEncodeFlux:prompt:t5xxl;CLIPTextEncodeFlux:prompt:clip_l"),
            ("negative_prompt", "CLIPTextEncode:negative_prompt:text;MZ_ChatGLM3_V2:negative_prompt:text;KolorsTextEncode:prompt_negative_prompt:negative_prompt"),
            ("clip_model", "DualCLIPLoader:clip_model:clip_name1;DualCLIPLoaderGGUF:clip_model:clip_name1;CLIPLoaderGGUF:clip_model:clip_name;CLIPLoader:clip_model:clip_name"),
            ("llms_model", "MZ_ChatGLM3Loader:llms_model:chatglm3_checkpoint;DownloadAndLoadChatGLM3:llms_model:precision"),
            ("input_image", "LoadImage:input_image:image"),
            ("layer_diffuse_injection", "LayeredDiffusionApply:layer_diffuse_apply:config"),
            ("sd_version", "LayeredDiffusionDecode:layer_diffuse_decode:sd_version;LayeredDiffusionDecodeRGBA:layer_diffuse_decode_rgba:sd_version"),
            ("layer_diffuse_cond", "LayeredDiffusionCondApply:layer_diffuse_cond_apply:config"),
            ("light_source_text_switch", "easy imageSwitch:ic_light_source_text_switch:boolean"),
            ("light_source_shape_switch", "easy imageSwitch:ic_light_source_shape_switch:boolean"),
            ("light_source_text", "LightSource:ic_light_source_text:light_position"),
            ("light_apply", "LoadAndApplyICLightUnet:ic_light_apply:model_path"),
            ("light_detail_transfer", "DetailTransfer:ic_light_detail_transfer:mode"),
            ("light_source_start_color", "CreateGradientFromCoords:ic_light_source_color:start_color"),
            ("light_source_end_color", "CreateGradientFromCoords:ic_light_source_color:end_color"),
            ("light_editor_path", "SplineEditor:ic_light_editor:points_store"),
        ]);

        Self {
            params,
            fooo2node,
        }
    }

    fn set_mapping_rule(&mut self, maps: HashMap<String, String>) {
        self.fooo2node.extend(maps);
    }

    fn update_params(&mut self, new_params: HashMap<String, String>) {
        self.params.extend(new_params);
    }

    fn delete_params(&mut self, keys: Vec<String>) {
        for k in keys {
            self.params.remove(&k);
        }
    }

    fn convert2comfy(&self, workflow: &str) -> Result<String, serde_json::Error> {
        // Parse the JSON string into a serde_json::Value
        let mut workflow_json: Value = serde_json::from_str(workflow)?;

        for (pk1, v) in &self.params {
            if let Some(nk) = self.fooo2node.get(pk1) {
                self.replace_key(&mut workflow_json, nk, v);
            }
        }

        // Convert the modified serde_json::Value back to a JSON string
        let new_workflow = serde_json::to_string(&workflow_json)?;

        Ok(new_workflow)
    }

    fn replace_key(&self, workflow: &mut Value, nk: &str, v: &str) {
        let lines: Vec<&str> = nk.split(';').collect();
        for line in lines {
            let parts: Vec<&str> = line.trim().split(':').collect();
            let class_type = parts[0].trim().to_string();
            let meta_title = parts[1].trim().to_string();
            let inputs = parts[2].trim().to_string();

            if let Value::Array(nodes) = workflow {
                for node in nodes.iter_mut() {
                    if node["class_type"] == class_type && node["_meta"]["title"] == meta_title {
                        if inputs.contains('|') {
                            let keys: Vec<&str> = inputs.split('|').collect();
                            let vs: Vec<&str> = v.trim().split('|').collect();
                            for i in 0..keys.len() {
                                node["inputs"][keys[i]] = Value::String(vs[i].to_string());
                            }
                        } else {
                            node["inputs"][inputs] = Value::String(v.to_string());
                        }
                    }
                }
            }
        }
    }
}

fn test() {
    let mut params = HashMap::new();
    params.insert("seed".to_string(), "12345".to_string());
    params.insert("steps".to_string(), "50".to_string());

    let workflow = r#"[
        {
            "class_type": "KSampler",
            "_meta": {
                "title": "main_sampler"
            },
            "inputs": {}
        }
    ]"#;

    let comfy_task_params = ComfyTaskParams::new(params);
    match comfy_task_params.convert2comfy(workflow) {
        Ok(updated_workflow) => println!("{}", updated_workflow),
        Err(e) => eprintln!("Error: {}", e),
    }
}