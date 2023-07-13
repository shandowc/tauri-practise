import type { Setting } from '../types/settings';
import { invoke } from "@tauri-apps/api/tauri";

const SETTING_KEY = "settings";
const CURRNT_VERSION = 1;
const DEFAULT_SETTING = {
    version: CURRNT_VERSION,
    annotations: [
        {
            inspoint: "flock:detect_module",
            key: "detect_confidence",
            value_path: "$.confidence",
        },
        {
            inspoint: "flock:select_module",
            key: "select_status",
            value_path: "$.status",
        },
        {
            inspoint: "flock:select_module",
            key: "select_quality",
            value_path: "$.quality",
        },
        {
            inspoint: "flock:face_quality_module",
            key: "quality_quality",
            value_path: "$.quality",
        },
        {
            inspoint: "flock:face_quality_module",
            key: "quality_integrate_quality",
            value_path: "$.integrate_quality",
        }
    ]
};

export function getConfig(): Setting {
    let s = localStorage.getItem(SETTING_KEY);
    let setting: Setting;
    if (!s) {
        setting = DEFAULT_SETTING
    } else {
        setting = JSON.parse(s);
        if (setting.version != CURRNT_VERSION) {
            setting = DEFAULT_SETTING;
        } 
    }
    return setting;
}

export async function saveConfig(s: Setting) {
    await invoke("set_config", {config: s});
    const str = JSON.stringify(s);
    localStorage.setItem(SETTING_KEY, str);
}

export function getLastInspectDir(): string {
    let s = localStorage.getItem("last_inspect_dir");
    if (!s) {
        return '';
    }
    return s;
}

export function setLastInspectDir(s: string) {
    localStorage.setItem("last_inspect_dir", s);
}