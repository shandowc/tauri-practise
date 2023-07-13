export interface Setting {
    version: number;
    annotations: AnnotationConfig[];
}

export interface AnnotationConfig {
    inspoint: String,
    key: String,
    value_path: String,
}