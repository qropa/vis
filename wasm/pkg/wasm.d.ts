/* tslint:disable */
/* eslint-disable */
/**
* @param {string} f
* @param {number} t
* @param {number} seed
* @returns {Res}
*/
export function vis(f: string, t: number, seed: number): Res;
/**
* @param {string} f
* @param {number} seed
* @returns {number}
*/
export function get_max_turn(f: string, seed: number): number;
/**
*/
export class Datum {
  free(): void;
/**
*/
  name: string;
/**
*/
  value: string;
}
/**
*/
export class Res {
  free(): void;
/**
*/
  data: (Datum)[];
/**
*/
  error: string;
/**
*/
  score: number;
/**
*/
  svg: string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_datum_free: (a: number, b: number) => void;
  readonly __wbg_get_datum_name: (a: number, b: number) => void;
  readonly __wbg_set_datum_name: (a: number, b: number, c: number) => void;
  readonly __wbg_get_datum_value: (a: number, b: number) => void;
  readonly __wbg_set_datum_value: (a: number, b: number, c: number) => void;
  readonly __wbg_res_free: (a: number, b: number) => void;
  readonly __wbg_get_res_score: (a: number) => number;
  readonly __wbg_set_res_score: (a: number, b: number) => void;
  readonly __wbg_get_res_data: (a: number, b: number) => void;
  readonly __wbg_set_res_data: (a: number, b: number, c: number) => void;
  readonly vis: (a: number, b: number, c: number, d: number) => number;
  readonly get_max_turn: (a: number, b: number, c: number) => number;
  readonly __wbg_set_res_error: (a: number, b: number, c: number) => void;
  readonly __wbg_set_res_svg: (a: number, b: number, c: number) => void;
  readonly __wbg_get_res_error: (a: number, b: number) => void;
  readonly __wbg_get_res_svg: (a: number, b: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
