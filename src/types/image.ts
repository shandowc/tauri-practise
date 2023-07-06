
export interface FrameInfo {
  timestamp: number;
  image_data: string;
  targets: Target[];
  jsons: Record<string, string[]>;
}

export interface Point {
  x: number;
  y: number;
}

export interface Rect {
  left: number;
  top: number;
  width: number;
  height: number;
}

export interface Target {
  track_id: number;
  label: number;
  roi: Rect;
  track_selected: boolean;
  selected: boolean;
  annotations: string[];
}

export interface Setting {
  annotations: AnnotationConfig[];
}

export interface AnnotationConfig {
  inspoint: String,
  key: String,
  value_path: String,
}
