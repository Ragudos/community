export function isObject(obj: unknown): obj is Object {
    return obj !== null && typeof obj === 'object';
}
export function isString(obj: unknown): obj is string {
    return typeof obj === "string" ;
}
export function isNumber(obj: unknown): obj is number {
    return typeof obj === "number" ;
}
export function isBoolean(obj: unknown): obj is boolean {
    return typeof obj === "boolean" ;
}
export function isArray(obj: unknown): obj is unknown[] {
    return Array.isArray(obj) || obj instanceof Array;
}